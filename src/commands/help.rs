use crate::{errors::ParrotError, utils::create_embed_response};
use serenity::{
    builder::CreateEmbed, client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

pub async fn help(
    ctx: &Context,
    interaction: &mut ApplicationCommandInteraction,
) -> Result<(), ParrotError> {
    let embed = create_help_embed().await;
    create_embed_response(&ctx.http, interaction, embed).await
}

pub async fn create_help_embed() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.title("Bantuan Perintah Kang Ngamen");
    embed.description(r#"
- `/autopause` - Toggles whether to pause after a song ends.
- `/clear` - Clears the queue.
- `/leave` - Leave the voice channel the bot is connected to.
- `/managesources` - Manage streaming from different sources.
- `/np` - Displays information about the current track.
- `/pause` - Pauses the current track.
- `/play` - Add a track to the queue. Requires a "query" specifying the media to play.
- `/superplay` - Add a track to the queue in a special way. This includes subcommands like "next", "jump", "all", "reverse", and "shuffle", each requiring a "query".
- `/queue` - Shows the queue.
- `/remove` - Removes a track from the queue. This command allows specifying an "index" and optionally an "until" parameter for range deletion.
- `/repeat` - Toggles looping for the current track.
- `/resume` - Resumes the current track.
- `/seek` - Seeks current track to the given position. Requires a "timestamp" in the format HH:MM:SS.
- `/shuffle` - Shuffles the queue.
- `/skip` - Skips the current track. Allows specifying a "to" parameter for skipping to a specific track index.
- `/stop` - Stops the bot and clears the queue.
- `/summon` - Summons the bot in your voice channel.
- `/version` - Displays the current version.
- `/voteskip` - Starts a vote to skip the current track.
- `/help` - Displays the help message."#);

    embed.color(0x00bfff);

    embed
}
