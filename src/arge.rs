use chrono::{DateTime, Local};
use reqwest::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug)]
pub struct Location {
    abbreviation: String,
    pub response: LocationResponse
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationResponse {
    pub cprof: u32,
    pub costc: u32,
    pub vcode: String,
}

impl Location {
    pub async fn new(abbreviation: String) -> Option<Location> {
        return match get(&format!("https://display.necta.at/arge/assets/profiles/{}.json", abbreviation)).await {
            Ok(response) => {
                match response.json::<LocationResponse>().await {
                    Ok(response) => {
                        Some(Location {
                            abbreviation,
                            response
                        })
                    },
                    _ => {
                        println!("Error: Could not parse response");
                        None
                    }
                }
            }
            Err(error) => {
                println!("Error: {}", error);
                None
            }
        };
    }

    pub async fn get_items_by_date(&self, date: DateTime<Local>) -> Option<Vec<Item>> {
        let url = format!("https://main.necta.at/public?action=cprof_menu&profile={}&costc={}&language=1&date={}&vcode={}", &self.response.cprof, &self.response.costc, &date.format("%Y-%m-%d"), &self.response.vcode);

        return match reqwest::Client::new().post(&url).send().await {
            Ok(response) => {
                match response.json::<Vec<Item>>().await {
                    Ok(response) => {
                        Some(response)
                    },
                    _ => {
                        println!("Error: Could not parse response");
                        None
                    }
                }
            }
            Err(error) => {
                println!("Error: {}", error);
                None
            }
        };
    }

    pub async fn get_items(&self) -> Option<Vec<Item>> {
        let date: DateTime<Local> = Local::now();
        self.get_items_by_date(date).await
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    #[serde(rename = "catvar_id")]
    pub catvar_id: String,
    #[serde(rename = "catvar0t_bez")]
    pub title: String,
    #[serde(rename = "pstruc0t_bez")]
    pub designation: Option<String>,
    #[serde(rename = "component_id")]
    pub component_id: Option<String>,
    #[serde(rename = "genmenu_id")]
    pub genmenu_id: String,
    #[serde(rename = "c_allergen_str")]
    pub c_allergen_str: String,
    #[serde(rename = "c_picture_url")]
    pub c_picture_url: String,
    #[serde(rename = "pstruc_menu_main_f")]
    pub pstruc_menu_main_f: String,
    #[serde(rename = "catvar_code")]
    pub code: String,
    #[serde(rename = "component0t_zustext")]
    pub component0t_zustext: Option<String>,
    #[serde(rename = "component0t_bez_extern")]
    pub component0t_bez_extern: Value,
}
