use serde_json::Value;
use ureq::get;
use std::env;

// TODO: Adding a cicle to manage library with more of 20 entry
fn main() {
    // getting command line args
    let args: Vec<String> = env::args().collect();
    // getting user id
    let username = &args[1];
    let id_url = "https://kitsu.io/api/edge/users?filter[name]=";
    let id_request_url = format!("{}{}", id_url, username);
    let response: ureq::Response = get(&id_request_url).call().unwrap();
    let body: String = response.into_string().unwrap();
    let json: Value = serde_json::from_str(&body).unwrap();
    let id = json
        .get("data")
        .and_then(|value| value.get(0))
        .and_then(|value| value.get("id"))
        .and_then(|value| value.as_str())
        .unwrap();

    // getting user library entries
    let url = "https://kitsu.io/api/edge/users/<id>/library-entries?page[limit]=20&page[offset]=0";
    let request_url = str::replace(url, "<id>", id);
    let response: ureq::Response = get(&request_url).call().unwrap();
    let body: String = response.into_string().unwrap();
    let json: Value = serde_json::from_str(&body).unwrap();
    let n_anime = json
        .get("meta")
        .and_then(|value| value.get("count"))
        .unwrap();
    let mut anime_ids: Vec<&str> = Vec::new();
    let mut i = 0;
    while i < n_anime.as_i64().unwrap() {
        let anime_id = &json["data"][i as usize]["id"];
        anime_ids.push(anime_id.as_str().unwrap());
        i = i + 1;
    }
    // interpretation of all library entries
    let mut anime_titles: Vec<String> = Vec::new();
    let url = "https://kitsu.io/api/edge/library-entries/<anime-id>/anime";
    let mut i = 0;
    while i < n_anime.as_i64().unwrap() {
        let request_url = str::replace(url, "<anime-id>", anime_ids[i as usize]);
        let response: ureq::Response = get(&request_url).call().unwrap();
        let body: String = response.into_string().unwrap();
        let json: Value  = serde_json::from_str(&body).unwrap();
        let anime_title = &json["data"]["attributes"]["canonicalTitle"];
        anime_titles.push(anime_title.as_str().unwrap().to_owned());
        println!("{}", anime_titles[i as usize]);
        i = i + 1;
    }
    // getting information about each anime
}
