extern crate serenity;

use self::serenity::client::Context;
use self::serenity::model::User;


pub fn test(context: Context, author: User) {
    let name = author.name;
    let response = format!("{}, what is a test?", name);
    if let Err(why) = context.say(&*response) {
        println!("Error sending message: {:?}", why);
    }
}
