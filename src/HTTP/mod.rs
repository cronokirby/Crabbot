use std::io::Read;

extern crate hyper;
use self::hyper::client::Client;
use self::hyper::error::Result;



pub fn get(url: &str) -> Result<String> {
    let client = Client::new();
    let mut body = String::new();
    client.get(url).send()?
          .read_to_string(&mut body).unwrap();
    Ok(body)
}
