use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use ts_rs::TS;

use crate::inits::Stats;

pub async fn get<T>(url: &str, token: &str) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let res = Client::new()
        .get(format!("{}{}", env!("API_URL"), url))
        .bearer_auth(token)
        .send()
        .await
        .map_err(|err| format!("request failed: {:#?}", err))?;

    if res.status().is_success() {
        Ok(res
            .json()
            .await
            .map_err(|err| format!("failed to parse: {:#?}", err))?)
    } else {
        Err(format!("request failed: {:#?}", res.text().await.unwrap()))
    }
}

pub async fn post<T>(url: &str, token: &str, data: T) -> Result<(), String>
where
    T: Serialize,
{
    let res = Client::new()
        .post(format!("{}{}", env!("API_URL"), url))
        .bearer_auth(token)
        .json(&data)
        .send()
        .await
        .map_err(|err| format!("request failed: {:#?}", err))?;

    if !res.status().is_success() {
        Err(format!("request failed: {:#?}", res.text().await.unwrap()))
    } else {
        Ok(())
    }
}

pub async fn get_players() -> Result<Vec<LeaderboardPlayer>, String> {
    let res = Client::new()
        .get(format!("{}{}", env!("API_URL"), "/players"))
        .send()
        .await
        .map_err(|err| format!("request failed: {:#?}", err))?;

    if res.status().is_success() {
        Ok(res
            .json()
            .await
            .map_err(|err| format!("failed to parse: {:#?}", err))?)
    } else {
        Err(format!("request failed: {:#?}", res.text().await.unwrap()))
    }
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct LeaderboardPlayer {
    pub money: u32,
    pub stamina_lvl: u32,
    pub regen_lvl: u32,
    pub auto_lvl: u32,
    pub user: LeaderboardUser,
}

#[derive(Debug, Serialize, Deserialize, TS)]
pub struct LeaderboardUser {
    pub name: String,
    pub tag: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiPlayer {
    pub id: String,
    pub money: u32,
    pub stamina_lvl: u32,
    pub regen_lvl: u32,
    pub auto_lvl: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiUser {
    pub id: String,
    pub name: String,
    pub tag: String,
    pub assigned: bool,
    pub player: ApiPlayer,
    pub stats: Stats,
}
