use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main structure for Minecraft player statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct MinecraftStats {
    /// The player's statistics, organized by category
    pub stats: StatCategories,
    /// The player's data version
    #[serde(rename = "DataVersion")]
    pub data_version: i32,
}

/// Categories of statistics as defined by Minecraft
#[derive(Debug, Serialize, Deserialize)]
pub struct StatCategories {
    /// Custom statistics, including movement, deaths, and time played
    #[serde(rename = "minecraft:custom")]
    pub custom: HashMap<String, i32>,

    /// Blocks broken/mined by the player
    #[serde(rename = "minecraft:mined")]
    pub mined: Option<HashMap<String, i32>>,

    /// Items crafted by the player
    #[serde(rename = "minecraft:crafted")]
    pub crafted: Option<HashMap<String, i32>>,

    /// Items used by the player
    #[serde(rename = "minecraft:used")]
    pub used: Option<HashMap<String, i32>>,

    /// Items broken by the player (tools, armor, etc.)
    #[serde(rename = "minecraft:broken")]
    pub broken: Option<HashMap<String, i32>>,

    /// Items picked up by the player
    #[serde(rename = "minecraft:picked_up")]
    pub picked_up: Option<HashMap<String, i32>>,

    /// Items dropped by the player
    #[serde(rename = "minecraft:dropped")]
    pub dropped: Option<HashMap<String, i32>>,

    /// Mobs/players killed by the player
    #[serde(rename = "minecraft:killed")]
    pub killed: Option<HashMap<String, i32>>,

    /// Times the player was killed by mobs/players
    #[serde(rename = "minecraft:killed_by")]
    pub killed_by: Option<HashMap<String, i32>>,

    /// Custom entity interactions (like trading with villagers)
    #[serde(rename = "minecraft:custom_entity")]
    pub custom_entity: Option<HashMap<String, i32>>,
}

/// Options for the bot command
#[derive(poise::ChoiceParameter)]
pub enum GetStatsOption {
    #[name = "Custom Statistics"]
    #[name = "custom"]
    Custom,

    #[name = "Blocks Mined"]
    #[name = "mined"]
    Mined,

    #[name = "Items Crafted"]
    #[name = "crafted"]
    Crafted,

    #[name = "Items Used"]
    #[name = "used"]
    Used,

    #[name = "Items Broken"]
    #[name = "broken"]
    Broken,

    #[name = "Items Picked Up"]
    #[name = "picked_up"]
    PickedUp,

    #[name = "Items Dropped"]
    #[name = "dropped"]
    Dropped,

    #[name = "Mobs/Players Killed"]
    #[name = "killed"]
    Killed,

    #[name = "Killed By"]
    #[name = "killed_by"]
    KilledBy,

    #[name = "Custom Entity Interactions"]
    #[name = "custom_entity"]
    CustomEntity,
}

/// Common custom statistics as defined by the Minecraft Wiki
/// These are commonly used statistics that are part of the "minecraft:custom" category
#[derive(Debug, Serialize, Deserialize)]
pub struct CommonCustomStats {
    // Movement statistics
    #[serde(rename = "minecraft:aviate_one_cm")]
    pub aviate_one_cm: Option<i32>,
    #[serde(rename = "minecraft:boat_one_cm")]
    pub boat_one_cm: Option<i32>,
    #[serde(rename = "minecraft:climb_one_cm")]
    pub climb_one_cm: Option<i32>,
    #[serde(rename = "minecraft:crouch_one_cm")]
    pub crouch_one_cm: Option<i32>,
    #[serde(rename = "minecraft:fall_one_cm")]
    pub fall_one_cm: Option<i32>,
    #[serde(rename = "minecraft:fly_one_cm")]
    pub fly_one_cm: Option<i32>,
    #[serde(rename = "minecraft:horse_one_cm")]
    pub horse_one_cm: Option<i32>,
    #[serde(rename = "minecraft:minecart_one_cm")]
    pub minecart_one_cm: Option<i32>,
    #[serde(rename = "minecraft:pig_one_cm")]
    pub pig_one_cm: Option<i32>,
    #[serde(rename = "minecraft:sprint_one_cm")]
    pub sprint_one_cm: Option<i32>,
    #[serde(rename = "minecraft:strider_one_cm")]
    pub strider_one_cm: Option<i32>,
    #[serde(rename = "minecraft:swim_one_cm")]
    pub swim_one_cm: Option<i32>,
    #[serde(rename = "minecraft:walk_on_water_one_cm")]
    pub walk_on_water_one_cm: Option<i32>,
    #[serde(rename = "minecraft:walk_under_water_one_cm")]
    pub walk_under_water_one_cm: Option<i32>,
    #[serde(rename = "minecraft:walk_one_cm")]
    pub walk_one_cm: Option<i32>,

