use reqwest::Error;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct Summoner {
    id: String,
    accountId: String,
    puuid: String,
    name: String,
    profileIconId: i32,
    revisionDate: i64,
    summonerLevel: i32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key = "RGAPI-43081cef-fa19-400b-af8a-48dc7f2b1a58";
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
