extern crate serenity;
use serenity::Client;

extern crate crabbot;
use crabbot::misc_commands::commands as misc_commands;

use std::env;




fn main() {
    // Configure the client to use a preset token
    let token = env::var("DISCORD_TOKEN1")
        .expect("Expected a token in the environment");

    // Creating a new instance of the client
    let mut client = Client::login_bot(&token);

    // Setting a handler for messages
    client.on_message(|context, message| {
        let author = message.author;
        let prefix = message.content.split_whitespace().nth(0);
        match prefix {
            Some("!test") => misc_commands::test(context, author),
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