    // Interaction statistics
    #[serde(rename = "minecraft:animals_bred")]
    pub animals_bred: Option<i32>,
    #[serde(rename = "minecraft:bell_ring")]
    pub bell_ring: Option<i32>,
    #[serde(rename = "minecraft:eat_cake_slice")]
    pub eat_cake_slice: Option<i32>,
    #[serde(rename = "minecraft:cauldron_filled")]
    pub cauldron_filled: Option<i32>,
    #[serde(rename = "minecraft:cauldron_used")]
    pub cauldron_used: Option<i32>,
    #[serde(rename = "minecraft:chest_opened")]
    pub chest_opened: Option<i32>,
    #[serde(rename = "minecraft:clean_armor")]
    pub clean_armor: Option<i32>,
    #[serde(rename = "minecraft:clean_banner")]
    pub clean_banner: Option<i32>,
    #[serde(rename = "minecraft:clean_shulker_box")]
    pub clean_shulker_box: Option<i32>,
    #[serde(rename = "minecraft:enchant_item")]
    pub enchant_item: Option<i32>,
    #[serde(rename = "minecraft:fish_caught")]
    pub fish_caught: Option<i32>,
    #[serde(rename = "minecraft:inspect_dispenser")]
    pub inspect_dispenser: Option<i32>,
    #[serde(rename = "minecraft:inspect_dropper")]
    pub inspect_dropper: Option<i32>,
    #[serde(rename = "minecraft:inspect_hopper")]
    pub inspect_hopper: Option<i32>,
    #[serde(rename = "minecraft:interact_with_anvil")]
    pub interact_with_anvil: Option<i32>,
    #[serde(rename = "minecraft:interact_with_beacon")]
    pub interact_with_beacon: Option<i32>,
    #[serde(rename = "minecraft:interact_with_blast_furnace")]
    pub interact_with_blast_furnace: Option<i32>,
    #[serde(rename = "minecraft:interact_with_brewingstand")]
    pub interact_with_brewingstand: Option<i32>,
    #[serde(rename = "minecraft:interact_with_campfire")]
    pub interact_with_campfire: Option<i32>,
    #[serde(rename = "minecraft:interact_with_cartography_table")]
    pub interact_with_cartography_table: Option<i32>,
    #[serde(rename = "minecraft:interact_with_crafting_table")]
    pub interact_with_crafting_table: Option<i32>,
    #[serde(rename = "minecraft:interact_with_furnace")]
    pub interact_with_furnace: Option<i32>,
    #[serde(rename = "minecraft:interact_with_grindstone")]
    pub interact_with_grindstone: Option<i32>,
    #[serde(rename = "minecraft:interact_with_lectern")]
    pub interact_with_lectern: Option<i32>,
    #[serde(rename = "minecraft:interact_with_loom")]
    pub interact_with_loom: Option<i32>,
    #[serde(rename = "minecraft:interact_with_smithing_table")]
    pub interact_with_smithing_table: Option<i32>,
    #[serde(rename = "minecraft:interact_with_smoker")]
    pub interact_with_smoker: Option<i32>,
    #[serde(rename = "minecraft:interact_with_stonecutter")]
    pub interact_with_stonecutter: Option<i32>,
    #[serde(rename = "minecraft:open_barrel")]
    pub open_barrel: Option<i32>,
    #[serde(rename = "minecraft:open_enderchest")]
    pub open_enderchest: Option<i32>,
    #[serde(rename = "minecraft:play_noteblock")]
    pub play_noteblock: Option<i32>,
    #[serde(rename = "minecraft:play_record")]
    pub play_record: Option<i32>,
    #[serde(rename = "minecraft:pot_flower")]
    pub pot_flower: Option<i32>,
    #[serde(rename = "minecraft:shulker_box_opened")]
    pub shulker_box_opened: Option<i32>,
    #[serde(rename = "minecraft:sleep_in_bed")]
    pub sleep_in_bed: Option<i32>,
    #[serde(rename = "minecraft:talked_to_villager")]
    pub talked_to_villager: Option<i32>,
    #[serde(rename = "minecraft:target_hit")]
    pub target_hit: Option<i32>,
    #[serde(rename = "minecraft:traded_with_villager")]
    pub traded_with_villager: Option<i32>,
    #[serde(rename = "minecraft:trigger_trapped_chest")]
    pub trigger_trapped_chest: Option<i32>,
    #[serde(rename = "minecraft:tune_noteblock")]
    pub tune_noteblock: Option<i32>,
    #[serde(rename = "minecraft:use_cauldron")]
    pub use_cauldron: Option<i32>,

    // Combat statistics
    #[serde(rename = "minecraft:damage_absorbed")]
    pub damage_absorbed: Option<i32>,
    #[serde(rename = "minecraft:damage_blocked_by_shield")]
    pub damage_blocked_by_shield: Option<i32>,
    #[serde(rename = "minecraft:damage_dealt")]
    pub damage_dealt: Option<i32>,
    #[serde(rename = "minecraft:damage_dealt_absorbed")]
    pub damage_dealt_absorbed: Option<i32>,
    #[serde(rename = "minecraft:damage_dealt_resisted")]
    pub damage_dealt_resisted: Option<i32>,
    #[serde(rename = "minecraft:damage_resisted")]
    pub damage_resisted: Option<i32>,
    #[serde(rename = "minecraft:damage_taken")]
    pub damage_taken: Option<i32>,
    #[serde(rename = "minecraft:deaths")]
    pub deaths: Option<i32>,
    #[serde(rename = "minecraft:mob_kills")]
    pub mob_kills: Option<i32>,
    #[serde(rename = "minecraft:player_kills")]
    pub player_kills: Option<i32>,

    // Miscellaneous statistics
    #[serde(rename = "minecraft:crouch_time")]
    pub crouch_time: Option<i32>,
    #[serde(rename = "minecraft:jump")]
    pub jump: Option<i32>,
    #[serde(rename = "minecraft:leave_game")]
    pub leave_game: Option<i32>,
    #[serde(rename = "minecraft:play_time")]
    pub play_time: Option<i32>,
    #[serde(rename = "minecraft:sneak_time")]
    pub sneak_time: Option<i32>,
    #[serde(rename = "minecraft:time_since_death")]
    pub time_since_death: Option<i32>,
    #[serde(rename = "minecraft:time_since_rest")]
    pub time_since_rest: Option<i32>,
    #[serde(rename = "minecraft:total_world_time")]
    pub total_world_time: Option<i32>,

    // Raid statistics
    #[serde(rename = "minecraft:raid_trigger")]
    pub raid_trigger: Option<i32>,
    #[serde(rename = "minecraft:raid_win")]
    pub raid_win: Option<i32>,
}

