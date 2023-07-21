use reqwest::Error;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct Summoner {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "puuid")]
    puuid: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "profileIconId")]
    profile_icon_id: i32,
    #[serde(rename = "revisionDate")]
    revision_date: i64,
    #[serde(rename = "summonerLevel")]
    summoner_level: i32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key = std::env::var("RIOT_API_KEY").expect("RIOT_API_KEY is not set");
    let summoner_name = "Harlley Davidson";

    let summoner_url = format!(
        "https://br1.api.riotgames.com/lol/summoner/v4/summoners/by-name/{}?api_key={}",
        summoner_name, api_key
    );

    let response = reqwest::get(&summoner_url).await?;
    let summoner: Summoner = response.json().await?;

    println!("{:#?}", summoner);

    Ok(())
}
