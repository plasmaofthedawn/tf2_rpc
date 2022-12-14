use std::fmt;
use std::fmt::{Display, Formatter};
use a2s::info::Info;
use discord_rpc_client::Client;
use lazy_static::lazy_static;
use std::sync::Mutex;
use discord_rpc_client::models::Activity;
use discord_rpc_client::models::payload::Payload;

lazy_static! {
    static ref DRPC: Mutex<Client> = {
        let mut drpc = Client::new(451950787996680192);
        drpc.start();
        Mutex::new(drpc)
    };
}

//TODO: is this here really neccessary
// discord_rpc_client::error::Result is private so i kinda hve to make my own and it's annoying
// error code is here https://gitlab.com/valeth/discord-rpc-client.rs/-/blob/master/src/error.rs

type Result<T> = std::result::Result<T, ActivityError>;
pub struct ActivityError;
impl Display for ActivityError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("Error doing an activity.")
    }
}

pub fn set_activity_playing(map: &str, name: &str, players: u8, max_players: u8) -> Result<Payload<Activity>> {
      let t = DRPC.lock().unwrap().set_activity(|act| act
        .state(format!("{} ({}/{})", map, players, max_players))
        .details(name)
        .assets(|ass| ass
            .large_image(map_to_image_name(map))
            .large_text(map)
        )
     );

    if t.is_ok() {
        Ok(t.unwrap())
    } else {
        Err(ActivityError)
    }

}

pub fn set_activity_playing_from_info(info: &Info) -> Result<Payload<Activity>> {
    set_activity_playing(&info.map, &info.name, info.players, info.max_players)
}

pub fn set_activity_menu () -> Result<Payload<Activity>>{
    let t = DRPC.lock().unwrap().set_activity(|act| act
        .details("Main Menu")
        .assets(|ass| ass
            .large_image("mainmenu")
            .large_text("Main Menu")
            .small_image("tf2button")
            .small_text("TF2_RPC by plasmaofthedawn#1435 and viruz#9907")
        )
    );

    if t.is_ok() {
        Ok(t.unwrap())
    } else {
        Err(ActivityError)
    }
}