// Helper function to extract common custom stats
impl MinecraftStats {
    pub fn get_common_custom_stats(&self) -> CommonCustomStats {
        CommonCustomStats {
            // Movement statistics
            aviate_one_cm: self.stats.custom.get("minecraft:aviate_one_cm").cloned(),
            boat_one_cm: self.stats.custom.get("minecraft:boat_one_cm").cloned(),
            climb_one_cm: self.stats.custom.get("minecraft:climb_one_cm").cloned(),
            crouch_one_cm: self.stats.custom.get("minecraft:crouch_one_cm").cloned(),
            fall_one_cm: self.stats.custom.get("minecraft:fall_one_cm").cloned(),
            fly_one_cm: self.stats.custom.get("minecraft:fly_one_cm").cloned(),
            horse_one_cm: self.stats.custom.get("minecraft:horse_one_cm").cloned(),
            minecart_one_cm: self.stats.custom.get("minecraft:minecart_one_cm").cloned(),
            pig_one_cm: self.stats.custom.get("minecraft:pig_one_cm").cloned(),
            sprint_one_cm: self.stats.custom.get("minecraft:sprint_one_cm").cloned(),
            strider_one_cm: self.stats.custom.get("minecraft:strider_one_cm").cloned(),
            swim_one_cm: self.stats.custom.get("minecraft:swim_one_cm").cloned(),
            walk_on_water_one_cm: self
                .stats
                .custom
                .get("minecraft:walk_on_water_one_cm")
                .cloned(),
            walk_under_water_one_cm: self
                .stats
                .custom
                .get("minecraft:walk_under_water_one_cm")
                .cloned(),
            walk_one_cm: self.stats.custom.get("minecraft:walk_one_cm").cloned(),

            // Interaction statistics
            animals_bred: self.stats.custom.get("minecraft:animals_bred").cloned(),
            bell_ring: self.stats.custom.get("minecraft:bell_ring").cloned(),
            eat_cake_slice: self.stats.custom.get("minecraft:eat_cake_slice").cloned(),
            cauldron_filled: self.stats.custom.get("minecraft:cauldron_filled").cloned(),
            cauldron_used: self.stats.custom.get("minecraft:cauldron_used").cloned(),
            chest_opened: self.stats.custom.get("minecraft:chest_opened").cloned(),
            clean_armor: self.stats.custom.get("minecraft:clean_armor").cloned(),
            clean_banner: self.stats.custom.get("minecraft:clean_banner").cloned(),
            clean_shulker_box: self
                .stats
                .custom
                .get("minecraft:clean_shulker_box")
                .cloned(),
            enchant_item: self.stats.custom.get("minecraft:enchant_item").cloned(),
            fish_caught: self.stats.custom.get("minecraft:fish_caught").cloned(),
            inspect_dispenser: self
                .stats
                .custom
                .get("minecraft:inspect_dispenser")
                .cloned(),
            inspect_dropper: self.stats.custom.get("minecraft:inspect_dropper").cloned(),
            inspect_hopper: self.stats.custom.get("minecraft:inspect_hopper").cloned(),
            interact_with_anvil: self
                .stats
                .custom
                .get("minecraft:interact_with_anvil")
                .cloned(),
            interact_with_beacon: self
                .stats
                .custom
                .get("minecraft:interact_with_beacon")
                .cloned(),
            interact_with_blast_furnace: self
                .stats
                .custom
                .get("minecraft:interact_with_blast_furnace")
                .cloned(),
            interact_with_brewingstand: self
                .stats
                .custom
                .get("minecraft:interact_with_brewingstand")
                .cloned(),
            interact_with_campfire: self
                .stats
                .custom
                .get("minecraft:interact_with_campfire")
                .cloned(),
            interact_with_cartography_table: self
                .stats
                .custom
                .get("minecraft:interact_with_cartography_table")
                .cloned(),
            interact_with_crafting_table: self
                .stats
                .custom
                .get("minecraft:interact_with_crafting_table")
                .cloned(),
            interact_with_furnace: self
                .stats
                .custom
                .get("minecraft:interact_with_furnace")
                .cloned(),
            interact_with_grindstone: self
                .stats
                .custom
                .get("minecraft:interact_with_grindstone")
                .cloned(),
            interact_with_lectern: self
                .stats
                .custom
                .get("minecraft:interact_with_lectern")
                .cloned(),
            interact_with_loom: self
                .stats
                .custom
                .get("minecraft:interact_with_loom")
                .cloned(),
            interact_with_smithing_table: self
                .stats
                .custom
                .get("minecraft:interact_with_smithing_table")
                .cloned(),
            interact_with_smoker: self
                .stats
                .custom
                .get("minecraft:interact_with_smoker")
                .cloned(),
            interact_with_stonecutter: self
                .stats
                .custom
                .get("minecraft:interact_with_stonecutter")
                .cloned(),
            open_barrel: self.stats.custom.get("minecraft:open_barrel").cloned(),
            open_enderchest: self.stats.custom.get("minecraft:open_enderchest").cloned(),
            play_noteblock: self.stats.custom.get("minecraft:play_noteblock").cloned(),
            play_record: self.stats.custom.get("minecraft:play_record").cloned(),
            pot_flower: self.stats.custom.get("minecraft:pot_flower").cloned(),
            shulker_box_opened: self
                .stats
                .custom
                .get("minecraft:shulker_box_opened")
                .cloned(),
            sleep_in_bed: self.stats.custom.get("minecraft:sleep_in_bed").cloned(),
            talked_to_villager: self
                .stats
                .custom
                .get("minecraft:talked_to_villager")
                .cloned(),
            target_hit: self.stats.custom.get("minecraft:target_hit").cloned(),
            traded_with_villager: self
                .stats
                .custom
                .get("minecraft:traded_with_villager")
                .cloned(),
            trigger_trapped_chest: self
                .stats
                .custom
                .get("minecraft:trigger_trapped_chest")
                .cloned(),
            tune_noteblock: self.stats.custom.get("minecraft:tune_noteblock").cloned(),
            use_cauldron: self.stats.custom.get("minecraft:use_cauldron").cloned(),

            // Combat statistics
            damage_absorbed: self.stats.custom.get("minecraft:damage_absorbed").cloned(),
            damage_blocked_by_shield: self
                .stats
                .custom
                .get("minecraft:damage_blocked_by_shield")
                .cloned(),
            damage_dealt: self.stats.custom.get("minecraft:damage_dealt").cloned(),
            damage_dealt_absorbed: self
                .stats
                .custom
                .get("minecraft:damage_dealt_absorbed")
                .cloned(),
            damage_dealt_resisted: self
                .stats
                .custom
                .get("minecraft:damage_dealt_resisted")
                .cloned(),
            damage_resisted: self.stats.custom.get("minecraft:damage_resisted").cloned(),
            damage_taken: self.stats.custom.get("minecraft:damage_taken").cloned(),
            deaths: self.stats.custom.get("minecraft:deaths").cloned(),
            mob_kills: self.stats.custom.get("minecraft:mob_kills").cloned(),
            player_kills: self.stats.custom.get("minecraft:player_kills").cloned(),

            // Miscellaneous statistics
            crouch_time: self.stats.custom.get("minecraft:crouch_time").cloned(),
            jump: self.stats.custom.get("minecraft:jump").cloned(),
            leave_game: self.stats.custom.get("minecraft:leave_game").cloned(),
            play_time: self.stats.custom.get("minecraft:play_time").cloned(),
            sneak_time: self.stats.custom.get("minecraft:sneak_time").cloned(),
            time_since_death: self.stats.custom.get("minecraft:time_since_death").cloned(),
            time_since_rest: self.stats.custom.get("minecraft:time_since_rest").cloned(),
            total_world_time: self.stats.custom.get("minecraft:total_world_time").cloned(),

            // Raid statistics
            raid_trigger: self.stats.custom.get("minecraft:raid_trigger").cloned(),
            raid_win: self.stats.custom.get("minecraft:raid_win").cloned(),
        }
    }
    pub fn get_readable_name(category: &str) -> String {
        match category {
            "minecraft:custom" => "Custom Statistic".to_string(),
            "minecraft:mined" => "Mined Statistic".to_string(),
            "minecraft:crafted" => "Crafted Statistic".to_string(),
            "minecraft:used" => "Used Statistic".to_string(),
            "minecraft:broken" => "Broken Statistic".to_string(),
            "minecraft:picked_up" => "Picked Up Statistic".to_string(),
            "minecraft:dropped" => "Dropped Statistic".to_string(),
            "minecraft:killed" => "Killed Statistic".to_string(),
            "minecraft:killed_by" => "Killed By Statistic".to_string(),
            "minecraft:custom_entity" => "Custom Entity Statistic".to_string(),
            _ => "Unknown Statistic".to_string(),
        }
    }
}

