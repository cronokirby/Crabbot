use std::env;

extern crate serenity;
use serenity::Client;

extern crate crabbot;
use crabbot::commands;
use crabbot::commands::speedrun::commands as speedrun;


fn main() {
    // Configure the client to use a preset token
    let token = env::var("DISCORD_TOKEN1")
        .expect("Expected a token in the environment");

    // Creating a new instance of the client
    let mut client = Client::login_bot(&token);

    // Setting a handler for messages
    client.on_message(|context, message| {
        let author = message.author;
        let channel = message.channel_id;
        let words: Vec<&str> = message.content.split_whitespace().collect();
        // Match against the first word of a message
        let prefix = words.get(0);
        match prefix {
            Some(&"<test") => commands::test(context, channel, author),
            Some(&"<number") => commands::number(),
            Some(&"<categories") => speedrun::categories(context, channel, &words),
            Some(&"<time") => speedrun::time(context, channel, &words),
            _ => {}
        }
    });

    // Setting a handler for ready
    client.on_ready(|_context, ready| {
        println!("Connected as: {}", ready.user.name);
    });

    // Starting the bot
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
