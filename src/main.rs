use dotenv::dotenv;
use teloxide::{prelude::*, utils::command::BotCommands, types::ParseMode};
pub use commands::Command;
pub use ap_env::get_env_variable;

#[path = "./architecture/commands.rs"]
mod commands;

#[path = "./lib/ap_env.rs"]
mod ap_env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::new(get_env_variable("BOT_TOKEN".to_string()));

    teloxide::commands_repl(bot, answer, Command::ty()).await;
}

async fn answer(bot: Bot, message: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => 
            bot.send_message(
                message.chat.id,
                 "Hi there!\nUse /help to see my commands"
                ).await?,
        Command::Help => 
            bot.send_message(
                message.chat.id,
                 Command::descriptions().to_string()
                ).await?,
        Command::Ping => 
            bot.send_message(
                message.chat.id,
                 "Pong!"
            ).await?,
        Command::Itg => 
            bot.send_message(
                message.chat.id, 
                format!(
                    "Your id: `{}`\nChat id: `{}`",
                    message.from().unwrap().id,
                    message.chat.id)
                ).parse_mode(ParseMode::MarkdownV2).await?
    };

    Ok(())
}