use crate::{
    errors::ParrotError,
    guild::settings::{GuildSettings, GuildSettingsMap},
    utils::create_embed_response,
};
use serenity::{
    builder::CreateEmbed, client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

pub async fn volume(
    ctx: &Context,
    interaction: &mut ApplicationCommandInteraction,
) -> Result<(), ParrotError> {
    let guild_id = interaction.guild_id.unwrap();
    let mut data = ctx.data.write().await;
    let settings = data.get_mut::<GuildSettingsMap>().unwrap();

    let guild_settings = settings
        .entry(guild_id)
        .or_insert_with(|| GuildSettings::new(guild_id));

    let args = interaction.data.options.clone();
    let volume = args
        .first()
        .and_then(|o| o.value.as_ref())
        .and_then(|v| v.as_f64())
        .map(|v| v as f32 / 100.0);

    match volume {
        None => {
            let mut embed = CreateEmbed::default();
            embed.description(format!("Volume: {}%", guild_settings.volume * 100.0));
            create_embed_response(&ctx.http, interaction, embed).await
        }
        Some(volume) => {
            guild_settings.set_volume(volume);
            guild_settings.save()?;

            let mut embed = CreateEmbed::default();
            embed.description(format!("Volume set to: {}%", volume * 100.0));
            create_embed_response(&ctx.http, interaction, embed).await
        }
    }
}
