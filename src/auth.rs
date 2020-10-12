use serde::{Deserialize, Serialize};
use serde_json::json;

pub struct Authentication {
    uuid: String,
    name: String,
    access_token: String,
}

impl Authentication {
    pub fn uuid(&self) -> &str {
        &self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn access_token(&self) -> &str {
        &self.access_token
    }
}

pub fn authenticate(username: &str, password: &str) -> anyhow::Result<Authentication> {
    let client = reqwest::blocking::Client::new();
    let response: Response = client
        .post("https://authserver.mojang.com/authenticate")
        .json(&json!({
            "agent": {
                "name": "Minecraft",
                "version": 1
            },
            "username": username,
            "password": password
        }))
        .send()?
        .json()?;

    Ok(Authentication {
        uuid: response.selected_profile.id,
        name: response.selected_profile.name,
        access_token: response.access_token,
    })
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Response {
    access_token: String,
    selected_profile: Profile,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Profile {
    id: String,
    name: String,
}
