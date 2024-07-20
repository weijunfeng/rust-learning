use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::ops::Index;

use http::header;
use reqwest::{Client, Error, Url};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

/// 仓库tag信息
#[derive(Debug, Serialize, Deserialize)]
struct RepoTag {
    /// tag 名
    name: String,
}

/// 仓库发布信息
#[derive(Debug, Serialize, Deserialize)]
struct RepoRelease {
    /// 发布名
    name: String,
    /// tag 名
    tag_name: String,
    /// 发布内容
    body: String,
    /// 发布日期
    published_at: String,
    /// 发布分支
    target_commitish: String,
    /// 是否是草稿
    draft: bool,
    /// 是否是预发布
    prerelease: bool,
}

/// 仓库信息
#[derive(Debug)]
struct Repo {
    /// 仓库owner
    owner: String,
    /// 仓库名
    name: String,
    /// 仓库地址
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut file = match File::create("repo_versions.txt") {
        Err(why) => panic!("couldn't create file: {}", why),
        Ok(file) => file,
    };

    let client = client_builder().build()?;
    let repos = parse_repo();
    for ref repo in repos {
        let latest_release = get_latest_release(&client, repo).await;
        let release_tag = if let Ok(ref release) = latest_release {
            &(release.tag_name)
        } else {
            ""
        };
        let all_tag = get_all_tag(&client, repo).await;
        let tag = filter_tag(all_tag, release_tag);
        let result = format!("{repo:?}:\n{:#?}\n{:?}\n\n\n", latest_release, tag);
        file.write_all(result.as_bytes()).expect("couldn't write to file");
    }
    Ok(())
}

fn filter_tag(all_tag: Result<Vec<RepoTag>, String>, target_tag_name: &str) -> Result<Vec<RepoTag>, String> {
    let all_tag = all_tag?;
    let index = all_tag.iter().position(|x| x.name == target_tag_name);
    let min_index = if let Some(index) = index {
        index + 1
    } else {
        5
    };
    return Ok(all_tag.into_iter().take(min_index).collect());
}

async fn get_latest_release(client: &Client, repo: &Repo) -> Result<RepoRelease, String> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/releases/latest",
        owner = repo.owner,
        repo = repo.name
    );
    request(client, &request_url).await
}

async fn get_all_tag(client: &Client, repo: &Repo) -> Result<Vec<RepoTag>, String> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/tags",
        owner = repo.owner,
        repo = repo.name
    );
    return request(client, &request_url).await;
}

async fn request<T: DeserializeOwned>(client: &Client, request_url: &String) -> Result<T, String> {
    let response = client.get(request_url).send().await;
    return match response {
        Ok(response) => {
            if response.status().is_success() {
                response.json().await.map_err(|e| format!("{}", e))
            } else {
                let text = response.text().await;
                text.map_err(|e| format!("{}", e))
                    .and_then(|text| Err(format!("response {text}, for url ({request_url})", )))
            }
        }
        Err(e) => {
            Err(format!("{}", e))
        }
    };
}

fn client_builder() -> reqwest::ClientBuilder {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "User-Agent",
        header::HeaderValue::from_static("89113789@qq.com"),
    );
    headers.insert(
        "Authorization",
        header::HeaderValue::from_static("token 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"),
    );
    headers.insert(
        "Accept",
        header::HeaderValue::from_static("application/vnd.github+json"),
    );
    return Client::builder().default_headers(headers);
}

fn parse_to_repo(line: Option<&String>) -> Option<Repo> {
    let mut line = line?.trim();
    line = if line.starts_with("//") {
        &line[2..].trim()
    } else {
        ""
    };
    let url = Url::parse(line).ok()?;
    match url.host_str() {
        Some(host) => {
            if host != "github.com" {
                return None;
            }
        }
        None => {
            return None;
        }
    }
    // println!("url:{url:?}");
    let mut path_segments = url.path_segments()?;
    let owner = path_segments.next()?;
    let mut name = path_segments.next()?;
    name = if name.ends_with(".git") {
        &name[0..(name.len() - 4)]
    } else {
        name
    };
    if name.is_empty() {
        return None;
    }
    return Some(Repo {
        owner: owner.to_owned(),
        name: name.to_owned(),
        url: format!("{url}"),
    });
}

fn parse_repo() -> Vec<Repo> {
    let file = File::open("Versions.kt");
    if let Ok(file) = file {
        return BufReader::new(file).lines()
            .filter_map(|line| parse_to_repo(line.ok().as_ref()))
            .collect();
    }
    return vec![];
}
