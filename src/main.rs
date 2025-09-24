use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let response = reqwest::get(&args.url).await?;

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