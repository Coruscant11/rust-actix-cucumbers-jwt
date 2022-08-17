use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

pub async fn get_id_fgo(ctx: Context, command: &ApplicationCommandInteraction) {
    println!("Get command : {:?}", command);
}
