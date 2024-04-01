use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::fs;
use titlecase::titlecase;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The input json file
    #[arg(required = true)]
    file: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ValueData {
    #[serde(rename(deserialize = "Value"))]
    value: u64,
    #[serde(rename(deserialize = "Notes"))]
    notes: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct NameModifierData {
    #[serde(rename(deserialize = "Name"))]
    name: String,
    #[serde(rename(deserialize = "Modifier"))]
    modifier: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct NameContentData {
    #[serde(rename(deserialize = "Name"))]
    name: String,
    #[serde(rename(deserialize = "Content"))]
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AbilitiesData {
    #[serde(rename(deserialize = "Str"))]
    str: u64,
    #[serde(rename(deserialize = "Dex"))]
    dex: u64,
    #[serde(rename(deserialize = "Con"))]
    con: u64,
    #[serde(rename(deserialize = "Int"))]
    int: u64,
    #[serde(rename(deserialize = "Wis"))]
    wis: u64,
    #[serde(rename(deserialize = "Cha"))]
    cha: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct ImprovedInitiativeData {
    #[serde(rename(deserialize = "Source"))]
    source: String,
    #[serde(rename(deserialize = "Description"))]
    description: String,
    #[serde(rename(deserialize = "Player"))]
    player: String,
    #[serde(rename(deserialize = "Version"))]
    version: String,
    #[serde(rename(deserialize = "ImageURL"))]
    image_url: String,
    #[serde(rename(deserialize = "Name"))]
    name: String,
    #[serde(rename(deserialize = "Type"))]
    r#type: String,
    #[serde(rename(deserialize = "AC"))]
    ac: ValueData,
    #[serde(rename(deserialize = "HP"))]
    hp: ValueData,
    #[serde(rename(deserialize = "Speed"))]
    speed: Vec<String>,
    #[serde(rename(deserialize = "Abilities"))]
    abilities: AbilitiesData,
    #[serde(rename(deserialize = "Saves"))]
    saves: Vec<NameModifierData>,
    #[serde(rename(deserialize = "Skills"))]
    skills: Vec<NameModifierData>,
    #[serde(rename(deserialize = "DamageVulnerabilities"))]
    damage_vulnerabilities: Vec<String>,
    #[serde(rename(deserialize = "DamageResistances"))]
    damage_resistances: Vec<String>,
    #[serde(rename(deserialize = "DamageImmunities"))]
    damage_immunities: Vec<String>,
    #[serde(rename(deserialize = "ConditionImmunities"))]
    condition_immunities: Vec<String>,
    #[serde(rename(deserialize = "Senses"))]
    senses: Vec<String>,
    #[serde(rename(deserialize = "Languages"))]
    languages: Vec<String>,
    #[serde(rename(deserialize = "Challenge"))]
    challenge: String,
    #[serde(rename(deserialize = "Traits"))]
    traits: Vec<NameContentData>,
    #[serde(rename(deserialize = "Actions"))]
    actions: Vec<NameContentData>,
    #[serde(rename(deserialize = "BonusActions"))]
    bonus_actions: Vec<NameContentData>,
    #[serde(rename(deserialize = "Reactions"))]
    reactions: Vec<NameContentData>,
    #[serde(rename(deserialize = "LegendaryActions"))]
    legendary_actions: Vec<NameContentData>,
    #[serde(rename(deserialize = "MythicActions"))]
    mythic_actions: Vec<NameContentData>,
}

fn main() {
    let env = env_logger::Env::default()
        .filter_or("MSTSB_LOG_LEVEL", "info")
        .write_style_or("MSTSB_LOG_STYLE", "always");

    env_logger::Builder::from_env(env)
        .format_module_path(false)
        .format_target(false)
        .format_indent(None)
        .init();

    let args = Args::parse();

    let cwd = std::env::current_dir().expect("Error getting current working directory");

    let json_content: String =
        fs::read_to_string(cwd.join(args.file)).expect("Failed to load JSON file");
    let data: ImprovedInitiativeData =
        serde_json::from_str(json_content.as_str()).expect("Error parsing configuration file");

    println!("```statblock");
    println!("name: {}", data.name);
    let re = regex::Regex::new(r"^(\w+) (\w+) \(([\w ]+)\), ([\w ]+)$").unwrap();
    if let Some(caps) = re.captures(&data.r#type) {
        println!("size: {}", titlecase(&caps[1]));
        println!("type: {}", titlecase(&caps[2]));
        println!("subtype: {}", titlecase(&caps[3]));
        println!("alignment: {}", titlecase(&caps[4]));
    };
    println!("columns: 2");
    println!("ac: {}", data.ac.value);
    println!("hp: {}", data.hp.value);
    println!(
        "hit_dice: {}",
        data.hp
            .notes
            .strip_prefix("(")
            .unwrap_or(data.hp.notes.as_str())
            .strip_suffix(")")
            .unwrap_or(data.hp.notes.as_str())
    );
    println!("speed: {}", data.speed.get(0).unwrap_or(&String::from("")));
    println!(
        "stats: [{str}, {dex}, {con}, {int}, {wis}, {cha}]",
        str = data.abilities.str,
        dex = data.abilities.dex,
        con = data.abilities.con,
        int = data.abilities.int,
        wis = data.abilities.wis,
        cha = data.abilities.cha
    );
    if data.saves.len() > 0 {
        println!("saves:");
        for save in data.saves {
            println!(r#"  - {}: {}"#, save.name.to_lowercase(), save.modifier);
        }
    }
    if data.skills.len() > 0 {
        println!("skillsaves:");
        for save in data.skills {
            println!(r#"  - {}: {}"#, save.name.to_lowercase(), save.modifier);
        }
    }
    if data.damage_vulnerabilities.len() > 0 {
        println!(
            "damage_vulnerabilities: {}",
            data.damage_vulnerabilities.join(", ")
        );
    }
    if data.damage_resistances.len() > 0 {
        println!("damage_resistances: {}", data.damage_resistances.join(", "));
    }
    if data.damage_immunities.len() > 0 {
        println!("damage_immunities: {}", data.damage_immunities.join(", "));
    }
    if data.condition_immunities.len() > 0 {
        println!(
            "condition_immunities: {}",
            data.condition_immunities.join(", ")
        );
    }
    if data.senses.len() > 0 {
        println!("senses: {}", titlecase(&data.senses.join(", ")));
    }
    if data.languages.len() > 0 {
        println!("languages: {}", data.languages.join(", "));
    }
    println!(r#"cr: "{}""#, data.challenge);
    if data.traits.len() > 0 {
        println!("traits:");
        for r#trait in data.traits {
            println!(r#"  - name: "{}""#, r#trait.name);
            println!(r#"    desc: "{}""#, r#trait.content.replace("\n", "\\n"));
        }
    }
    if data.actions.len() > 0 {
        println!("actions:");
        for action in data.actions {
            println!(r#"  - name: "{}""#, action.name);
            println!(r#"    desc: "{}""#, action.content.replace("\n", "\\n"));
        }
    }
    if data.legendary_actions.len() > 0 {
        println!("legendary_actions:");
        for action in data.legendary_actions {
            println!(r#"  - name: "{}""#, action.name);
            println!(r#"    desc: "{}""#, action.content.replace("\n", "\\n"));
        }
    }
    if data.reactions.len() > 0 {
        println!("reactions:");
        for action in data.reactions {
            println!(r#"  - name: "{}""#, action.name);
            println!(r#"    desc: "{}""#, action.content.replace("\n", "\\n"));
        }
    }
    if data.mythic_actions.len() > 0 {
        println!("mythic_actions:");
        for action in data.mythic_actions {
            println!(r#"  - name: "{}""#, action.name);
            println!(r#"    desc: "{}""#, action.content.replace("\n", "\\n"));
        }
    }
    println!("```");
}
