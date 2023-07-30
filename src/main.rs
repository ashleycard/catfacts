use clap::Parser;

#[derive(Parser)]
struct Cli {
    amount: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let n = &args.amount;
    for _i in 1..=n.parse().unwrap() {
        let resp = reqwest::get("https://meowfacts.herokuapp.com")
            .await?
            .json::<serde_json::Value>()
            .await?;

        println!("{}", resp["data"]);
    }
    Ok(())
}