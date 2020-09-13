mod errors;
mod utils;
mod wikia;

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
        "3" | "3j" => Game {
            entry: PersonaTitle::P3J,
            entry_text: "Persona 3 FES",
            tab_names: vec!["The Journey".to_string(), "Persona 3".to_string()],
            variant: Some("Normal")
        },
        "3a" => Game {
            entry: PersonaTitle::P3A,
            entry_text: "Persona 3 FES",
            tab_names: vec!["The Answer".to_string()],
            variant: None
        },
        "4" => Game {
            entry: PersonaTitle::P4,
            entry_text: "Persona 4",
            tab_names: vec!["Persona 4".to_string()],
            variant: None
        },
        "4g" => Game {
            entry: PersonaTitle::P4G,
            entry_text: "Persona 4 Golden",
            tab_names: vec!["Golden".to_string()],
            variant: None
        },
        _ => Game {
            entry: PersonaTitle::P3J,
            entry_text: "Persona 3 FES",
            tab_names: vec!["The Journey".to_string()],
            variant: Some("Normal")
        },
    }
}

fn normalize_variant(variant: &str, game: &mut Game) {
    match variant {
        "sub" => {
            game.variant = Some("Sub")
        },
        "normal" | _ => {
            game.variant = Some("Normal")
        }
    }
}

fn main() -> anyhow::Result<()> {
    let opts: Opts = argh::from_env();

    let mut game = determine_game(&opts.persona);
    normalize_variant(&opts.variant, &mut game);

    let page_id = wikia::get_shadow_page_id(&opts.shadow)?;

    if page_id == -1 {
        return Err(errors::NoShadowError.into());
    }

    let page = wikia::page_html(&page_id)?;

    let appears_in = wikia::appears_in(&page, &game)?;
    if !appears_in {
        return Err(errors::NoShadowError.into());
    }

    let subsection = wikia::game_section(&page, &game)?;

    let table_node = wikia::game_table(&subsection, &game)?;

    let table_data = wikia::extract_table_data(&table_node)?;

    utils::print_resistances(&table_data);

    Ok(())
}
