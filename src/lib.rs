use airtable_flows::create_record;
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use schedule_flows::schedule_cron_job;
use serde::Deserialize;
use serde_json::Value;
use slack_flows::send_message_to_channel;
use std::env;
use store_flows::{global_get, global_set};

#[no_mangle]
pub fn run() {
    schedule_cron_job(
        String::from("33 * * * *"),
        String::from("cron_job_evoked"),
        callback,
    );
}

fn callback(_body: Vec<u8>) {
    dotenv().ok();

    let github_token: String =
        env::var("github_token").unwrap_or("GitHub token not found".to_string());
    let account: &str = "jaykchen";
    let base_id: &str = "apptJFYvsGrrywvWh";
    let table_name: &str = "users";

    let search_key_word = "GitHub WASMEDGE";

    let bearer_token = format!("bearer {}", github_token);

    let mut writer = Vec::new();

    let query_params: Value = serde_json::json!({
        "q": search_key_word,
        "sort": "created",
        "order": "desc"
    });

    let query_string = serde_urlencoded::to_string(&query_params).unwrap();
    let url_str = format!("https://api.github.com/search/issues?{}", query_string);
    let url = Uri::try_from(url_str.as_str()).unwrap();

    match Request::new(&url)
        .method(Method::GET)
        .header("Authorization", &bearer_token)
        .header("User-Agent", "flows-network connector")
        .header("Content-Type", "application/vnd.github.v3+json")
        .send(&mut writer)
    {
        Ok(res) => {
            if !res.status_code().is_success() {
                send_message_to_channel("ik8", "ch_err", res.status_code().to_string());
            }
            let response: Result<SearchResult, _> = serde_json::from_slice(&writer);
            match response {
                Err(_e) => {
                    send_message_to_channel("ik8", "ch_err", _e.to_string());

                }

                Ok(search_result) => {
                    let time_entries_last_saved: Option<DateTime<Utc>> =
                        match global_get("time_entries_last_saved")
                            .unwrap()
                            .to_string()
                            .parse()
                        {
                            Ok(t) => Some(t),
                            Err(_) => None,
                        };
                    for item in search_result.items {
                        let name = item.user.login;
                        let title = item.title;
                        send_message_to_channel("ik8", "ch_in", title.to_string());
                        let html_url = item.html_url;
                        let time = item.created_at;

                        if time_entries_last_saved.is_none()
                            || time_entries_last_saved.is_some()
                                && time > time_entries_last_saved.unwrap()
                        {
                            let text = format!(
                                "{name} mentioned WASMEDGE in issue: {title}  @{html_url}\n{time}"
                            );
                            send_message_to_channel("ik8", "ch_mid", text);

                            let data = serde_json::json!({
                                "Name": name,
                                "Repo": html_url,
                                "Created": time,
                            });
                            create_record(account, base_id, table_name, data.clone());
                            let time_serialized: Value =
                                serde_json::to_value(&time).expect("Failed to serialize date-time");
                            global_set("time_entries_last_saved", time_serialized);
                            send_message_to_channel("ik8", "ch_out", data.to_string());
                        }
                        send_message_to_channel("ik8", "ch_mid", time.to_string());
                    }
                }
            }
        }
        Err(_) => {}
    }
}

#[derive(Debug, Deserialize)]
struct SearchResult {
    items: Vec<Issue>,
}

#[derive(Debug, Deserialize)]
struct Issue {
    html_url: String,
    title: String,
    user: User,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct User {
    login: String,
}
