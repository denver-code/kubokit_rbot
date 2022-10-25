use dotenv::dotenv;
use teloxide::{prelude::*, utils::command::BotCommands};

use kubokit_rbot::{util, architecture::commands::Command};
use kubokit_rbot::internal::commands::{idevelop_event::idevelop_event, itg_event::itg_event};

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::new(util::ap_env::get_env_variable("BOT_TOKEN".to_string()));

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
        Command::Ping | Command::Iping => {
            bot.send_message(
                message.chat.id,
                 "Pong!"
            ).await?;
        },
        Command::Itg => itg_event(bot, message).await?,
        Command::Idevelop => idevelop_event(bot, message).await?
    };

    Ok(())
}