pub fn create_killed_by_stat_names() -> HashMap<String, String> {
    let mut map = HashMap::new();

    // Hostile mobs
    map.insert(
        "minecraft:zombie".to_string(),
        "Killed by Zombies".to_string(),
    );
    map.insert(
        "minecraft:skeleton".to_string(),
        "Killed by Skeletons".to_string(),
    );
    map.insert(
        "minecraft:creeper".to_string(),
        "Killed by Creepers".to_string(),
    );
    map.insert(
        "minecraft:spider".to_string(),
        "Killed by Spiders".to_string(),
    );
    map.insert(
        "minecraft:enderman".to_string(),
        "Killed by Endermen".to_string(),
    );
    map.insert(
        "minecraft:witch".to_string(),
        "Killed by Witches".to_string(),
    );
    map.insert(
        "minecraft:slime".to_string(),
        "Killed by Slimes".to_string(),
    );
    map.insert(
        "minecraft:phantom".to_string(),
        "Killed by Phantoms".to_string(),
    );
    map.insert(
        "minecraft:drowned".to_string(),
        "Killed by Drowned".to_string(),
    );
    map.insert(
        "minecraft:blaze".to_string(),
        "Killed by Blazes".to_string(),
    );
    map.insert(
        "minecraft:ghast".to_string(),
        "Killed by Ghasts".to_string(),
    );
    map.insert(
        "minecraft:magma_cube".to_string(),
        "Killed by Magma Cubes".to_string(),
    );
    map.insert(
        "minecraft:wither_skeleton".to_string(),
        "Killed by Wither Skeletons".to_string(),
    );
    map.insert(
        "minecraft:piglin".to_string(),
        "Killed by Piglins".to_string(),
    );
    map.insert(
        "minecraft:piglin_brute".to_string(),
        "Killed by Piglin Brutes".to_string(),
    );
    map.insert(
        "minecraft:hoglin".to_string(),
        "Killed by Hoglins".to_string(),
    );
    map.insert(
        "minecraft:zoglin".to_string(),
        "Killed by Zoglins".to_string(),
    );
    map.insert(
        "minecraft:vindicator".to_string(),
        "Killed by Vindicators".to_string(),
    );
    map.insert(
        "minecraft:evoker".to_string(),
        "Killed by Evokers".to_string(),
    );
    map.insert(
        "minecraft:ravager".to_string(),
        "Killed by Ravagers".to_string(),
    );

    // Bosses
    map.insert(
        "minecraft:ender_dragon".to_string(),
        "Killed by Ender Dragons".to_string(),
    );
    map.insert(
        "minecraft:wither".to_string(),
        "Killed by Withers".to_string(),
    );
    map.insert(
        "minecraft:elder_guardian".to_string(),
        "Killed by Elder Guardians".to_string(),
    );

    // Passive mobs
    map.insert("minecraft:bee".to_string(), "Killed by Bees".to_string());
    map.insert("minecraft:wolf".to_string(), "Killed by Wolves".to_string());
    map.insert(
        "minecraft:iron_golem".to_string(),
        "Killed by Iron Golems".to_string(),
    );

    // Special cases
    map.insert(
        "minecraft:player".to_string(),
        "Killed by Players".to_string(),
    );

    map
}

