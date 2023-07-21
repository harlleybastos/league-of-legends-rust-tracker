use serde_derive::Deserialize;
use reqwest::Error;

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
    
    println!("Enter a summoner name: ");

    let mut summoner_name = String::new();
    std::io::stdin().read_line(&mut summoner_name)
        .expect("Failed to read line");

    summoner_name = summoner_name.trim().to_string();

    let summoner_url = format!(
        "https://br1.api.riotgames.com/lol/summoner/v4/summoners/by-name/{}?api_key={}",
        summoner_name, api_key
    );

    let response = reqwest::get(&summoner_url).await?;
    let summoner: Summoner = response.json().await?;

    println!("Summoner {} has ID: {} with Profile Icon ID: {} and Summoner Level: {}", summoner.name, summoner.id, summoner.profile_icon_id, summoner.summoner_level);

    Ok(())
}
