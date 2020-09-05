mod errors;
mod utils;
mod wikia;

use std::error::Error;
use argh::FromArgs;
use wikia::{Game, PersonaTitle};

#[derive(FromArgs)]
/// Find shadow resistance/weakness information
struct Opts {
    /// name of shadow
    #[argh(option, short = 's')]
    shadow: String,

    /// persona series number.
    /// One of: 3, 3j, 3a, 4, 4g
    #[argh(option, short = 'p')]
    persona: String,

    /// enemy variant.
    /// Defaults to 'normal', can be 'normal' or 'sub'
    #[argh(option, short = 'v', default = "String::from(\"The Journey\")")]
    variant: String
}

fn determine_game(game: &String) -> Game {
    match game.to_lowercase().as_str() {
        "3" => Game {
            entry: PersonaTitle::P3,
            tab_name: "Persona 3".to_string(),
            variant: Some("Normal Encounter".to_string())
        },
        "3j" => Game {
            entry: PersonaTitle::P3J,
            tab_name: "The Journey".to_string(),
            variant: Some("Normal Encounter".to_string())
        },
        "3a" => Game {
            entry: PersonaTitle::P3A,
            tab_name: "The Answer".to_string(),
            variant: None
        },
        "4" => Game {
            entry: PersonaTitle::P4,
            tab_name: "Persona 4".to_string(),
            variant: None
        },
        "4g" => Game {
            entry: PersonaTitle::P4G,
            tab_name: "Golden".to_string(),
            variant: None
        },
        _ => Game {
            entry: PersonaTitle::P3J,
            tab_name: "The Journey".to_string(),
            variant: Some("Normal Encounter".to_string())
        },
    }
}

fn normalize_variant(variant: &str, game: Game) -> Game {
    match variant {
        "normal" => Game {
            entry: game.entry.clone(),
            tab_name: game.tab_name.clone(),
            variant: Some("Normal Encounter".to_string())
        },
        "sub" => Game {
            entry: game.entry.clone(),
            tab_name: game.tab_name.clone(),
            variant: Some("Sub-boss".to_string())
        },
        _ => Game {
            entry: game.entry.clone(),
            tab_name: game.tab_name.clone(),
            variant: Some("Normal Encounter".to_string())
        }
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let opts: Opts = argh::from_env();

    let game = determine_game(&opts.persona);

    let page_id = match wikia::get_shadow_page_id(&opts.shadow) {
        Ok(id) => { id },
        Err(e) => panic!(e.to_string())
    };

    if page_id == -1 {
        eprintln!("Shadow not found.");
        return Err(errors::NoShadowError.into());
    }

    let page = match wikia::page_html(&page_id) {
        Ok(p) => { p },
        Err(e) => { panic!(e.to_string()) }
    };

    let subsection = match wikia::game_section(&page, &game) {
        Ok(s) => { s },
        Err(e) => { panic!(e.to_string()) }
    };

    let game_normalized_variant = normalize_variant(&opts.variant, game);

    let table_node = match wikia::game_table(&subsection, &game_normalized_variant) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e.to_string());
            std::process::exit(1);
        }
    };

    let table_data = match wikia::extract_table_data(table_node) {
        Ok(t) => { t },
        Err(_) => {
            eprintln!("found table, but failed to parse it. Maybe HTML changed?");
            std::process::exit(1);
        }
    };

    utils::print_resistances(&table_data);

    Ok(())
}