pub fn create_custom_stat_names() -> HashMap<String, String> {
    let mut map = HashMap::new();

    // Movement statistics
    map.insert(
        "minecraft:aviate_one_cm".to_string(),
        "Distance by Elytra".to_string(),
    );
    map.insert(
        "minecraft:boat_one_cm".to_string(),
        "Distance by Boat".to_string(),
    );
    map.insert(
        "minecraft:climb_one_cm".to_string(),
        "Distance Climbed".to_string(),
    );
    map.insert(
        "minecraft:crouch_one_cm".to_string(),
        "Distance Crouched".to_string(),
    );
    map.insert(
        "minecraft:fall_one_cm".to_string(),
        "Distance Fallen".to_string(),
    );
    map.insert(
        "minecraft:fly_one_cm".to_string(),
        "Distance Flown".to_string(),
    );
    map.insert(
        "minecraft:horse_one_cm".to_string(),
        "Distance by Horse".to_string(),
    );
    map.insert(
        "minecraft:minecart_one_cm".to_string(),
        "Distance by Minecart".to_string(),
    );
    map.insert(
        "minecraft:pig_one_cm".to_string(),
        "Distance by Pig".to_string(),
    );
    map.insert(
        "minecraft:sprint_one_cm".to_string(),
        "Distance Sprinted".to_string(),
    );
    map.insert(
        "minecraft:strider_one_cm".to_string(),
        "Distance on Strider".to_string(),
    );
    map.insert(
        "minecraft:swim_one_cm".to_string(),
        "Distance Swum".to_string(),
    );
    map.insert(
        "minecraft:walk_on_water_one_cm".to_string(),
        "Distance Walked on Water".to_string(),
    );
    map.insert(
        "minecraft:walk_under_water_one_cm".to_string(),
        "Distance Walked under Water".to_string(),
    );
    map.insert(
        "minecraft:walk_one_cm".to_string(),
        "Distance Walked".to_string(),
    );

    // Interaction statistics
    map.insert(
        "minecraft:animals_bred".to_string(),
        "Animals Bred".to_string(),
    );
    map.insert("minecraft:bell_ring".to_string(), "Bell Rings".to_string());
    map.insert(
        "minecraft:eat_cake_slice".to_string(),
        "Cake Slices Eaten".to_string(),
    );
    map.insert(
        "minecraft:cauldron_filled".to_string(),
        "Cauldrons Filled".to_string(),
    );
    map.insert(
        "minecraft:cauldron_used".to_string(),
        "Water Taken from Cauldron".to_string(),
    );
    map.insert(
        "minecraft:chest_opened".to_string(),
        "Chests Opened".to_string(),
    );
    map.insert(
        "minecraft:clean_armor".to_string(),
        "Armor Pieces Cleaned".to_string(),
    );
    map.insert(
        "minecraft:clean_banner".to_string(),
        "Banners Cleaned".to_string(),
    );
    map.insert(
        "minecraft:clean_shulker_box".to_string(),
        "Shulker Boxes Cleaned".to_string(),
    );
    map.insert(
        "minecraft:enchant_item".to_string(),
        "Items Enchanted".to_string(),
    );
    map.insert(
        "minecraft:fish_caught".to_string(),
        "Fish Caught".to_string(),
    );
    map.insert(
        "minecraft:inspect_dispenser".to_string(),
        "Dispensers Searched".to_string(),
    );
    map.insert(
        "minecraft:inspect_dropper".to_string(),
        "Droppers Searched".to_string(),
    );
    map.insert(
        "minecraft:inspect_hopper".to_string(),
        "Hoppers Searched".to_string(),
    );
    map.insert(
        "minecraft:interact_with_anvil".to_string(),
        "Interactions with Anvil".to_string(),
    );
    map.insert(
        "minecraft:interact_with_beacon".to_string(),
        "Interactions with Beacon".to_string(),
    );
    map.insert(
        "minecraft:interact_with_blast_furnace".to_string(),
        "Interactions with Blast Furnace".to_string(),
    );
    map.insert(
        "minecraft:interact_with_brewingstand".to_string(),
        "Interactions with Brewing Stand".to_string(),
    );
    map.insert(
        "minecraft:interact_with_campfire".to_string(),
        "Interactions with Campfire".to_string(),
    );
    map.insert(
        "minecraft:interact_with_cartography_table".to_string(),
        "Interactions with Cartography Table".to_string(),
    );
    map.insert(
        "minecraft:interact_with_crafting_table".to_string(),
        "Interactions with Crafting Table".to_string(),
    );
    map.insert(
        "minecraft:interact_with_furnace".to_string(),
        "Interactions with Furnace".to_string(),
    );
    map.insert(
        "minecraft:interact_with_grindstone".to_string(),
        "Interactions with Grindstone".to_string(),
    );
    map.insert(
        "minecraft:interact_with_lectern".to_string(),
        "Interactions with Lectern".to_string(),
    );
    map.insert(
        "minecraft:interact_with_loom".to_string(),
        "Interactions with Loom".to_string(),
    );
    map.insert(
        "minecraft:interact_with_smithing_table".to_string(),
        "Interactions with Smithing Table".to_string(),
    );
    map.insert(
        "minecraft:interact_with_smoker".to_string(),
        "Interactions with Smoker".to_string(),
    );
    map.insert(
        "minecraft:interact_with_stonecutter".to_string(),
        "Interactions with Stonecutter".to_string(),
    );
    map.insert(
        "minecraft:open_barrel".to_string(),
        "Barrels Opened".to_string(),
    );
    map.insert(
        "minecraft:open_enderchest".to_string(),
        "Ender Chests Opened".to_string(),
    );
    map.insert(
        "minecraft:play_noteblock".to_string(),
        "Note Blocks Played".to_string(),
    );
    map.insert(
        "minecraft:play_record".to_string(),
        "Records Played".to_string(),
    );
    map.insert(
        "minecraft:pot_flower".to_string(),
        "Flowers Potted".to_string(),
    );
    map.insert(
        "minecraft:shulker_box_opened".to_string(),
        "Shulker Boxes Opened".to_string(),
    );
    map.insert(
        "minecraft:sleep_in_bed".to_string(),
        "Times Slept in a Bed".to_string(),
    );
    map.insert(
        "minecraft:talked_to_villager".to_string(),
        "Talks with Villagers".to_string(),
    );
    map.insert(
        "minecraft:target_hit".to_string(),
        "Targets Hit".to_string(),
    );
    map.insert(
        "minecraft:traded_with_villager".to_string(),
        "Traded with Villagers".to_string(),
    );
    map.insert(
        "minecraft:trigger_trapped_chest".to_string(),
        "Trapped Chests Triggered".to_string(),
    );
    map.insert(
        "minecraft:tune_noteblock".to_string(),
        "Note Blocks Tuned".to_string(),
    );
    map.insert(
        "minecraft:use_cauldron".to_string(),
        "Cauldrons Used".to_string(),
    );

    // Combat statistics
    map.insert(
        "minecraft:damage_absorbed".to_string(),
        "Damage Absorbed".to_string(),
    );
    map.insert(
        "minecraft:damage_blocked_by_shield".to_string(),
        "Damage Blocked by Shield".to_string(),
    );
    map.insert(
        "minecraft:damage_dealt".to_string(),
        "Damage Dealt".to_string(),
    );
    map.insert(
        "minecraft:damage_dealt_absorbed".to_string(),
        "Damage Dealt (Absorbed)".to_string(),
    );
    map.insert(
        "minecraft:damage_dealt_resisted".to_string(),
        "Damage Dealt (Resisted)".to_string(),
    );
    map.insert(
        "minecraft:damage_resisted".to_string(),
        "Damage Resisted".to_string(),
    );
    map.insert(
        "minecraft:damage_taken".to_string(),
        "Damage Taken".to_string(),
    );
    map.insert(
        "minecraft:deaths".to_string(),
        "Number of Deaths".to_string(),
    );
    map.insert("minecraft:mob_kills".to_string(), "Mob Kills".to_string());
    map.insert(
        "minecraft:player_kills".to_string(),
        "Player Kills".to_string(),
    );

    // Miscellaneous statistics
    map.insert(
        "minecraft:crouch_time".to_string(),
        "Time Crouched".to_string(),
    );
    map.insert("minecraft:jump".to_string(), "Jumps".to_string());
    map.insert("minecraft:leave_game".to_string(), "Games Quit".to_string());
    map.insert("minecraft:play_time".to_string(), "Play Time".to_string());
    map.insert(
        "minecraft:sneak_time".to_string(),
        "Time Sneaked".to_string(),
    );
    map.insert(
        "minecraft:time_since_death".to_string(),
        "Time Since Last Death".to_string(),
    );
    map.insert(
        "minecraft:time_since_rest".to_string(),
        "Time Since Last Rest".to_string(),
    );
    map.insert(
        "minecraft:total_world_time".to_string(),
        "Total World Time".to_string(),
    );

    // Raid statistics
    map.insert(
        "minecraft:raid_trigger".to_string(),
        "Raids Triggered".to_string(),
    );
    map.insert("minecraft:raid_win".to_string(), "Raids Won".to_string());

    map
}

