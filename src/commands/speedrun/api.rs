extern crate hyper;
use self::hyper::error::Result;

extern crate rustc_serialize;
use self::rustc_serialize::json::{Json};

use http;


fn fetch<T>(url: &str, parser: &Fn(&str) -> T) -> Result<T> {
    let json = http::get(url);
    json.map(|str| parser(&*str))
}


type Url = String;

#[derive (Debug)]
pub struct Category {
    pub name: String,
    pub leaderboard: Url
}


// This acts as a helper function to `parse_categories`, getting mapped over an array
fn parse_category(json: &Json) -> Category {
    let data = json.as_object().unwrap();

    let name = data.get("name").unwrap().as_string().unwrap();

    let leaderboard = data.get("links").unwrap().as_array().unwrap()
                          [3].as_object().unwrap()
                          .get("uri").unwrap().as_string().unwrap();

    Category{ name: name.to_string()
            , leaderboard: leaderboard.to_string() }
}


// This function isn't really safe, but an error in json parsing is relatively rare
fn parse_categories(json_string: &str) -> Vec<Category> {
    // The use of 2 variables here is to extend the lifetime of `data`
    let data = Json::from_str(json_string).unwrap();
    let category_data = data.as_object().unwrap()
                            .get("data").unwrap().as_array().unwrap();
    let categories = category_data.iter()
                            .map(|json| parse_category(&json)).collect();
    categories
}



pub fn fetch_categories(url: &str) -> Result<Vec<Category>> {
    fetch(url, &parse_categories)
}


pub struct Run {
    pub user_name: String,
    pub time: String,
    pub video: Option<Url>
}


fn parse_name(json: &str) -> String {
    let data = Json::from_str(json).unwrap();
    data.as_object().unwrap()
        .get("data").unwrap().as_object().unwrap()
        .get("names").unwrap().as_object().unwrap()
        .get("international").unwrap().as_string().unwrap()
        .to_string()
}

// If the player is a user, and fetching the name fails, this will fail
fn parse_run(json: &Json) -> Run {
    let data = json.as_object().unwrap()
                   .get("run").unwrap().as_object().unwrap();
    let user_name = {
        let player = data.get("players").unwrap().as_array().unwrap()
                         [0].as_object().unwrap();
        let user_type = player.get("rel").unwrap().as_string().unwrap();
        match user_type {
            "user"  => {
                let url = player.get("uri").unwrap().as_string().unwrap();
                fetch(url, &parse_name).unwrap()
            },
            _ => player.get("name").unwrap().as_string().unwrap().to_string()
        }
    };
    let time = data.get("times").unwrap().as_object().unwrap()
                   .get("primary_t").unwrap();

    let video = data.get("videos").and_then(|links|
                    links.as_object().unwrap()
                         .get("links").unwrap().as_array().unwrap()
                         [0].as_object().unwrap()
                         .get("uri").unwrap().as_string())
                         .map(|s| s.to_string());
    Run{ user_name: user_name
       , time: time.to_string()
       , video: video }
}

fn parse_leaderboard(json_string: &str) -> Vec<Run> {
    let data = Json::from_str(json_string).unwrap();
    let leaderboard = data.as_object().unwrap()
                          .get("data").unwrap().as_array().unwrap()
                          [0].as_object().unwrap()
                          .get("runs").unwrap().as_array().unwrap();
    let runs = leaderboard.iter()
                          .map(|json| parse_run(&json))
                          .collect();
    runs
}

pub fn fetch_runs(url: &str) -> Result<Vec<Run>> {
    fetch(url, &parse_leaderboard)
}
