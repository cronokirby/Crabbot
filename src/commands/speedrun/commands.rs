extern crate hyper;
use self::hyper::error::Result;

extern crate serenity;
use self::serenity::client::Context;
use self::serenity::model::ChannelId;

use commands::messages::send_min_embed;
use commands::speedrun::api;


fn format_categories(categories: &Vec<api::Category>) -> String {
    let names: Vec<String> = categories.iter().map(|cat|
                                       format!("*{}*", &*cat.name))
                                       .collect();
    names.join("\n")
}

fn get_categories(abbreviation: &str) -> Result<Vec<api::Category>> {
    let url = format!("http://www.speedrun.com/api/v1/games/{}/categories",
                      abbreviation);
    api::fetch_categories(&*url)
}

pub fn categories<C>(context: Context, channel_id: C, words: &Vec<&str>)
    where C: Into<ChannelId> {
    let abbreviation = words[1];
    let response = match get_categories(abbreviation) {
        Ok(cats) => format!("Here's a list of categories for **{}**:\n{}",
                               abbreviation, format_categories(&cats)),
        Err(why) => format!("There was an error somewhere: {:?}", why)
    };
    send_min_embed(context, channel_id, &*response);
}


fn format_run(run: &api::Run, rank: &str) -> String {
    let place = match rank {
        "1" => "**WR**".to_string(),
        "2" => "**2nd** place time".to_string(),
        "3" => "**3rd** place time".to_string(),
        n   => format!("**{}th** place time", n)
    };
    let time = {
        // removes the milliseconds
        let time = run.time.split(".").nth(0).unwrap()
                           .parse::<i32>().unwrap();
        let (minutes, seconds) = (time / 60, time % 60);
        let hours = match minutes / 60 {
            0 => "".to_string(),
            h => format!("{}:", h)
        };
        let minutes = minutes % 60;
        format!("{}{}:{}", hours, minutes, seconds)
    };
    format!("The {} is **{}** by **{}**\n{:?}", place, time, run.user_name, run.video)
}


pub fn time<C>(context: Context, channel_id: C, words: &Vec<&str>)
    where C: Into<ChannelId> {
    let place = words[1].parse::<usize>();
    if let Err(_) = place {
        send_min_embed(context, channel_id, "the first argument must be a number");
        return ()
    }
    let abbreviation = words[2];
    let cat_name = &words[3..].join(" ");
    let categories = get_categories(abbreviation);
    let leaderboard = categories.ok().and_then(|cats| cats.into_iter()
        .find(|category| category.name == *cat_name)
        .map(|cat| cat.leaderboard));
    let runs = leaderboard.and_then(|lb| api::fetch_runs(&lb).ok());
    let requested_run = runs.and_then(|r|
        r.get(place.unwrap() - 1)
         .map(|run| format_run(&run, words[1])));
    let response = match requested_run {
        Some(string) => string,
        None => "Woops, that run doesn't seem to exist...".to_string()
    };
    send_min_embed(context, channel_id, &*response);
}
