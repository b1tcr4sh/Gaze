use reqwest;
use reqwest::Result;
use serde::Deserialize;
use serde_json;


pub enum Service {
    Modrinth,
    _Curseforge
}


impl SearchResponse {
    pub fn Print(&self) {
        println!("Showing {} of {} results:", self.limit, self.total_hits);

        for hit in self.hits {
            println!("{} / {} (Latest) -- {}", hit.title, hit.versions[0], hit.author);
            println!("  {}", hit.description);
        }
    }
}

pub async fn get(_service: Service, query: &String) -> Result<SearchResponse> {
    let base_url: &str = "https://api.modrinth.com/v2/search?q=";
    let request: String = String::from(base_url) + &query;


    let body = reqwest::get(request)
    .await?
    .text()
    .await?;

    
    let hits: SearchResponse = serde_json::from_str(&body).expect("Failed to deserialize res.");

    return Ok(hits);
}

#[derive(Deserialize)]
pub struct Hit {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub client_side: String,
    pub server_side: String,
    pub project_type: String,
    pub downloads: i32,
    pub icon_url: String,
    pub project_id: String,
    pub author: String,
    pub display_categories: Vec<String>,
    pub versions: Vec<String>,
    pub follows: i32,
    pub date_created: String,
    pub date_modified: String,
    pub latest_version: String,
    pub license: String,
    pub gallery: Vec<String>
}

#[derive(Deserialize)]
pub struct SearchResponse {
    pub hits: Vec<Hit>,
    pub offset: i32,
    pub limit: i32,
    pub total_hits: i32
}