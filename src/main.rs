extern crate serde;

use std::env;
use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct Msg {
    c: f32,
    h: f32,
    l: f32,
    o: f32,
    pc: f32,
    t: f32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let syms: Vec<String> = env::args().collect();
//    println!("{}", syms[1]);
    let url = format!("https://finnhub.io/api/v1/quote?symbol={}&token=bs2fitvrh5rc90r581q0", syms[1]);
    let resp = reqwest::get(&url).await?;
    println!("Status: {}", resp.status());
//    let data: String = resp.text().await?;
//    println!("{}", data);
    let items: Msg = resp.json::<Msg>().await?;
    println!("{:?}", items);
    Ok(())
}