/// Creates mapping for mined block statistics
pub fn create_mined_stat_names() -> HashMap<String, String> {
    let mut map = HashMap::new();

    // Common blocks
    map.insert("minecraft:stone".to_string(), "Stone Mined".to_string());
    map.insert("minecraft:dirt".to_string(), "Dirt Mined".to_string());
    map.insert(
        "minecraft:grass_block".to_string(),
        "Grass Blocks Mined".to_string(),
    );
    map.insert("minecraft:gravel".to_string(), "Gravel Mined".to_string());
    map.insert("minecraft:sand".to_string(), "Sand Mined".to_string());
    map.insert(
        "minecraft:coal_ore".to_string(),
        "Coal Ore Mined".to_string(),
    );
    map.insert(
        "minecraft:iron_ore".to_string(),
        "Iron Ore Mined".to_string(),
    );
    map.insert(
        "minecraft:gold_ore".to_string(),
        "Gold Ore Mined".to_string(),
    );
    map.insert(
        "minecraft:diamond_ore".to_string(),
        "Diamond Ore Mined".to_string(),
    );
    map.insert(
        "minecraft:redstone_ore".to_string(),
        "Redstone Ore Mined".to_string(),
    );
    map.insert(
        "minecraft:lapis_ore".to_string(),
        "Lapis Lazuli Ore Mined".to_string(),
    );
    map.insert(
        "minecraft:emerald_ore".to_string(),
        "Emerald Ore Mined".to_string(),
    );
    map.insert(
        "minecraft:nether_quartz_ore".to_string(),
        "Nether Quartz Ore Mined".to_string(),
    );
    map.insert(
        "minecraft:ancient_debris".to_string(),
        "Ancient Debris Mined".to_string(),
    );

    // Wood types
    map.insert(
        "minecraft:oak_log".to_string(),
        "Oak Logs Mined".to_string(),
    );
    map.insert(
        "minecraft:spruce_log".to_string(),
        "Spruce Logs Mined".to_string(),
    );
    map.insert(
        "minecraft:birch_log".to_string(),
        "Birch Logs Mined".to_string(),
    );
    map.insert(
        "minecraft:jungle_log".to_string(),
        "Jungle Logs Mined".to_string(),
    );
    map.insert(
        "minecraft:acacia_log".to_string(),
        "Acacia Logs Mined".to_string(),
    );
    map.insert(
        "minecraft:dark_oak_log".to_string(),
        "Dark Oak Logs Mined".to_string(),
    );
    map.insert(
        "minecraft:crimson_stem".to_string(),
        "Crimson Stems Mined".to_string(),
    );
    map.insert(
        "minecraft:warped_stem".to_string(),
        "Warped Stems Mined".to_string(),
    );

    // Deepslate variants
    map.insert(
        "minecraft:deepslate".to_string(),
        "Deepslate Mined".to_string(),
    );
    map.insert(
        "minecraft:deepslate_coal_ore".to_string(),
        "Deepslate Coal Ore Mined".to_string(),
    );
    map.insert(
        "minecraft:deepslate_iron_ore".to_string(),
        "Deepslate Iron Ore Mined".to_string(),
    );
    map.insert(
        "minecraft:deepslate_gold_ore".to_string(),
        "Deepslate Gold Ore Mined".to_string(),
    );
    map.insert(
        "minecraft:deepslate_diamond_ore".to_string(),
        "Deepslate Diamond Ore Mined".to_string(),
    );
    map.insert(
        "minecraft:deepslate_redstone_ore".to_string(),
        "Deepslate Redstone Ore Mined".to_string(),
    );
    map.insert(
        "minecraft:deepslate_lapis_ore".to_string(),
        "Deepslate Lapis Ore Mined".to_string(),
    );
    map.insert(
        "minecraft:deepslate_emerald_ore".to_string(),
        "Deepslate Emerald Ore Mined".to_string(),
    );

    // This is a subset - more blocks could be added as needed
    map
}

/// Creates mapping for crafted item statistics
pub fn create_crafted_stat_names() -> HashMap<String, String> {
    let mut map = HashMap::new();

    // Tools
    map.insert(
        "minecraft:wooden_pickaxe".to_string(),
        "Wooden Pickaxes Crafted".to_string(),
    );
    map.insert(
        "minecraft:stone_pickaxe".to_string(),
        "Stone Pickaxes Crafted".to_string(),
    );
    map.insert(
        "minecraft:iron_pickaxe".to_string(),
        "Iron Pickaxes Crafted".to_string(),
    );
    map.insert(
        "minecraft:golden_pickaxe".to_string(),
        "Golden Pickaxes Crafted".to_string(),
    );
    map.insert(
        "minecraft:diamond_pickaxe".to_string(),
        "Diamond Pickaxes Crafted".to_string(),
    );
    map.insert(
        "minecraft:netherite_pickaxe".to_string(),
        "Netherite Pickaxes Crafted".to_string(),
    );

    // Swords
    map.insert(
        "minecraft:wooden_sword".to_string(),
        "Wooden Swords Crafted".to_string(),
    );
    map.insert(
        "minecraft:stone_sword".to_string(),
        "Stone Swords Crafted".to_string(),
    );
    map.insert(
        "minecraft:iron_sword".to_string(),
        "Iron Swords Crafted".to_string(),
    );
    map.insert(
        "minecraft:golden_sword".to_string(),
        "Golden Swords Crafted".to_string(),
    );
    map.insert(
        "minecraft:diamond_sword".to_string(),
        "Diamond Swords Crafted".to_string(),
    );
    map.insert(
        "minecraft:netherite_sword".to_string(),
        "Netherite Swords Crafted".to_string(),
    );

    // Armor
    map.insert(
        "minecraft:leather_helmet".to_string(),
        "Leather Helmets Crafted".to_string(),
    );
    map.insert(
        "minecraft:iron_helmet".to_string(),
        "Iron Helmets Crafted".to_string(),
    );
    map.insert(
        "minecraft:diamond_helmet".to_string(),
        "Diamond Helmets Crafted".to_string(),
    );
    map.insert(
        "minecraft:netherite_helmet".to_string(),
        "Netherite Helmets Crafted".to_string(),
    );

    // Building blocks
    map.insert(
        "minecraft:crafting_table".to_string(),
        "Crafting Tables Crafted".to_string(),
    );
    map.insert(
        "minecraft:furnace".to_string(),
        "Furnaces Crafted".to_string(),
    );
    map.insert("minecraft:chest".to_string(), "Chests Crafted".to_string());
    map.insert("minecraft:torch".to_string(), "Torches Crafted".to_string());

    // This is a subset - more items could be added as needed
    map
}

