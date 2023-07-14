use serde::Deserialize;
use chrono::Datelike;

#[derive(Deserialize, Debug)]
pub struct Dag {
  pub datum: String,
  pub veckodag: String,
  #[serde(alias = "arbetsfri dag")]
  pub arbetsfri_dag: String,
  #[serde(alias = "röd dag")]
  pub rod_dag: String,
  pub vecka: String,
  #[serde(alias = "dag i vecka")]
  pub dag_i_vecka: String,
  #[serde(default)]
  pub helgdag: String,
  pub namnsdag: Vec<String> 
}

#[derive(Deserialize, Debug)]
pub struct Namnsdag {
  pub cachetid: String,
  pub version: String,
  pub uri: String,
  pub startdatum: String,
  pub slutdatum: String,
  pub dagar: Vec<Dag>
}

fn get_date_string() -> String {
    let current_date = chrono::Utc::now();
    let year = current_date.year();
    let month = current_date.month();
    let day = current_date.day();

    format!("{}/{}/{}", year, month, day)
}

const BASE_URL: &str = "https://sholiday.faboul.se/dagar/v2.1/";

pub async fn get_namnsdag () -> Result<Namnsdag, reqwest_wasm::Error>  {
    let date_url = get_date_string();
    let url = format!("{}{}", BASE_URL, date_url);

    reqwest_wasm::get(&url)
    .await?
    .json::<Namnsdag>()
    .await
}

pub fn get_names(payload: &Namnsdag) -> Vec<String> {
    payload.dagar.iter().map(|val | {
        val.namnsdag.to_owned()
    }).flatten().collect()
}

/* 
Example JSON Payload
{
  "cachetid": "2021-09-29 21:06:03",
  "version": "2.1",
  "uri": "/dagar/v2.1/2015/01/06",
  "startdatum": "2015-01-06",
  "slutdatum": "2015-01-06",
  "dagar": [
    {
      "datum": "2015-01-06",
      "veckodag": "Tisdag",
      "arbetsfri dag": "Ja",
      "r\u00f6d dag": "Ja",
      "vecka": "02",
      "dag i vecka": "2",
      "helgdag": "Trettondedag jul",
      "namnsdag": ["Kasper", "Melker", "Baltsar"],
      "flaggdag": ""
    }
  ]
} 
*/
