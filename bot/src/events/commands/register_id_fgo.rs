use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

pub async fn register_id_fgo(ctx: Context, command: &ApplicationCommandInteraction) {
    println!("Register command : {:?}", command);
}