/// Creates mapping for used item statistics
pub fn create_used_stat_names() -> HashMap<String, String> {
    let mut map = HashMap::new();

    // Consumables
    map.insert("minecraft:apple".to_string(), "Apples Eaten".to_string());
    map.insert(
        "minecraft:mushroom_stew".to_string(),
        "Mushroom Stew Eaten".to_string(),
    );
    map.insert("minecraft:bread".to_string(), "Bread Eaten".to_string());
    map.insert(
        "minecraft:porkchop".to_string(),
        "Raw Porkchops Eaten".to_string(),
    );
    map.insert(
        "minecraft:cooked_porkchop".to_string(),
        "Cooked Porkchops Eaten".to_string(),
    );
    map.insert(
        "minecraft:golden_apple".to_string(),
        "Golden Apples Eaten".to_string(),
    );
    map.insert(
        "minecraft:enchanted_golden_apple".to_string(),
        "Enchanted Golden Apples Eaten".to_string(),
    );
    map.insert("minecraft:potion".to_string(), "Potions Drunk".to_string());

    // Tools
    map.insert(
        "minecraft:wooden_pickaxe".to_string(),
        "Wooden Pickaxes Used".to_string(),
    );
    map.insert(
        "minecraft:stone_pickaxe".to_string(),
        "Stone Pickaxes Used".to_string(),
    );
    map.insert(
        "minecraft:iron_pickaxe".to_string(),
        "Iron Pickaxes Used".to_string(),
    );
    map.insert(
        "minecraft:golden_pickaxe".to_string(),
        "Golden Pickaxes Used".to_string(),
    );
    map.insert(
        "minecraft:diamond_pickaxe".to_string(),
        "Diamond Pickaxes Used".to_string(),
    );
    map.insert(
        "minecraft:netherite_pickaxe".to_string(),
        "Netherite Pickaxes Used".to_string(),
    );

    // Buckets
    map.insert(
        "minecraft:water_bucket".to_string(),
        "Water Buckets Used".to_string(),
    );
    map.insert(
        "minecraft:lava_bucket".to_string(),
        "Lava Buckets Used".to_string(),
    );
    map.insert(
        "minecraft:milk_bucket".to_string(),
        "Milk Buckets Used".to_string(),
    );

    // Throwables
    map.insert(
        "minecraft:ender_pearl".to_string(),
        "Ender Pearls Thrown".to_string(),
    );
    map.insert(
        "minecraft:snowball".to_string(),
        "Snowballs Thrown".to_string(),
    );
    map.insert("minecraft:egg".to_string(), "Eggs Thrown".to_string());

    // This is a subset - more items could be added as needed
    map
}

/// Creates mapping for broken item statistics
pub fn create_broken_stat_names() -> HashMap<String, String> {
    let mut map = HashMap::new();

    // Tools
    map.insert(
        "minecraft:wooden_pickaxe".to_string(),
        "Wooden Pickaxes Broken".to_string(),
    );
    map.insert(
        "minecraft:stone_pickaxe".to_string(),
        "Stone Pickaxes Broken".to_string(),
    );
    map.insert(
        "minecraft:iron_pickaxe".to_string(),
        "Iron Pickaxes Broken".to_string(),
    );
    map.insert(
        "minecraft:golden_pickaxe".to_string(),
        "Golden Pickaxes Broken".to_string(),
    );
    map.insert(
        "minecraft:diamond_pickaxe".to_string(),
        "Diamond Pickaxes Broken".to_string(),
    );
    map.insert(
        "minecraft:netherite_pickaxe".to_string(),
        "Netherite Pickaxes Broken".to_string(),
    );

    // Swords
    map.insert(
        "minecraft:wooden_sword".to_string(),
        "Wooden Swords Broken".to_string(),
    );
    map.insert(
        "minecraft:stone_sword".to_string(),
        "Stone Swords Broken".to_string(),
    );
    map.insert(
        "minecraft:iron_sword".to_string(),
        "Iron Swords Broken".to_string(),
    );
    map.insert(
        "minecraft:golden_sword".to_string(),
        "Golden Swords Broken".to_string(),
    );
    map.insert(
        "minecraft:diamond_sword".to_string(),
        "Diamond Swords Broken".to_string(),
    );
    map.insert(
        "minecraft:netherite_sword".to_string(),
        "Netherite Swords Broken".to_string(),
    );

    // Armor
    map.insert(
        "minecraft:leather_helmet".to_string(),
        "Leather Helmets Broken".to_string(),
    );
    map.insert(
        "minecraft:iron_helmet".to_string(),
        "Iron Helmets Broken".to_string(),
    );
    map.insert(
        "minecraft:diamond_helmet".to_string(),
        "Diamond Helmets Broken".to_string(),
    );
    map.insert(
        "minecraft:netherite_helmet".to_string(),
        "Netherite Helmets Broken".to_string(),
    );

    // This is a subset - more items could be added as needed
    map
}

/// Creates mapping for picked up item statistics
pub fn create_picked_up_stat_names() -> HashMap<String, String> {
    let mut map = HashMap::new();

    // Resources
    map.insert(
        "minecraft:diamond".to_string(),
        "Diamonds Picked Up".to_string(),
    );
    map.insert(
        "minecraft:iron_ingot".to_string(),
        "Iron Ingots Picked Up".to_string(),
    );
    map.insert(
        "minecraft:gold_ingot".to_string(),
        "Gold Ingots Picked Up".to_string(),
    );
    map.insert(
        "minecraft:netherite_ingot".to_string(),
        "Netherite Ingots Picked Up".to_string(),
    );
    map.insert(
        "minecraft:emerald".to_string(),
        "Emeralds Picked Up".to_string(),
    );
    map.insert("minecraft:coal".to_string(), "Coal Picked Up".to_string());
    map.insert(
        "minecraft:lapis_lazuli".to_string(),
        "Lapis Lazuli Picked Up".to_string(),
    );
    map.insert(
        "minecraft:redstone".to_string(),
        "Redstone Picked Up".to_string(),
    );
    map.insert(
        "minecraft:quartz".to_string(),
        "Nether Quartz Picked Up".to_string(),
    );

    // Items
    map.insert(
        "minecraft:apple".to_string(),
        "Apples Picked Up".to_string(),
    );
    map.insert(
        "minecraft:arrow".to_string(),
        "Arrows Picked Up".to_string(),
    );
    map.insert("minecraft:bone".to_string(), "Bones Picked Up".to_string());
    map.insert(
        "minecraft:string".to_string(),
        "String Picked Up".to_string(),
    );
    map.insert(
        "minecraft:feather".to_string(),
        "Feathers Picked Up".to_string(),
    );

    // This is a subset - more items could be added as needed
    map
}

