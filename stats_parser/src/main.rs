mod query;
use anyhow::Result;
use poise::serenity_prelude::{Colour, CreateEmbed, CreateEmbedFooter};
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
    for chunk in response {
        ctx.say(chunk).await?;
    }
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
    for chunk in response {
        ctx.say(chunk).await?;
    }
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


fn test_main(uuid: String, stats_option: Option<GetStatsOption>) -> Result<Vec<String>> {
    println!("stats/{}.json", uuid);
    let json_str = fs::read_to_string(format!("../../../school_smp/world/stats/{}.json", uuid))?;
    println!("../../school_smp/world/stats/{}.json", uuid);
    let stats = serde_json::from_str::<MinecraftStats>(&json_str)?.stats;
    let mut output = Vec::new();
    if let Some(stats_option) = stats_option {
        let mut temp_output = String::new();
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
        output.push(temp_output);
        return Ok(output);
    }
    for (key, value) in serde_json::to_value(&stats)?.as_object().unwrap() {
        let readable_key = MinecraftStats::get_readable_name(key);
        println!("## {}", readable_key);
        let mut temp_output = String::new();
        temp_output.push_str(&format!("## {} \n", readable_key));
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
        output.push(temp_output);
    }
    Ok(output)
}

/// Gets the playtime of all the players in the server.
#[poise::command(slash_command, prefix_command)]
async fn playtime(ctx: Context<'_>) -> Result<(), Error> {
    let usercache_text = fs::read_to_string("../../../school_smp/usercache.json").unwrap();
    let usercache: Vec<UserCache> = usercache_text.to_json().unwrap();
    let mut out = String::new();
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
    let mut reply = CreateReply {
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
            commands: vec![get_stats(), server(),get_stats_username(), playtime()],
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
