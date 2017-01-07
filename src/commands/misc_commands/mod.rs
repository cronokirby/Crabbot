extern crate serenity;
use self::serenity::client::Context;
use self::serenity::model::{ChannelId, User};

use commands::messages::send_min_embed;


pub fn test<C>(context: Context, channel_id: C, author: User)
    where C: Into<ChannelId> {
    let name = author.name;
    let response = format!("{}, what is a test?", name);
    send_min_embed(context, channel_id, &*response);
}
