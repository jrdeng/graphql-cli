use std::collections::HashMap;

pub fn query(url: &str, token: &str, query_str: &str) -> Result<String, String> {
    // query_str is in GraphQL format, need to translate to json
    let body = query_to_json(query_str);

    let client = reqwest::blocking::Client::new();
    let res = client
        .post(url.to_owned())
        .header("User-Agent", "graphquery")
        .header("Authorization", format!("Bearer {}", token.to_owned()))
        .header("Content-Type", "application/json")
        .body(body)
        .send();

    if let Ok(resp) = res {
        if let Ok(text) = resp.text() {
            Ok(text)
        } else {
            Err("Failed to send request".to_owned())
        }
    } else {
        Err("Failed to send request".to_owned())
    }
}

pub fn query_with_args(
    url: &str,
    token: &str,
    query_str: &str,
    args: &HashMap<String, String>,
) -> Result<String, String> {
    match build_query_str(query_str, args) {
        Ok(new_query_str) => query(url, token, &new_query_str),
        Err(err) => Err(err),
    }
}

pub fn query_file(url: &str, token: &str, file: &str) -> Result<String, String> {
    match std::fs::read_to_string(file.to_owned()) {
        Ok(query_str) => query(url, token, &query_str),
        Err(err) => {
            log::error!(
                "Failed to read query string from file({}). err: {}",
                file.to_owned(),
                err
            );
            Err(err.to_string())
        }
    }
}

pub fn query_file_with_args(
    url: &str,
    token: &str,
    file: &str,
    args: &HashMap<String, String>,
) -> Result<String, String> {
    match std::fs::read_to_string(file.to_owned()) {
        Ok(query_str) => match build_query_str(&query_str, args) {
            Ok(new_query_str) => query(url, token, &new_query_str),
            Err(err) => Err(err),
        },
        Err(err) => {
            log::error!(
                "Failed to read query string from file({}). err: {}",
                file.to_owned(),
                err
            );
            Err(err.to_string())
        }
    }
}

fn query_to_json(query: &str) -> String {
    format!(
        "{{\"query\": \"{}\"}}",
        query
            .replace("\n", "")
            .replace("\r", "")
            .replace("\"", "\\\"")
    )
}

fn build_query_str(query_str: &str, args: &HashMap<String, String>) -> Result<String, String> {
    let mut new_query_str = query_str.to_owned();
    for (k, v) in args {
        let key_to_be_replaced = format!("$x:{k}");
        if new_query_str.contains(&key_to_be_replaced) {
            new_query_str = new_query_str.replace(&key_to_be_replaced, v);
        } else {
            log::error!("Extra argument key doesn't exist: {k}");
            return Err("KEY_NOT_FOUND".to_owned());
        }
    }
    Ok(new_query_str)
}
