extern crate serenity;
use self::serenity::client::Context;
use self::serenity::model::ChannelId;


// sends a default Embeddded message, handling errors
pub fn send_min_embed<C>(context: Context, channel_id: C, content: &str)
    where C: Into<ChannelId> {
    if let Err(why) = context.send_message(channel_id, |m| m
        .embed(|e| e
            .description(content)
        )
    ) { println!("Error sending embedded message {:?}", why); }
}
