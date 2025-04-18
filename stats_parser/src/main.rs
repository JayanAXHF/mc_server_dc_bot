mod pagination;
mod query;
use anyhow::Result;
use pagination::paginate;
use poise::serenity_prelude::{Colour, CreateEmbed, CreateEmbedFooter, Guild};
use poise::serenity_prelude::{Role, RoleId};
use poise::{CreateReply, serenity_prelude as serenity};
use query::stat_full;
use serde_json::Value;
use stats_parser::MinecraftStats;
use stats_parser::*;
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;


struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

fn convert_to_readable(key: &str) -> String {
    let split = key.split(':').collect::<Vec<&str>>();
    let value = split[1].to_string().replace('_', " ");
    value
        .split(" ")
        .map(|word| {
            let chars = word.chars().collect::<Vec<char>>();
            let mut out = String::new();
            out.push_str(&chars[0].to_uppercase().to_string());
            for char in &chars[1..] {
                out.push_str(&char.to_lowercase().to_string());
            }
            out
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn get_username(uuid: &String) -> Result<String> {
    let json_str = fs::read_to_string("../../../school_smp/usercache.json")?;
    let usercache: Vec<UserCache> = serde_json::from_str(&json_str)?;
    let mut username = String::new();
    for value in usercache {
        if value.uuid == *uuid {
            username = value.name;
        }
    }
    Ok(username)
}

fn get_uuid(username: &String) -> Result<String> {
    let json_str = fs::read_to_string("../../../school_smp/usercache.json")?;
    let usercache: Vec<UserCache> = serde_json::from_str(&json_str)?;
    let mut uuid = String::new();
    for value in usercache {
        if value.name == *username {
            uuid = value.uuid;
            return Ok(uuid)
        }
    }
    Ok(uuid)
}

/// Gets the stats of a uuid (player). To find your uuid, login to the server and run `/list uuids`.
#[poise::command(slash_command, prefix_command)]
async fn get_stats(
    ctx: Context<'_>,
    #[description = "UUID"] uuid: String,
    #[description = "What stats category to display"] stats: Option<GetStatsOption>,
) -> Result<(), Error> {
    let username = get_username(&uuid)?;
    ctx.say(format!("Username: {}", username)).await?;
    let response = test_main(uuid, stats)?;
    paginate(ctx, response).await?;

    Ok(())
}

/// Gets the stats of a username (player). To find your uuid, login to the server and run `/list uuids`.
#[poise::command(slash_command, prefix_command)]
async fn get_stats_username(
    ctx: Context<'_>,
    #[description = "Username"] username: String,
    #[description = "What stats category to display"] stats: Option<GetStatsOption>,
) -> Result<(), Error> {
    ctx.say(format!("Username: {}", username)).await?;
    let uuid = get_uuid(&username)?;
    let response = test_main(uuid, stats)?;
    paginate(ctx, response).await?;

    Ok(())
}

/// [USE THIS] Gets the stats of the server.
#[poise::command(slash_command, prefix_command)]
async fn server(ctx: Context<'_>) -> Result<(), Error> {
    let response = stat_full("141.148.218.223", 25566).await?;
    let mut players = String::new();
    for player in response.players {
        players.push_str(&format!("- {}\n", player));
    }
    let fields = vec![
        ("Version", response.version, false),
        (
            "Players",
            format!(
                "{}/{} players online",
                response.num_players, response.max_players
            ),
            false,
        ),
        ("Players", players, false),
    ];
    let embed = CreateEmbed::new()
        .title("Server Info")
        .description(response.motd)
        .color(Colour::DARK_GREEN)
        .fields(fields)
        .footer(CreateEmbedFooter::new("Owned by FRXGFA"));

    let reply = CreateReply {
        embeds: vec![embed],
        ..Default::default()
    };
    ctx.send(reply).await?;

    Ok(())
}


fn test_main(uuid: String, stats_option: Option<GetStatsOption>) -> Result<Vec<CreateEmbed>> {
    println!("stats/{}.json", uuid);
    let json_str = fs::read_to_string(format!("../../../school_smp/world/stats/{}.json", uuid))?;
    println!("../../school_smp/world/stats/{}.json", uuid);
    let stats = serde_json::from_str::<MinecraftStats>(&json_str)?.stats;
    let mut embeds = vec![];
    if let Some(stats_option) = stats_option {
        let mut temp_output = String::new();
        let embed = CreateEmbed::new()
            .title(format!(
                "{} for {}",
                stats_option.get_name(),
                get_username(&uuid)?
            ))
            .color(Colour::DARK_GOLD);

        let display_stats = match stats_option {
            GetStatsOption::Custom => Some(&stats.custom),
            GetStatsOption::Mined => stats.mined.as_ref(),
            GetStatsOption::Crafted => stats.crafted.as_ref(),
            GetStatsOption::Used => stats.used.as_ref(),
            GetStatsOption::Broken => stats.broken.as_ref(),
            GetStatsOption::PickedUp => stats.picked_up.as_ref(),
            GetStatsOption::Dropped => stats.dropped.as_ref(),
            GetStatsOption::Killed => stats.killed.as_ref(),
            GetStatsOption::KilledBy => stats.killed_by.as_ref(),
            GetStatsOption::CustomEntity => stats.custom_entity.as_ref(),
        };
        if let Some(display_stats) = display_stats {
            for (key, value) in serde_json::to_value(display_stats)?.as_object().unwrap() {
                let readable_key = convert_to_readable(key);
                temp_output.push_str(&format!("\t**{}**: {}\n", readable_key, value));
            }
        }
        let embed = embed.description(&temp_output);
        embeds.push(embed);
        return Ok(embeds);
    }
    for (i, (key, value)) in serde_json::to_value(&stats)?
        .as_object()
        .unwrap()
        .iter()
        .enumerate()
    {

        let readable_key = MinecraftStats::get_readable_name(key);
        let mut temp_output = String::new();
        let value_obj = value.as_object();
        let converter_function = match &key[..] {
            "minecraft:custom" => create_used_stat_names(),
            "minecraft:mined" => create_mined_stat_names(),
            "minecraft:crafted" => create_crafted_stat_names(),
            "minecraft:used" => create_used_stat_names(),
            "minecraft:broken" => create_broken_stat_names(),
            "minecraft:picked_up" => create_picked_up_stat_names(),
            "minecraft:dropped" => create_dropped_stat_names(),
            "minecraft:killed" => create_killed_stat_names(),
            "minecraft:killed_by" => create_killed_stat_names(),
            "minecraft:custom_entity" => HashMap::new(),
            _ => HashMap::new(),
        };
        if let Some(value_obj) = value_obj {
            for (sub_key, sub_value) in value_obj {
                let readable_sub_key = converter_function.get(sub_key);
                if let Some(readable_sub_key) = readable_sub_key {
                    temp_output.push_str(&format!("\t**{}**: {}\n", readable_sub_key, sub_value));
                } else {
                    let readable_sub_key = convert_to_readable(sub_key);
                    temp_output.push_str(&format!("\t**{}**: {}\n", readable_sub_key, sub_value));
                }
            }
        }
        let chunks = temp_output.lines().collect::<Vec<&str>>();
        let chunks = chunks.chunks(20).collect::<Vec<&[&str]>>();
        let fields: Vec<(String, String, bool)> = chunks
            .iter()
            .enumerate()
            .map(|(i, chunk)| {
                let chunk = chunk.join("\n");
                (format!("Part {}/{}", i + 1, chunks.len()), chunk, false)
            })
            .collect();

        let embed = CreateEmbed::new()
            .title(format!("{} for {}", readable_key, get_username(&uuid)?))
            .color(Colour::DARK_GREEN)
            .fields(fields)
            .footer(CreateEmbedFooter::new(format!("Part {}/{}", i + 1, 10)));

        embeds.push(embed);
    }
    Ok(embeds)

}

/// Gets the playtime of all the players in the server.
#[poise::command(slash_command, prefix_command)]
async fn playtime(ctx: Context<'_>) -> Result<(), Error> {
    let usercache_text = fs::read_to_string("../../../school_smp/usercache.json").unwrap();
    let usercache: Vec<UserCache> = usercache_text.to_json().unwrap();
    let mut fields: Vec<(String, String, bool)> = Vec::new();


    for entry in WalkDir::new("../../../school_smp/world/stats")
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file = entry.path();
        if file.is_file() {
            let file_name = file.file_name().unwrap().to_str().unwrap();
            if file_name.ends_with(".json") {
                let json_str = fs::read_to_string(file).unwrap();
                let stats: Value = json_str.to_json().unwrap();
                let player_name = file_name.split(".").collect::<Vec<_>>()[0]
                    .to_string()
                    .find_player(usercache.clone());
                    fields.push((
                    player_name.clone(),
                    fmt_time(
                        stats["stats"]["minecraft:custom"]["minecraft:play_time"]
                            .to_string()
                            .parse::<u64>()
                            .unwrap(),
                    ),
                    true,
                ));

            }
        }
    }
    let embed = serenity::CreateEmbed::new()
        .title("Playtime")
        .fields(fields)
        .color(serenity::Colour::TEAL);
    let reply = CreateReply {
        embeds: vec![embed],
        ..Default::default()
    };
    ctx.send(reply).await?;


    Ok(())
}

/// Gets the profile of an account.
#[poise::command(slash_command, prefix_command)]
async fn profile(
    ctx: Context<'_>,
    #[description = "Username"] username: String,
) -> Result<(), Error> {
    let uuid = get_uuid(&username)?;
    let json_str = fs::read_to_string(format!("../../../school_smp/world/stats/{}.json", uuid))?;
    let stats = serde_json::from_str::<MinecraftStats>(&json_str)?.stats;
    let playtime_ticks = stats.custom.get("minecraft:play_time");
    let playtime_string = if let Some(playtime_ticks) = playtime_ticks {
        let playtime_ticks = *playtime_ticks;
        let playtime_u64: u64 = playtime_ticks.try_into().unwrap();
        fmt_time(playtime_u64)
    } else {
        "N/A".to_string()
    };
    let killed_by = stats.killed_by;
    let mut deaths = 0;
    for (_, value) in serde_json::to_value(killed_by)?.as_object().unwrap().iter() {
        deaths += value.as_u64().unwrap();
    }
    let custom = stats.custom;
    let kills = custom.get("minecraft:player_kills");
    let kills: u64 = if let Some(kills) = kills {
        let kills = *kills;
        kills.try_into().unwrap()
    } else {
        println!("No kills");
        0_u64
    };


    let embed = CreateEmbed::new()
        .title(format!("{}'s Profile", username))
        .description(format!("UUID: `{}`", uuid))
        .fields([
            ("Playtime".to_string(), playtime_string, true),
            ("Kills".to_string(), kills.to_string(), true),
            ("Deaths".to_string(), deaths.to_string(), true),
        ])
        .color(Colour::DARK_GREEN);

    let reply = CreateReply {
        embeds: vec![embed],
        ..Default::default()
    };
    ctx.send(reply).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn timetable(
    ctx: Context<'_>,
    #[description = "What day of the week"] day: Day,
    #[description = "Section"] section: Option<Section>,
) -> Result<(), Error> {
    let section = if let None = section {
        let user = ctx.author_member().await.unwrap();
        let user_roles = user.roles.clone();
        let server_roles = {
            let guild = ctx.guild().unwrap();
            guild
                .roles
                .clone()
                .iter()
                .map(|r| r.1.clone())
                .collect::<Vec<Role>>()
        };

        let server_roles = server_roles
            .iter()
            .filter(|r| {
                let name = r.name.clone().to_lowercase();
                let sections = [
                    "everest",
                    "nilgiris",
                    "kamet",
                    "aravallis",
                    "shivaliks",
                    "vindhyas",
                    "himalayas",
                ];
                sections.contains(&name.as_str())
            })
            .collect::<Vec<&Role>>();

        let mut has_section_role = None;
        for role in server_roles.iter() {
            let section = role.name.clone();
            let id = role.id;
            for role in user_roles.iter() {
                if *role == id {
                    has_section_role = Some(Section::from(section.as_str()));
                    break;
                }
            }
        }
        has_section_role.expect("No section role found")
    } else {
        section.unwrap()
    };

    let section_string = String::from(section);
    let toml_file = format!("{}.toml", section_string);
    let timetable_raw =
        fs::read_to_string(format!("timetables/{}", toml_file)).expect("Could not read timetable");
    let timetable: Timetable = toml::from_str(&timetable_raw).expect("Could not parse timetable");
    let day = timetable.get_day(day);
    let text = day
        .iter()
        .map(|x| format!("- {}", x))
        .collect::<Vec<String>>()
        .join("\n");
    let embed = CreateEmbed::new()
        .title(format!("Timetable for {}", section_string))
        .description(format!("**{}**", text))
        .color(Colour::DARK_GREEN);
    let reply = CreateReply {
        embeds: vec![embed],
        ..Default::default()
    };
    ctx.send(reply).await?;

    Ok(())
}



#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![get_stats(), server(),get_stats_username(), playtime(), profile(), timetable()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
