extern crate serenity;
use self::serenity::client::Context;
use self::serenity::model::ChannelId;

use commands::messages::send_min_embed;
use commands::speedrun::api;


pub fn categories<C>(context: Context, channel_id: C, words: &Vec<&str>)
    where C: Into<ChannelId>{
    let abbreviation = words[1];
    let url = format!("http://www.speedrun.com/api/v1/games/{}/categories",
                      abbreviation);
    let categories = api::fetch_categories(&*url);
    let response = format!("Here's a list of categories for {}:\n{:?}",
                           abbreviation, categories);
    send_min_embed(context, channel_id, &*response);
}
