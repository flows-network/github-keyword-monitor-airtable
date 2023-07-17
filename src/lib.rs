use airtable_flows::create_record;
use chrono::{DateTime, Duration, Utc};
use dotenv::dotenv;
use flowsnet_platform_sdk::logger;
use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use log;
use schedule_flows::schedule_cron_job;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashSet;
use std::env;
use store_flows::{get, set, Expire, ExpireKind};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    schedule_cron_job(
        String::from("8 8 * * *"),
        String::from("cron_job_evoked"),
        callback,
    )
    .await;
}

async fn callback(_body: Vec<u8>) {
    dotenv().ok();
    logger::init();

    let airtable_token_name = env::var("airtable_token_name").unwrap_or("github".to_string());
    let airtable_base_id = env::var("airtable_base_id").unwrap_or("appNEswczILgUsxML".to_string());
    let airtable_table_name = env::var("airtable_table_name").unwrap_or("mention".to_string());
    let search_key_word = env::var("KEYWORD").unwrap_or("WASMEDGE".to_string());

    let query_params: Value = serde_json::json!({
        "q": search_key_word,
        "sort": "created",
        "order": "desc"
    });

    let query_string = serde_urlencoded::to_string(&query_params).unwrap();
    let url_str = format!("https://api.github.com/search/issues?{}", query_string);
    // further filter with following condition:
    // let url_str = format!("https://api.github.com/search/issues?q={search_key_word}+in:title+state:open+repo:second-state/WasmEdge&sort=created&order=desc");

    let url = Uri::try_from(url_str.as_str()).unwrap();
    let mut writer = Vec::new();

    // search on GitHub is subject to a lower rate limit when done without a token
    match Request::new(&url)
        .method(Method::GET)
        .header("User-Agent", "flows-network connector")
        .header("Content-Type", "application/vnd.github.v3+json")
        .send(&mut writer)
    {
        Ok(res) => {
            if !res.status_code().is_success() {
                log::debug!("Error sending request: {:?}", res.status_code());
            };

            let response: Result<SearchResult, _> = serde_json::from_slice(&writer);
            match response {
                Err(_e) => {
                    log::debug!("Error parsing response: {:?}", _e.to_string());
                }

                Ok(search_result) => {
                    let now = Utc::now();
                    let one_day_earlier = now - Duration::days(1);
                    let one_day_earlier = one_day_earlier.date_naive();

                    for item in search_result.items {
                        let name = item.user.login;
                        // let title = item.title;
                        let html_url = item.html_url;

                        let time = item.created_at;
                        let date;
                        match time.parse::<DateTime<Utc>>() {
                            Ok(t) => date = t.date_naive(),
                            Err(_) => {
                                continue;
                            }
                        }
                        if date > one_day_earlier {
                            // reported issues/pull_requests stored by key `issue_records` as HashSet<String> in store
                            // shall expire after 30 days of its creation
                            let expire = Expire {
                                kind: ExpireKind::Ex,
                                value: 1000 * 3600 * 24 * 30,
                            };

                            let mut records: HashSet<String> = get("issue_records")
                                .and_then(|val| serde_json::from_value(val).ok())
                                .unwrap_or_default();

                            if records.contains(&html_url) {
                                continue;
                            } else {
                                records.insert(html_url.clone());
                                set("issue_records", serde_json::json!(records), Some(expire));
                            }

                            let data = serde_json::json!({
                                "Name": name,
                                "Repo": html_url,
                                "Created": time,
                            });
                            create_record(
                                &airtable_token_name,
                                &airtable_base_id,
                                &airtable_table_name,
                                data.clone(),
                            );
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        Err(_e) => {
            log::debug!("Error getting response from GitHub: {:?}", _e.to_string());
        }
    }
}

#[derive(Debug, Deserialize)]
struct SearchResult {
    // incomplete_results: bool,
    items: Vec<Issue>,
}

#[derive(Debug, Deserialize)]
struct Issue {
    html_url: String,
    title: String,
    user: User,
    created_at: String,
    // body: Option<String>,
}

#[derive(Debug, Deserialize)]
struct User {
    login: String,
}
