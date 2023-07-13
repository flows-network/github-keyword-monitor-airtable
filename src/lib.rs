use airtable_flows::create_record;
use chrono::{DateTime, Duration, NaiveDate, Utc};
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
use slack_flows::send_message_to_channel;
use std::collections::HashSet;
use std::env;
use store_flows::{get, set};
#[no_mangle]
pub fn run() {
    schedule_cron_job(
        String::from("25 * * * *"),
        String::from("cron_job_evoked"),
        callback,
    );
}

fn callback(_body: Vec<u8>) {
    dotenv().ok();
    logger::init();

    let airtable_token_name = env::var("airtable_token_name").unwrap_or("github".to_string());
    let airtable_base_id = env::var("airtable_base_id").unwrap_or("appNEswczILgUsxML".to_string());
    let airtable_table_name = env::var("airtable_table_name").unwrap_or("mention".to_string());

    let search_key_word = "WASMEDGE";
    let query_params: Value = serde_json::json!({
        "q": search_key_word,
        "sort": "created",
        "order": "desc"
    });

    let query_string = serde_urlencoded::to_string(&query_params).unwrap();
    let url_str = format!("https://api.github.com/search/issues?{}", query_string);
    // let url_str = format!("https://api.github.com/search/issues?q={search_key_word}+in:title+state:open+repo:second-state/WasmEdge&sort=created&order=desc");

    let url = Uri::try_from(url_str.as_str()).unwrap();
    let mut writer = Vec::new();

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
                    let one_day_earlier = one_day_earlier.date_naive(); // get the NaiveDate

                    for item in search_result.items {
                        let name = item.user.login;
                        // let title = item.title;
                        let html_url = item.html_url;
                        send_message_to_channel("ik8", "ch_in", html_url.to_string());

                        let time = item.created_at;
                        let datetime: DateTime<Utc> = time.parse().unwrap(); // Parse the date and time string
                        let date: NaiveDate = datetime.date_naive(); // Convert to just date

                        if date > one_day_earlier {
                            match get("issue_records") {
                                Some(records) => {
                                    let records: HashSet<String> =
                                        serde_json::from_value(records).unwrap_or_default();

                                    if records.contains(&html_url) {
                                        continue;
                                    } else {
                                        let mut records = records;
                                        records.insert(html_url.clone());
                                        set("issue_records", serde_json::json!(records), None);
                                    }
                                }

                                None => {
                                    let mut inner = HashSet::<String>::new();
                                    inner.insert(html_url.clone());
                                    set("issue_records", serde_json::json!(inner), None);
                                }
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

                            send_message_to_channel("ik8", "general", data.to_string());
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
