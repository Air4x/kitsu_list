use serde_json::Value;
use std::env;

// TODO: Adding a cicle to manage library with more of 20 entry
// TODO: Adding a manpage for *nix system ?

fn getting_arg() -> String {
    env::args().nth(1).unwrap()
}

fn main() {
    let args: Vec<String> = vec!["--help".to_string()];
    if getting_arg().eq(&args[0]) {
        println!("How to use the program:");
        println!("\tkitsu_list <your username>");
        println!("or");
        println!("\tkitsu_list -h");
        println!("to print this help");
    } else {
        let agent = ureq::agent();
        // getting user id
        let username = getting_arg(); // User's username taken as a command line argument
        let id_url = "https://kitsu.io/api/edge/users?filter[name]=";
        let id_request_url = format!("{}{}", id_url, username); // Definitive url to obtain the user's ID
        let response: ureq::Response = agent.get(&id_request_url).call().unwrap();
        let body: String = response.into_string().unwrap();
        let json: Value = serde_json::from_str(&body).unwrap();
        let id = json
            .get("data")
            .and_then(|value| value.get(0))
            .and_then(|value| value.get("id"))
            .and_then(|value| value.as_str())
            .unwrap();

        // getting user library entries
        let url =
            "https://kitsu.io/api/edge/users/<id>/library-entries?page[limit]=20&page[offset]=0";
        let request_url = str::replace(url, "<id>", id);
        let response: ureq::Response = agent.get(&request_url).call().unwrap();
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
            i += 1;
        }
        // interpretation of all library entries
        let url = "https://kitsu.io/api/edge/library-entries/<anime-id>/anime";
        let mut i = 0;
        let separetor_string = " : ";
        while i < n_anime.as_i64().unwrap() {
            let request_url = str::replace(url, "<anime-id>", anime_ids[i as usize]);
            let response: ureq::Response = agent.get(&request_url).call().unwrap();
            let body: String = response.into_string().unwrap();
            let json: Value = serde_json::from_str(&body).unwrap();
            let anime_title = &json["data"]["attributes"]["canonicalTitle"]
                .as_str()
                .unwrap();
            let anime_status = &json["data"]["attributes"]["status"].as_str().unwrap();
            println!("{}{}{}", anime_title, separetor_string, anime_status);
            i += 1;
        }
    }
}
