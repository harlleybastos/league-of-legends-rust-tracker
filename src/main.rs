use serde_derive::Deserialize;
use reqwest::Error;
use std::env;

#[derive(Deserialize, Debug)]
struct Summoner {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "profileIconId")]
    profile_icon_id: i32,
    #[serde(rename = "summonerLevel")]
    summoner_level: i32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key = std::env::var("RIOT_API_KEY").expect("RIOT_API_KEY is not set");
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a summoner name as the first argument!");
        std::process::exit(1);
    }

    let summoner_name = &args[1];

    let summoner_url = format!(
        "https://br1.api.riotgames.com/lol/summoner/v4/summoners/by-name/{}?api_key={}",
        summoner_name, api_key
    );

    let response = reqwest::get(&summoner_url).await?;
    let summoner: Summoner = response.json().await?;

    println!("Summoner {} has ID: {} with Profile Icon ID: {} and Summoner Level: {}", summoner.name, summoner.id, summoner.profile_icon_id, summoner.summoner_level);

    Ok(())
}
