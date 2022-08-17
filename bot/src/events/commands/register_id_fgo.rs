use crate::requests::api_client::{send_register_player_id_request_to_api, ApiError};
use lib::models::server::FGOServer;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use std::str::FromStr;

pub async fn register_id_fgo(ctx: Context, command: &ApplicationCommandInteraction) {
    println!("Register ID command received from: {}", command.user.tag());

    let option_server = match command.data.options.get(0).and_then(|v| v.value.clone()) {
        Some(o) => match FGOServer::from_str(o.as_str().unwrap_or("bad server")) {
            Ok(server) => server,
            Err(why) => {
                println!("{}", why);
                println!(
                    "Cancelling the register_id_fgo command from [{}] on the channel [{}]",
                    command.user.tag(),
                    command.channel_id
                );
                return;
            }
        },
        None => {
            println!("Failed resolving the server option on the command. Cancelling the register_id_fgo command from [{}] on the channel [{}]", command.user.tag(), command.channel_id);
            return;
        }
    };

    let option_fgo_id = match command.data.options.get(1).and_then(|v| v.value.clone()) {
        Some(o) => {
            if let Some(fgo_id) = o.as_str() {
                fgo_id.to_string()
            } else {
                println!("Please provide a valid FGO ID");
                return;
            }
        }
        None => {
            println!("Failed resolving the ID option on the command. Cancelling the register_id_fgo command from [{}] on the channel [{}]", command.user.tag(), command.channel_id);
            return;
        }
    };

    let mut option_fgo_id = option_fgo_id.trim().to_string();
    option_fgo_id.retain(|c| !c.is_whitespace());
    option_fgo_id.retain(|c| c.is_numeric());

    match send_register_player_id_request_to_api(
        &command.user.id.to_string(),
        option_server,
        &option_fgo_id,
        &command.user.name,
    )
    .await
    {
        Ok(()) => {
            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.content("The ID has been registered.")
                        })
                })
                .await
            {
                println!("Error sending the response : {:?}", why);
            }
        }
        Err(e) => {
            let error_message: String = match e {
                ApiError::BadFieldError => "The field are not valid.".to_string(),
                _ => "Failed to register the ID to the server.".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(error_message))
                })
                .await
            {
                println!("Error sending the response : {:?}", why);
            }
        }
    }
}