fn map_to_image_name(map_name: &str) -> &'static str {
    match map_name {
        "surf_" => "surf",
        "jump_" => "jump",
        "bhop_" => "bhop",
        "cp_orange_" => "orange",
        "mge_" => "mge",
        "trade_" => "trade",
        "achievement_" => "achievementidle",
        "pl_vigil" => "vigil",
        "koth_cascade" => "cascade",
        "cp_cardinal" => "cardinal",
        "koth_ramjam" => "ramjam",
        "koth_ashville" => "ashville",
        "koth_coalplant" => "coalplant",
        "koth_airfield" => "airfield",
        "cp_warmfrost" => "warmfrost",
        "ctf_2fort" => "2fort",
        "ctf_2fort_invasion" => "2fortinvasion",
        "cp_5gorge" => "5gorge",
        "rd_asteroid" => "asteroid",
        "cp_badlands" => "badlands",
        "cp_prolands" => "badlands",
        "koth_badlands" => "badlands",
        "arena_badlands" => "badlands",
        "pl_badwater" => "badwater",
        "plr_bananabay" => "bananabay",
        "pl_barnblitz" => "barnblitz",
        "mvm_bigrock" => "bigrock",
        "pl_borneo" => "borneo",
        "koth_brazil" => "brazil",
        "pass_brickyard" => "brickyard",
        "pl_fifthcurve_event" => "brimstone",
        "arena_byre" => "byre",
        "pl_cactuscanyon" => "cactuscanyon",
        "sd_doomsday_event" => "carnage",
        "cp_cloak" => "cloak",
        "mvm_coaltown" => "coaltown",
        "cp_coldfront" => "coldfront",
        "mvm_decoy" => "decoy",
        "cp_degrootkeep" => "degrootkeep",
        "pass_district" => "district",
        "sd_doomsday" => "doomsday",
        "ctf_doublecross" => "doublecross",
        "cp_dustbowl" => "dustbowl",
        "tr_dustbowl" => "dustbowl",
        "cp_egypt_final" => "egypt",
        "pl_enclosure_final" => "enclosure",
        "mvm_example" => "example",
        "cp_fastlane" => "fastlane",
        "cp_foundry" => "foundry",
        "ctf_foundry" => "foundry",
        "cp_freight_final1" => "freight",
        "pl_frontier_final" => "frontier",
        "mvm_ghost_town" => "ghosttown",
        "pl_goldrush" => "goldrush",
        "cp_gorge" => "gorge",
        "ctf_gorge" => "gorge",
        "cp_gorge_event" => "gorgeevent",
        "cp_granary" => "granary",
        "arena_granary" => "granary",
        "cp_gravelpit" => "gravelpit",
        "cp_gullywash_final1" => "gullywash",
        "koth_harvest_final" => "harvest",
        "koth_harvest_event" => "harvestevent",
        "ctf_hellfire" => "hellfire",
        "koth_highpass" => "highpass",
        "plr_hightower" => "hightower",
        "plr_hightower_event" => "hightowerevent",
        "pl_hoodoo_final" => "hoodoo",
        "tc_hydro" => "hydro",
        "itemtest" => "itemtest",
        "cp_junction_final" => "junction",
        "koth_king" => "kongking",
        "koth_lakeside_final" => "lakeside",
        "koth_lakeside_event" => "lakesideevent",
        "ctf_landfall" => "landfall",
        "koth_lazarus" => "lazarus",
        "arena_lumberyard" => "lumberyard",
        "mvm_mannhattan" => "mannhattan",
        "cp_manor_event" => "mannmanor",
        "mvm_mannworks" => "mannworks",
        "koth_maple_ridge" => "mapleridge",
        "cp_mercenarypark" => "mercpark",
        "cp_metalworks" => "metalworks",
        "pl_millstone_event" => "millstoneevent",
        "koth_moonshine_event" => "moonshine",
        "cp_mossrock" => "mossrock",
        "cp_mountainlab" => "mountainlab",
        "plr_nightfall_final" => "nightfall",
        "koth_nucleus" => "nucleus",
        "arena_nucleus" => "nucleus",
        "arena_offblast_final" => "offblast",
        "plr_pipeline" => "pipeline",
        "pd_pit_of_death_event" => "pitofdeath",
        "cp_powerhouse" => "powerhouse",
        "koth_probed" => "probed",
        "cp_process_final" => "process",
        "koth_product" => "product",
        "arena_ravine" => "ravine",
        "mvm_rottenburg" => "rottenburg",
        "koth_sawmill" => "sawmill",
        "arena_sawmill" => "sawmill",
        "ctf_sawmill" => "sawmill",
        "cp_sunshine_event" => "sinshine",
        "cp_snakewater" => "snakewater",
        "cp_snowplow" => "snowplow",
        "pl_snowycoast" => "snowycoast",
        "cp_standin" => "standin",
        "cp_steel" => "steel",
        "koth_suijin" => "suijin",
        "cp_sunshine" => "sunshine",
        "pl_swiftwater" => "swiftwater",
        "tr_target" => "target",
        "pl_thundermountain" => "thundermountain",
        "pass_timberlodge" => "timberlodge",
        "ctf_turbine" => "turbine",
        "pl_upward" => "upward",
        "cp_vanguard" => "vanguard",
        "koth_viaduct" => "viaduct",
        "koth_viaduct_event" => "viaductevent",
        "arena_watchtower" => "watchtower",
        "pd_watergate" => "watergate",
        "cp_well" => "well",
        "ctf_well" => "well",
        "arena_well" => "well",
        "cp_yukon" => "yukon",
        "cp_glassworks" => "glassworks",
        "cp_kalinka" => "kalinka",
        "cp_rumble" => "rumble",
        "cp_stoneyridge" => "stoneyridge",
        "koth_ashville_rc2" => "ashvillenight",
        "koth_ashville_rc1_nb" => "ashvillenight",
        "koth_clearcut" => "clearcut",
        "koth_databank" => "databank",
        "koth_gibson" => "gibson",
        "koth_harter" => "harter",
        "koth_harvestalpine" => "harvestalpine",
        "koth_slaughter" => "slaughter",
        "koth_spillway" => "spillway",
        "koth_synthetic" => "synthetic",
        "pl_camber" => "camber",
        "pl_cashworks" => "cashworks",
        "pl_fifthcurve_rc" => "fifthcurve",
        "pl_fifthcurve_b" => "fifthcurve",
        "pl_sludgepit" => "sludgepit",
        "pl_stranded" => "stranded",
        "pl_summercoast" => "summercoast",
        "koth_bagel" => "bagelfall",
        _ => "unknown"
    }
}