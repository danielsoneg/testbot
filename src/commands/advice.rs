// use serenity::framework::standard::{macros::command, Args, CommandResult};
// use serenity::model::prelude::*;
// use serenity::prelude::*;

// #[command]
// async fn advice(ctx: &Context, msg: &Message) -> CommandResult {
//     let endpoint = "https://api.adviceslip.com/advice";
//     let result_text = reqwest::blocking::get(endpoint)?.json()?;
//     // let result = serde_json::from_str(&result_text).expect("json from string failed");
//     let results = format!("{:?}", result_text);

//     let _ = msg.channel_id.say(&ctx.http, results).await;
//     Ok(())
// }

// #[command]
// async fn advice_id(_ctx: &Context, _msg: &Message, _args: Args) -> CommandResult {
//     let _endpoint = "https://api.adviceslip.com/advice/{slip_id}";
//     Ok(())
// }

// #[command]
// async fn advice_search(_ctx: &Context, _msg: &Message, _args: Args) -> CommandResult {
//     let _endpoint = "https://api.adviceslip.com/advice/search/{query}";
//     Ok(())
// }
