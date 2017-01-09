extern crate serenity;
use self::serenity::client::Context;
use self::serenity::model::{ChannelId, User};

use commands::messages::send_min_embed;
use http;


pub fn test<C>(context: Context, channel_id: C, author: User)
    where C: Into<ChannelId> {
    let name = author.name;
    let response = format!("{}, what is a test?", name);
    send_min_embed(context, channel_id, &*response);
}


pub fn number() {
    let url = "http://numbersapi.com/42";
    match http::get(&url) {
        Ok(content) => println!("{:?}", content),
        Err(why) => println!("command `number` failed:\n{:?}", why)
    }
}
