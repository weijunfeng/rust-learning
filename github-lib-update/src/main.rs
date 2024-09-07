use http::{header, HeaderMap};
use reqwest::{Client, Error, Response, Url};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

mod constant;

/// 仓库tag信息
#[derive(Debug, Serialize, Deserialize)]
struct RepoTag {
    /// tag 名
    name: String,
}

/// 发布内容信息
#[derive(Serialize, Deserialize)]
struct Body(String);

/// 仓库发布信息
#[derive(Debug, Serialize, Deserialize)]
struct RepoRelease {
    /// 发布名
    name: String,
    /// tag 名
    tag_name: String,
    /// 发布内容
    body: Body,
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

/// 重写 Body 默认 debug输出信息，处理支持文本换行，默认的 Debug输出会把换行符当成字符串输出
impl Debug for Body {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 从`all_tag`过滤出`target_tag_name`所在位置的数据，如果`target_tag_name`不在`all_tag`中则返回最多 5 个数据长度
fn filter_tag<'a>(
    all_tag: Result<&'a Vec<RepoTag>, &'a String>,
    target_tag_name: &str,
) -> Result<&'a [RepoTag], &'a String> {
    let all_tag = all_tag?;
    let index = all_tag.iter().position(|x| x.name == target_tag_name);
    if let Some(index) = index {
        let end_index = (index + 10).min(all_tag.len() - 1);
        Ok(&all_tag[index..=end_index])
    } else {
        Ok(&all_tag[0..=4.min(all_tag.len() - 1)])
    }
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
        "https://api.github.com/repos/{owner}/{repo}/tags?per_page=100",
        owner = repo.owner,
        repo = repo.name
    );
    get_tag_by_url(client, &request_url).await
}
// 
// async fn get_tag_by_url(client: &Client, request_url: &str) -> Result<Vec<RepoTag>, String> {
//     let response = client.get(request_url).send().await;
//     match response {
//         Ok(response) => {
//             let next_url = get_next_url(response.headers());
//             let mut result = parse_response(response, request_url).await;
//             if result.is_ok() {
//                 if let Some(url) = next_url {
//                     Box::pin(get_tag_by_url(client, &url)).await
//                         .and_then(|mut f| {
//                             f.append(result.as_mut().unwrap());
//                             Ok(f)
//                         })
//                 } else {
//                     result
//                 }
//             } else {
//                 result
//             }
//         }
//         Err(e) => Err(format!("{}", e)),
//     }
// }
async fn get_tag_by_url(client: &Client, request_url: &str) -> Result<Vec<RepoTag>, String> {
    let response = client.get(request_url).send().await.map_err(|e| format!("{}", e))?;

    let next_url = if !request_url.contains("page=4") {
        get_next_url(response.headers())
    } else {
        None
    };
    let mut result: Vec<RepoTag> = parse_response(response, request_url).await?;
    if let Some(url) = next_url {
        let mut next_tags: Vec<RepoTag> = Box::pin(get_tag_by_url(client, &url)).await?;
        result.append(&mut next_tags);
    }
    Ok(result)
}
// fn get_next_url(header_map: &HeaderMap) -> Option<String> {
//     if let Some(link_header) = header_map.get("Link") {
//         if let Ok(header_value) = link_header.to_str() {
//             for link in header_value.split(",") {
//                 if link.contains("rel=\"next\"") {
//                     let next_url = &link[link.find("<")? + 1..link.find(">")?];
//                     return Some(String::from(next_url));
//                 }
//             }
//         }
//     }
//     None
// }
fn get_next_url(header_map: &HeaderMap) -> Option<String> {
    header_map.get("Link")
        .and_then(|link_header| link_header.to_str().ok())
        .and_then(|header_value| {
            header_value.split(',')
                .find(|link| link.contains("rel=\"next\""))
                .and_then(|link| {
                    let start = link.find('<')? + 1;
                    let end = link.find('>')?;
                    Some(link[start..end].to_string())
                })
        })
}

async fn request<T: DeserializeOwned>(client: &Client, request_url: &String) -> Result<T, String> {
    let response = client.get(request_url).send().await;
    match response {
        Ok(response) => {
            parse_response(response, request_url).await
        }
        Err(e) => Err(format!("{}", e)),
    }
}

async fn parse_response<T: DeserializeOwned>(response: Response, request_url: &str) -> Result<T, String> {
    if response.status().is_success() {
        response.json().await.map_err(|e| format!("{}", e))
    } else {
        let text = response.text().await;
        text.map_err(|e| format!("{}", e))
            .and_then(|text| Err(format!("response {text}, for url ({request_url})")))
    }
}

fn client_builder() -> reqwest::ClientBuilder {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "User-Agent",
        header::HeaderValue::from_static(constant::USER_AGENT),
    );
    headers.insert(
        "Authorization",
        header::HeaderValue::from_str(&format!("token {}", constant::TOKEN)).expect("token fail"),
    );
    headers.insert(
        "Accept",
        header::HeaderValue::from_static("application/vnd.github+json"),
    );
    Client::builder().default_headers(headers)
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
    Some(Repo {
        owner: owner.to_owned(),
        name: name.to_owned(),
        url: format!("{url}"),
    })
}

fn parse_repo() -> Vec<Repo> {
    let file = File::open(constant::VERSION_FILE_NAME);
    if let Ok(file) = file {
        return BufReader::new(file)
            .lines()
            .filter_map(|line| parse_to_repo(line.ok().as_ref()))
            .collect();
    }
    vec![]
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut file = match File::create(constant::REPO_VERSION_FILE_NAME) {
        Err(why) => panic!("couldn't create file: {}", why),
        Ok(file) => file,
    };

    let client = client_builder().build()?;
    let repos = parse_repo();
    let mut repo_identify_set = HashSet::new();
    for (index, repo) in repos.iter().enumerate() {
        let repo_identify = format!("{}:{}", repo.owner, repo.name);
        if !repo_identify_set.insert(repo_identify) {
            continue;
        }
        let latest_release = get_latest_release(&client, repo).await;
        let release_tag = if let Ok(ref release) = latest_release {
            &(release.tag_name)
        } else {
            ""
        };
        let all_tag = get_all_tag(&client, repo).await;
        let tag = filter_tag(all_tag.as_ref(), release_tag);
        let result = format!("{repo:?}:\n{:#?}\n{:?}", latest_release, tag);
        file.write_all(result.as_bytes())
            .expect("couldn't write to file");
        if index == repos.len() - 1 {
            break;
        }
        file.write_all("\n\n\n".as_bytes())
            .expect("write file fail");
    }
    Ok(())
}
