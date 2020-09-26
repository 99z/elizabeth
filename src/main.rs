mod errors;
mod utils;
mod wikia;

use argh::FromArgs;
use wikia::{Game};

#[derive(FromArgs)]
/// Find shadow resistance/weakness information
struct Opts {
    /// name of shadow
    #[argh(option, short = 's')]
    shadow: String,

    /// persona series number.
    /// One of: 3, 3a, 4, 4g
    #[argh(option, short = 'p')]
    persona: String,

    /// shadow variant.
    /// Defaults to 'normal', can be 'normal' or 'sub'
    #[argh(option, short = 'v', default = "String::from(\"The Journey\")")]
    variant: String,

    /// get all shadow resistance info for specified game.
    /// Defaults to false
    #[argh(switch, short = 'a')]
    all: bool
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

    let mut game = utils::determine_game(&opts.persona.as_str());
    normalize_variant(&opts.variant, &mut game);

    let page_id = wikia::get_shadow_page_id(&opts.shadow)?;

    if page_id == -1 {
        return Err(errors::NoShadowError.into());
    }

    let page = wikia::page_html(&page_id)?;

    if opts.all {
        wikia::arcana_sections(&page, &game)?;
    } else {
        let appears_in = wikia::appears_in(&page, &game)?;
        if !appears_in {
            return Err(errors::NoShadowError.into());
        }

        let subsection = wikia::game_section(&page, &game)?;

        let table_node = wikia::game_table(&subsection, &game)?;

        let table_data = wikia::extract_table_data(&table_node)?;

        utils::print_resistances(&table_data);
    }

    Ok(())
}
