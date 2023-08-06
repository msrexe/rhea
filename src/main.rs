mod client;
mod cli;

use std::error::Error;
use clap::Parser;
use cli::AppArgs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = AppArgs::parse();

    let mut client = client::Client::new();

    let req = client::Request::new(
        client::Method::from_str(args.method.as_str())?,
        args.url,
        args.headers,
    ).body(args.body);

    let start = std::time::Instant::now();

    let result = client.start(req, args.threads, args.req_count).await;

    let duration = start.elapsed();

    println!("\n\nDuration: {:?}\n\nResponse stats\n{}", duration, result);

    Ok(()) 
}  
