mod task;
mod view;

use crossterm::style::Color;
use reqwest::{header, Url};
use serde_json::{Map, Value};
use std::{collections::HashMap, process::exit, time::Instant};
use task::Task;
use yup_oauth2::{AccessToken, InstalledFlowAuthenticator, InstalledFlowReturnMethod};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_secret = yup_oauth2::read_application_secret("config.json")
        .await
        .expect("config");

    let auth =
        InstalledFlowAuthenticator::builder(app_secret, InstalledFlowReturnMethod::HTTPRedirect)
            .persist_tokens_to_disk("tokencache.json")
            .build()
            .await?;
    let scopes = &["https://www.googleapis.com/auth/tasks"];
    let mut option_token: Option<AccessToken> = None;
    match auth.token(scopes).await {
        Err(e) => println!("error: {:?}", e),
        Ok(t) => {
            option_token = Some(t);
        }
    }
    if option_token.is_none() {
        println!("No token");
        exit(1);
    }
    let token = option_token.unwrap();
    let token_str = token.token().unwrap().to_string();

    let mut headers = header::HeaderMap::new();
    let mut auth_value = header::HeaderValue::from_str(&format!("Bearer {}", token_str))?;
    auth_value.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth_value);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let all_tasklist = client
        .get("https://tasks.googleapis.com/tasks/v1/users/@me/lists")
        .send()
        .await?;

    let response = all_tasklist.json::<Value>().await?;

    let first_task_id = response
        .get("items")
        .unwrap()
        .get(0)
        .unwrap()
        .get("id")
        .unwrap()
        .as_str()
        .unwrap();

    // let now = Instant::now();
    let tasks = client
        .get(format!(
            "https://tasks.googleapis.com/tasks/v1/lists/{}/tasks?showHidden=true",
            first_task_id
        ))
        .send()
        .await?;
    // println!("fetched in {}ms", now.elapsed().as_millis());
    let response = tasks
        .json::<Value>()
        .await?
        .get("items")
        .unwrap()
        .to_owned();

    let all_task = serde_json::from_value::<Vec<Task>>(response)?;
    view::color(Color::Green, " AGENDA\n");
    for val in all_task.iter() {
        if val.title().is_empty() {
            break;
        }
        view::color(Color::DarkGreen, "  ➜ ");
        view::print(format!("{}\n", title_case(val.title())));
    }
    Ok(())
}
fn title_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
