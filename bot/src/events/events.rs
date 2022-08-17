use serenity::async_trait;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Activity;
use serenity::model::gateway::Ready;
use serenity::model::prelude::command::Command;
use serenity::prelude::*;

use crate::events::commands;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.tag());

        /* Set the bot's activity. */
        let activity = Activity::watching("Kingdom");
        let _ = ctx.set_activity(activity).await;

        register_global_commands(&ctx).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        println!("Received interaction : {:?}", interaction.kind());
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {
                "set_id_fgo" => commands::register_id_fgo::register_id_fgo(ctx, &command).await,
                "id_fgo" => commands::get_id_fgo::get_id_fgo(ctx, &command).await,
                _ => println!("Unknown command"),
            }
        }
    }
}

async fn register_global_commands(ctx: &Context) {
    println!("Registering commands globally...");

    match Command::create_global_application_command(&ctx.http, |cmd| {
        cmd.name("set_id_fgo")
            .description("Register your FGO ID with the bot")
            .create_option(|option| {
                option
                    .name("server")
                    .description("FGO Server of the ID")
                    .required(true)
                    .kind(CommandOptionType::String)
                    .add_string_choice("NA", "NA")
                    .add_string_choice("JP", "JP")
            })
            .create_option(|option| {
                option
                    .name("id")
                    .description("Your FGO ID")
                    .required(true)
                    .kind(CommandOptionType::String)
            })
    })
    .await
    {
        Ok(_) => println!("Registered [set_id_fgo] command!"),
        Err(why) => println!("Error registering [set_id_fgo] command : {:?}", why),
    }

    match Command::create_global_application_command(&ctx.http, |cmd| {
        cmd.name("id_fgo")
            .description("Get the FGO ID of a user")
            .create_option(|option| {
                option
                    .name("user")
                    .description("The user to get the ID of")
                    .required(true)
                    .kind(CommandOptionType::User)
            })
    })
    .await
    {
        Ok(_) => println!("Registered [id_fgo] command!"),
        Err(why) => println!("Error registering [id_fgo] command: {:?}", why),
    }
}
