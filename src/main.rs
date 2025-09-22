use clap::Parser;
use reqwest::dns::Resolve;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    url: String,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let response = reqwest::get(&args.url).await?;
    println!("huarim po etoi huine {}", response.status());

    println!("State:");
    for(name, value) in response.headers() {
        println!(" {}: {:?}", name, value);
    }
    let body = response.text().await?;

    println!("\n{}", body);
    Ok(())
}
