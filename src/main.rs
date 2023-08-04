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
        let mut resp = reqwest::get("https://meowfacts.herokuapp.com")
            .await?
            .json::<serde_json::Value>()
            .await?;

        let b = &resp["data"].take();
        let mut c = format!("{}", b);
        c.remove(0);
        c.remove(0);
        let end = c.len();
        c.remove(end-1);
        c.remove(end-2);
        println!("{}", c);

    }
    Ok(())
}