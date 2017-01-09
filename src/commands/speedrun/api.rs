

extern crate rustc_serialize;
use self::rustc_serialize::json::{Json};

use http;


type Url = String;

#[derive (Debug)]
struct Category {
    name: String
}

// This acts as a helper function to `parse_categories`, getting mapped over an array
fn parse_category(json: &Json) -> Category {
    let name = json.as_object().unwrap()
                   .get("name").unwrap()
                   .as_string().unwrap();
    Category{ name: name.to_string() }
}


// This function isn't really safe, but an error in json parsing is relatively rare
// Call this with `catch_unwind`, because of that.
fn parse_categories(json_string: &str) -> Option<Vec<Category>> {
    // The use of 2 variables here is to extend the lifetime of `data`
    let data = Json::from_str(json_string).unwrap();
    let category_data = data.as_object().unwrap()
                            .get("data").unwrap()
                            .as_array().unwrap();
    let categories = category_data.iter()
                            .map(|json| parse_category(&json))
                            .collect();
    Some(categories)
}


pub fn testo() {
    let json = http::get("http://www.speedrun.com/api/v1/games/ff1/categories");
    match json {
        Ok(string) => println!("{:?}", parse_categories(&*string)),
        _ => {}
    }

}
