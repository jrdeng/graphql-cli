use std::{fs, process};

use clap::Parser;
use log::{debug, error};

#[derive(Parser, Debug)]
struct Args {
    /// URL of GraphQL API. for example: https://<your-server.com>/api/graphql
    #[arg(short, long, value_name = "API_URL")]
    url: String,

    /// Token to access the API
    #[arg(short, long, value_name = "TOKEN")]
    token: String,

    /// File that contains the query string. one of '-f' and '-q' must be set
    #[arg(short, long, value_name = "QUERY_FILE")]
    file: Option<String>,

    /// Query string. '-f' will be omitted if this is set
    #[arg(short, long, value_name = "API_URL")]
    query: Option<String>,

    /// Output file to store the query result. The result is echo to std out by default
    #[arg(short, long, value_name = "OUTPUT_FILE")]
    output: Option<String>,

    /// Extra arguments(multiple) to be used in graphql query.
    #[arg(short = 'x', long = "extra")]
    extras: Option<Vec<String>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = Args::parse();
    if args.file == None && args.query == None {
        error!("QUERY_FILE or QUERY_STRING must be set! use -h to see usage.");
        process::exit(-1);
    }

    debug!("{:#?}", args);

    let client = reqwest::blocking::Client::new();
    let url = args.url;
    let auth = format!("Bearer {}", args.token);
    let mut body = if args.query != None {
        query_to_json_str(&args.query.unwrap())
    } else {
        String::from("")
    };
    if body.is_empty() {
        match fs::read_to_string(args.file.clone().unwrap()) {
            Ok(data) => {
                body = query_to_json_str(&data);
            }
            Err(err) => {
                error!("Failed to read query string from file. err: {}", err);
                process::exit(-2);
            }
        }
    }
    debug!("body: {}", body);

    // handle extra arguments
    // "$x:key" in query json will be replaced by the "value" from extra argument: -x key=value
    if args.extras != None {
        for extra in args.extras.unwrap() {
            let kv: Vec<&str> = extra.split("=").collect();
            if kv.len() != 2 {
                error!("Wrong format in extra argument: {}", extra);
                process::exit(-3);
            }
            let key_to_be_replaced = format!("$x:{}", kv[0]);
            if body.contains(&key_to_be_replaced) {
                body = body.replace(&key_to_be_replaced, kv[1]);
            } else {
                error!("Extra argument key doesn't exist: {}", kv[0]);
                process::exit(-3);
            }
        }
        debug!("body after replace extras: {}", body);
    }

    let resp = client
        .post(url)
        .header("User-Agent", "graphquery")
        .header("Authorization", auth)
        .header("Content-Type", "application/json")
        .body(body)
        .send()?;

    if args.output == None {
        println!("{}", resp.text().unwrap());
    } else {
        let resp_text = resp.text().unwrap();
        match fs::write(args.output.clone().unwrap(), &resp_text) {
            Ok(_) => {}
            Err(err) => {
                error!(
                    "Failed to write file {}: {}",
                    args.output.clone().unwrap(),
                    err
                );
                println!("Query result:");
                println!("{}", &resp_text);
            }
        }
    }

    Ok(())
}

fn query_to_json_str(query: &String) -> String {
    format!(
        "{{\"query\": \"{}\"}}",
        query
            .replace("\n", "")
            .replace("\r", "")
            .replace("\"", "\\\"")
    )
}
