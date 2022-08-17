use serenity::async_trait;
use serenity::model::application::command::CommandOptionType;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Activity;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::interactions::application_command::ApplicationCommandOptionType;
use serenity::model::interactions::Interaction;
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

        let commands = Command::set_global_application_commands(&ctx.http, |cmds| {
            cmds.create_application_command(|cmd| {
                cmd.name("register_id_fgo")
                    .description("Register your FGO ID with the bot")
                    .create_option(|option| {
                        option
                            .name("Server")
                            .description("FGO Server of the ID")
                            .required(true)
                            .kind(CommandOptionType::String)
                            .add_string_choice("NA", "NA")
                            .add_string_choice("JP", "JP")
                    })
                    .create_option(|option| {
                        option
                            .name("ID")
                            .description("Your FGO ID")
                            .required(true)
                            .kind(CommandOptionType::String)
                    })
            })
            .create_application_command(|cmd| {
                cmd.name("id_fgo")
                    .description("Get the FGO ID of a user")
                    .create_option(|option| {
                        option
                            .name("Server")
                            .description("FGO Server of the ID")
                            .required(true)
                            .kind(CommandOptionType::String)
                            .add_string_choice("NA", "NA")
                            .add_string_choice("JP", "JP")
                    })
            })
        })
        .await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        println!("Received interaction : {:?}", interaction);
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {
                "register_id_fgo" => {
                    commands::register_id_fgo::register_id_fgo(ctx, &command).await
                }
                "id_fgo" => commands::get_id_fgo::get_id_fgo(ctx, &command).await,
                _ => println!("Unknown command"),
            }
        }
    }
}
