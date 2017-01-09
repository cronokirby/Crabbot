

extern crate rustc_serialize;
use self::rustc_serialize::json::{Json};

use http;


type url = String;

#[derive (Debug)]
struct Category {
    name: String
}

fn parse_category(json: &Json) -> Category {
    // This is theoretically unsafe...
    let name = json.as_object().unwrap()
                   .get("name").unwrap()
                   .as_string().unwrap();
    Category{ name: name.to_string() }
}



fn parse_categories(json_string: &str) -> Option<Vec<Category>> {
    let data = Json::from_str(json_string);
    if let Err(_) = data {
        return None
    }
    // This variable exists so that the data lives to the end of the scope
    let unwrapped = data.unwrap();
    let category_data = unwrapped.as_object()
                            .and_then(|f| f.get("data"))
                            .and_then(|f| f.as_array());
    // Not much point making this safe, since `parse_category` isn't
    let categories = category_data.unwrap()
                        .iter().map(|json| parse_category(&json))
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
