use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Start message.")]
    Start,
    #[command(description = "Display this text.")]
    Help,
    #[command(description = "Ping message.")]
    Ping,
    #[command(description = "Shows your id and chat id.")]
    Itg
}