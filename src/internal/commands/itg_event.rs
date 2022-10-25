use teloxide::{prelude::*, types::ParseMode};

pub async fn itg_event(bot: Bot, message: Message) -> ResponseResult<()>{
    bot.send_message(
        message.chat.id, 
        format!(
            "Your id: `{}`\nChat id: `{}`",
            message.from().unwrap().id,
            message.chat.id)
        ).parse_mode(ParseMode::MarkdownV2).await?;
    Ok(())
}