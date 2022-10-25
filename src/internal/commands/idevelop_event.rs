use teloxide::{prelude::*, types::ParseMode};

pub async fn idevelop_event(bot: Bot, message: Message) -> ResponseResult<()>{
    bot.send_message(
        message.chat.id, 
        "*CubeCat v1\\.4\\.0 CubeLavender* \n\
        Technical information: \n\
        \\- Rust \n\
        \\- Teloxide \n\
        \\- Coded by Denver\\(\\@rustsesh\\_bot\\)
        "
        ).parse_mode(ParseMode::MarkdownV2).await?;
    Ok(())
}