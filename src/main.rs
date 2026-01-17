mod api;
mod http;
mod model;

use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::{Result, anyhow};
use dialoguer::{Input, Select};
use futures::{StreamExt, stream};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::Client;
use tokio::{fs, io::AsyncWriteExt};

use model::{
    result::ResultData,
    search::SearchData,
};

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}

async fn run() -> Result<()> {
    let client = api::Api::new();

    // 交互式搜索与分页选择
    let (search_result, selection) = match search_flow(&client).await? {
        Some(result) => result,
        None => return Ok(()),
    };

    let book_id = &search_result[selection].book_id;
    let episodes = client.get_with_book_id(book_id.as_str()).await?;

    // 下载流程：并发池 + 进度条
    let title = sanitize_filename(&search_result[selection].title);
    download_series(&client, title, episodes, 3).await
}

enum SearchAction {
    Selected(usize),
    NextPage,
    PrevPage,
    NewSearch,
    Quit,
}

async fn search_flow(client: &api::Api) -> Result<Option<(Vec<SearchData>, usize)>> {
    loop {
        let keyword = prompt_keyword()?;
        let mut page: i8 = 1;
        loop {
            let search_result = client.search(keyword.as_str(), page).await.unwrap_or_default();
            match select_search_page(page, &search_result)? {
                SearchAction::Selected(index) => return Ok(Some((search_result, index))),
                SearchAction::NextPage => {
                    if page < i8::MAX {
                        page += 1;
                    }
                }
                SearchAction::PrevPage => {
                    if page > 1 {
                        page -= 1;
                    }
                }
                SearchAction::NewSearch => break,
                SearchAction::Quit => return Ok(None),
            }
        }
    }
}

fn prompt_keyword() -> Result<String> {
    let mut keyword: String = Input::new()
        .with_prompt("请输入要查询的短句名称")
        .interact_text()?;
    while keyword.trim().is_empty() {
        keyword = Input::new()
            .with_prompt("请输入要查询的短句名称")
            .interact_text()?;
    }
    Ok(keyword)
}

fn select_search_page(page: i8, items: &[SearchData]) -> Result<SearchAction> {
    let mut options: Vec<String> = items.iter().map(|item| item.title.clone()).collect();
    options.push("下一页".to_string());
    options.push("上一页".to_string());
    options.push("重新搜索".to_string());
    options.push("退出".to_string());

    let prompt = format!("搜索结果 (第 {page} 页)");
    let selection = Select::new()
        .with_prompt(prompt)
        .items(&options)
        .default(0)
        .interact()?;

    if selection < items.len() {
        return Ok(SearchAction::Selected(selection));
    }

    match options[selection].as_str() {
        "下一页" => Ok(SearchAction::NextPage),
        "上一页" => Ok(SearchAction::PrevPage),
        "重新搜索" => Ok(SearchAction::NewSearch),
        "退出" => Ok(SearchAction::Quit),
        _ => Err(anyhow!("未知操作选项")),
    }
}

async fn download_series(
    api: &api::Api,
    title: String,
    episodes: Vec<ResultData>,
    concurrency: usize,
) -> Result<()> {
    let http_client = Client::new();
    let start = Instant::now();

    let mp = MultiProgress::new();
    let _ = mp.println(format!("开始下载：{}", title));

    // 创建目录并初始化进度条
    fs::create_dir_all(&title).await?;
    let (progress_style, spinner_style) = build_progress_styles()?;

    let mut progress_bars = Vec::with_capacity(episodes.len());
    for item in &episodes {
        let pb = mp.add(ProgressBar::new_spinner());
        pb.set_style(spinner_style.clone());
        pb.set_message(item.title.clone());
        pb.enable_steady_tick(Duration::from_millis(120));
        progress_bars.push(pb);
    }
    let progress_bars = Arc::new(progress_bars);

    // 并发下载，完成后按顺序输出
    let results = stream::iter(episodes.into_iter().enumerate().map(|(index, item)| {
        let api = api.clone();
        let http_client = http_client.clone();
        let progress_style = progress_style.clone();
        let spinner_style = spinner_style.clone();
        let progress_bars = Arc::clone(&progress_bars);
        let title = title.clone();
        async move {
            let detail = api.get_with_video_id(item.video_id.as_str()).await?;
            let resp = http_client.get(detail.url).send().await?;

            let total_size = resp.content_length();
            let pb = progress_bars[index].clone();
            if let Some(size) = total_size {
                if size > 0 {
                    pb.disable_steady_tick();
                    pb.set_length(size);
                    pb.set_style(progress_style);
                } else {
                    pb.set_style(spinner_style);
                }
            }

            let filename = format!("{}/{}.mp4", title, sanitize_filename(&item.title));
            let mut file = fs::File::create(filename).await?;
            let mut stream = resp.bytes_stream();

            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                file.write_all(&chunk).await?;
                pb.inc(chunk.len() as u64);
            }

            pb.finish_with_message(format!("完成: {}", item.title));
            Ok::<(usize, String), anyhow::Error>((index, item.title))
        }
    }))
    .buffer_unordered(concurrency)
    .collect::<Vec<_>>()
    .await;

    let mut ordered = Vec::with_capacity(results.len());
    for result in results {
        match result {
            Ok(entry) => ordered.push(entry),
            Err(err) => {
                let _ = mp.println(format!("下载失败: {err:#}"));
            }
        }
    }
    ordered.sort_by_key(|(index, _)| *index);
    for (_index, name) in ordered {
        let _ = mp.println(format!("✅{}", name));
    }

    let _ = mp.println(format!("全部下载完成, 耗时: {:?}", start.elapsed()));
    Ok(())
}

fn build_progress_styles() -> Result<(ProgressStyle, ProgressStyle)> {
    let progress_style = ProgressStyle::default_bar()
        .template("{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
        .progress_chars("=>-");
    let spinner_style = ProgressStyle::default_spinner()
        .template("{spinner} {msg} {bytes} ({elapsed})")?;
    Ok((progress_style, spinner_style))
}

fn sanitize_filename(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for ch in name.chars() {
        match ch {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => out.push('_'),
            _ => out.push(ch),
        }
    }

    if out.is_empty() {
        "_".to_string()
    } else {
        out
    }
}
