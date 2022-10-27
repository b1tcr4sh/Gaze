use reqwest;
use reqwest::Result;

pub struct _Response {

}


pub enum Service {
    Modrinth,
    _Curseforge
}


pub async fn get_mod(_service: Service, query: &String) -> Result<String> {
    let base_url: &str = "https://api.modrinth.com/v2/search?q=";
    let request: String = String::from(base_url) + &query;


    let body = reqwest::get(request)
    .await?
    .text()
    .await?;

    return Ok(body);
}
