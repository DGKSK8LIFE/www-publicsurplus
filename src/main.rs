use tokio::runtime::Runtime;
use reqwest;

#[tokio::main]
async fn main() {
 public_surplus_auction("3474529").await;
}

async fn public_surplus_auction(id: &str) -> Result<(), reqwest::Error> {
    let url = format!("https://www.publicsurplus.com/sms/auction/view?auc={}", id);

    let res = reqwest::get(&url).await?;

    let body = res.text().await?;

    println!("Body: \n{}", body);

    Ok(())
}

