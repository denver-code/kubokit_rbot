use dotenv::dotenv;
use internal::commands::itg_event;
use teloxide::{prelude::*, utils::command::BotCommands};
pub use commands::Command;
pub use ap_env::get_env_variable;

#[path = "./architecture/commands.rs"]
mod commands;

#[path = "./lib/ap_env.rs"]
mod ap_env;

pub use internal::*;
mod internal;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::new(get_env_variable("BOT_TOKEN".to_string()));

    teloxide::commands_repl(bot, command_handler_event, Command::ty()).await;
}

async fn command_handler_event(bot: Bot, message: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start =>            {
                bot.send_message(
                    message.chat.id,
                     "Hi there!\nUse /help to see my commands"
                    ).await?;
            },
        Command::Help =>{
            bot.send_message(
                message.chat.id,
                 Command::descriptions().to_string()
                ).await?;
        },
        Command::Ping => {
            bot.send_message(
                message.chat.id,
                 "Pong!"
            ).await?;
        },
        Command::Itg => itg_event(bot, message).await?
    };

    Ok(())
}