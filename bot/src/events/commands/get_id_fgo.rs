use crate::requests::api_client::{send_get_player_id_request_to_api, ApiError};
use serenity::client::Context;
use serenity::model::application::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};
use serenity::model::application::interaction::InteractionResponseType;
use serenity::utils::Color;

pub async fn get_id_fgo(ctx: Context, command: &ApplicationCommandInteraction) {
    println!("Get ID command received from {}", command.user.tag());

    let option_user = match command
        .data
        .options
        .get(0)
        .and_then(|v| v.resolved.as_ref())
    {
        Some(o) => {
            if let CommandDataOptionValue::User(user, _member) = o {
                user
            } else {
                println!("Please provide a valid user");
                return;
            }
        }
        None => {
            println!("Failed resolving the ID option on the command. Cancelling the register_id_fgo command from [{}] on the channel [{}]", command.user.tag(), command.channel_id);
            return;
        }
    };

    match send_get_player_id_request_to_api(&option_user.id.to_string()).await {
        Ok(player) => {
            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.embed(|embed| {
                                embed
                                    .title(format!("{}'s IDs", player.name))
                                    .field(
                                        "NA",
                                        if player.na_id.len() > 0 {
                                            &player.na_id
                                        } else {
                                            "None."
                                        },
                                        true,
                                    )
                                    .field(
                                        "JP",
                                        if player.jp_id.len() > 0 {
                                            &player.jp_id
                                        } else {
                                            "None."
                                        },
                                        true,
                                    )
                                    .color(Color::BLUE)
                            })
                        })
                })
                .await
            {
                println!("Error sending the response : {:?}", why);
            }
        }
        Err(e) => {
            let error_message: String = match e {
                ApiError::DoNotExistsError => {
                    "The user doesn't have any ID registered.".to_string()
                }
                ApiError::BadFieldError => "The field are not valid.".to_string(),
                _ => "Failed to get the ID from the server.".to_string(),
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
