use clap::Parser;

mod error;
use error::MyToolError;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    url: String,

    #[arg(short = 'X', long, verbatim_doc_comment)]
    method: Option<String>,

    #[arg(short, long)]
    data: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), MyToolError> {
    let args = Args::parse();
    let method = match &args.method {
        Some(m) => m.parse().map_err(|_| MyToolError::InvalidMethod(m.clone()))?,

        None => {
            if args.data.is_some() {
            reqwest::Method::POST
            } else {
                reqwest::Method::GET
            }
        }
    };


    let _url_check = reqwest::Url::parse(&args.url).map_err(|_| MyToolError::InvalidUrl(args.url.clone()))?;

    let client = reqwest::Client::new();

    let mut request_builder = client.request(method, &args.url);

    if let Some(data) =  &args.data {
        request_builder = request_builder
        .header("Content-Type", "application/json")
        .body(data.clone());
    }

    let response = request_builder.send().await?;
    println!("Статус: {}", response.status());
    println!("Заголовки:");
    for (name, value) in response.headers() {
        println!("  {}: {:?}", name, value);
    }

    // УМНОЕ ЧТЕНИЕ ТЕЛА ОТВЕТА
    let is_json = response.headers()
        .get("content-type")
        .map(|ct| ct.to_str().unwrap_or(""))
        .unwrap_or("")
        .contains("json");

    let body = response.text().await?;

    // УМНЫЙ ВЫВОД
    if is_json {
        match serde_json::from_str::<serde_json::Value>(&body) {
            Ok(pretty_json) => {
                println!("Тело ответа (JSON):\n{}", serde_json::to_string_pretty(&pretty_json)?);
            }
            Err(_) => {
                println!("Сервер сказал, что это JSON, но он битый:");
                println!("{}", body);
            }
        }
    } else {
        println!("Тело ответа:\n{}", body);
    }

    Ok(())
}