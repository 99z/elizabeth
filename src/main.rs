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
        "3" => Game {
            entry: PersonaTitle::P3,
            tab_name: "Persona 3",
            variant: Some("Normal Encounter")
        },
        "3j" => Game {
            entry: PersonaTitle::P3J,
            tab_name: "The Journey",
            variant: Some("Normal Encounter")
        },
        "3a" => Game {
            entry: PersonaTitle::P3A,
            tab_name: "The Answer",
            variant: None
        },
        "4" => Game {
            entry: PersonaTitle::P4,
            tab_name: "Persona 4",
            variant: None
        },
        "4g" => Game {
            entry: PersonaTitle::P4G,
            tab_name: "Golden",
            variant: None
        },
        _ => Game {
            entry: PersonaTitle::P3J,
            tab_name: "The Journey",
            variant: Some("Normal Encounter")
        },
    }
}

fn normalize_variant(variant: &str, game: &mut Game) {
    match variant {
        "sub" => {
            game.variant = Some("Sub-boss")
        },
        "normal" | _ => {
            game.variant = Some("Normal Encounter")
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

    let subsection = wikia::game_section(&page, &game)?;

    let table_node = wikia::game_table(&subsection, &game)?;

    let table_data = wikia::extract_table_data(&table_node)?;

    utils::print_resistances(&table_data);

    Ok(())
}
