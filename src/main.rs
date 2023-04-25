use std::{collections::HashMap, fs, process};

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

    // handle extra arguments
    // "$x:key" in query json will be replaced by the "value" from extra argument: -x key=value
    let mut extra_args: Option<HashMap<String, String>> = None;
    if args.extras != None {
        let mut extra_args_hash = HashMap::new();
        for extra in args.extras.unwrap() {
            let kv: Vec<&str> = extra.split("=").collect();
            if kv.len() != 2 {
                error!("Wrong format in extra argument: {}", extra);
                process::exit(-2);
            }
            extra_args_hash.insert(kv[0].to_owned(), kv[1].to_owned());
        }
        debug!("extras: {:?}", extra_args_hash);
        extra_args = Some(extra_args_hash);
    }

    let resp: Result<String, String>;
    if let Some(query_str) = args.query {
        if extra_args != None {
            resp = graphquery_lib::query_with_args(
                &args.url,
                &args.token,
                &query_str,
                &extra_args.unwrap(),
            );
        } else {
            resp = graphquery_lib::query(&args.url, &args.token, &query_str);
        }
    } else {
        if extra_args != None {
            resp = graphquery_lib::query_file_with_args(
                &args.url,
                &args.token,
                &args.file.unwrap(),
                &extra_args.unwrap(),
            );
        } else {
            resp = graphquery_lib::query_file(&args.url, &args.token, &args.file.unwrap());
        }
    }

    if let Ok(resp_text) = resp {
        if args.output == None {
            println!("{}", resp_text);
        } else {
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
    } else {
        error!("Failed to send query");
        process::exit(-3);
    }

    Ok(())
}