/// Creates mapping for dropped item statistics
pub fn create_dropped_stat_names() -> HashMap<String, String> {
    let mut map = HashMap::new();

    // Resources
    map.insert(
        "minecraft:diamond".to_string(),
        "Diamonds Dropped".to_string(),
    );
    map.insert(
        "minecraft:iron_ingot".to_string(),
        "Iron Ingots Dropped".to_string(),
    );
    map.insert(
        "minecraft:gold_ingot".to_string(),
        "Gold Ingots Dropped".to_string(),
    );
    map.insert(
        "minecraft:netherite_ingot".to_string(),
        "Netherite Ingots Dropped".to_string(),
    );
    map.insert(
        "minecraft:emerald".to_string(),
        "Emeralds Dropped".to_string(),
    );
    map.insert("minecraft:coal".to_string(), "Coal Dropped".to_string());
    map.insert(
        "minecraft:lapis_lazuli".to_string(),
        "Lapis Lazuli Dropped".to_string(),
    );
    map.insert(
        "minecraft:redstone".to_string(),
        "Redstone Dropped".to_string(),
    );
    map.insert(
        "minecraft:quartz".to_string(),
        "Nether Quartz Dropped".to_string(),
    );

    // Items
    map.insert("minecraft:apple".to_string(), "Apples Dropped".to_string());
    map.insert("minecraft:arrow".to_string(), "Arrows Dropped".to_string());
    map.insert("minecraft:bone".to_string(), "Bones Dropped".to_string());
    map.insert("minecraft:string".to_string(), "String Dropped".to_string());
    map.insert(
        "minecraft:feather".to_string(),
        "Feathers Dropped".to_string(),
    );

    // This is a subset - more items could be added as needed
    map
}

/// Creates mapping for killed entity statistics
pub fn create_killed_stat_names() -> HashMap<String, String> {
    let mut map = HashMap::new();

    // Hostile mobs
    map.insert("minecraft:zombie".to_string(), "Zombies Killed".to_string());
    map.insert(
        "minecraft:skeleton".to_string(),
        "Skeletons Killed".to_string(),
    );
    map.insert(
        "minecraft:creeper".to_string(),
        "Creepers Killed".to_string(),
    );
    map.insert("minecraft:spider".to_string(), "Spiders Killed".to_string());
    map.insert(
        "minecraft:enderman".to_string(),
        "Endermen Killed".to_string(),
    );
    map.insert("minecraft:witch".to_string(), "Witches Killed".to_string());
    map.insert("minecraft:slime".to_string(), "Slimes Killed".to_string());
    map.insert(
        "minecraft:phantom".to_string(),
        "Phantoms Killed".to_string(),
    );
    map.insert(
        "minecraft:drowned".to_string(),
        "Drowned Killed".to_string(),
    );
    map.insert("minecraft:blaze".to_string(), "Blazes Killed".to_string());
    map.insert("minecraft:ghast".to_string(), "Ghasts Killed".to_string());
    map.insert(
        "minecraft:magma_cube".to_string(),
        "Magma Cubes Killed".to_string(),
    );
    map.insert(
        "minecraft:wither_skeleton".to_string(),
        "Wither Skeletons Killed".to_string(),
    );
    map.insert("minecraft:piglin".to_string(), "Piglins Killed".to_string());
    map.insert(
        "minecraft:piglin_brute".to_string(),
        "Piglin Brutes Killed".to_string(),
    );
    map.insert("minecraft:hoglin".to_string(), "Hoglins Killed".to_string());
    map.insert("minecraft:zoglin".to_string(), "Zoglins Killed".to_string());

    // Passive mobs
    map.insert("minecraft:pig".to_string(), "Pigs Killed".to_string());
    map.insert("minecraft:cow".to_string(), "Cows Killed".to_string());
    map.insert("minecraft:sheep".to_string(), "Sheep Killed".to_string());
    map.insert(
        "minecraft:chicken".to_string(),
        "Chickens Killed".to_string(),
    );
    map.insert("minecraft:rabbit".to_string(), "Rabbits Killed".to_string());
    map.insert("minecraft:squid".to_string(), "Squids Killed".to_string());
    map.insert(
        "minecraft:dolphin".to_string(),
        "Dolphins Killed".to_string(),
    );
    map.insert("minecraft:turtle".to_string(), "Turtles Killed".to_string());
    map.insert("minecraft:bat".to_string(), "Bats Killed".to_string());
    map.insert("minecraft:fox".to_string(), "Foxes Killed".to_string());
    map.insert("minecraft:ocelot".to_string(), "Ocelots Killed".to_string());
    map.insert(
        "minecraft:strider".to_string(),
        "Striders Killed".to_string(),
    );

    // Bosses
    map.insert(
        "minecraft:ender_dragon".to_string(),
        "Ender Dragons Killed".to_string(),
    );
    map.insert("minecraft:wither".to_string(), "Withers Killed".to_string());
    map.insert(
        "minecraft:elder_guardian".to_string(),
        "Elder Guardians Killed".to_string(),
    );

    // Special
    map.insert("minecraft:player".to_string(), "Players Killed".to_string());
    map.insert(
        "minecraft:villager".to_string(),
        "Villagers Killed".to_string(),
    );

    map
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserCache {
    pub name: String,
    pub uuid: String,
    #[serde(rename = "expiresOn")]
    expires_on: String,
}

pub trait ToJson<T> {
    fn to_json(&self) -> Result<T, serde_json::Error>;
}

impl<T> ToJson<T> for String
where
    T: for<'a> Deserialize<'a>,
{
    fn to_json(&self) -> Result<T, serde_json::Error> {
        serde_json::from_str(self)
    }
}

pub fn fmt_time<T: Into<u64>>(time: T) -> String {
    let time: u64 = time.into();
    let seconds = time / 20;
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

pub trait Findable {
    fn find_player(&self, stats: Vec<UserCache>) -> String;
}

impl Findable for String {
    fn find_player(&self, stats: Vec<UserCache>) -> String {
        for player in &stats {
            if player.uuid == *self {
                return player.name.clone();
            }
        }
        "Unknown".to_string()
    }
}

