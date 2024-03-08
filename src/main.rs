use tokio::runtime::Runtime;
use reqwest;
use chrono::{DateTime, Utc};

struct Auction<'a> {
    current_price: usize,
    time_left: &'a str,
    no_of_bids: usize,
    first_offer: usize,
    auction_started: DateTime<Utc>,
    auction_ends: DateTime<Utc>,
    seller: &'a str,
    pickup_location: &'a str,
    payment: &'a str,
    shipping: &'a str,
    description: &'a str,
    img_urls: Vec<&'a str>,
}

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

