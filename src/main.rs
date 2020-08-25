use std::collections::HashMap;
use serde::{Deserialize};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use titlecase::titlecase;
use core::fmt;
use scraper::{Selector, Html};
use std::error::Error;
use colored::*;
use argh::FromArgs;

#[derive(Debug, Clone)]
struct PageParseError;

#[derive(Debug, Clone)]
struct NoShadowError;

impl fmt::Display for PageParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "table data not found")
    }
}

impl std::error::Error for PageParseError {}

impl fmt::Display for NoShadowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no shadow specified or no matching variant found")
    }
}

impl std::error::Error for NoShadowError {}

#[derive(Deserialize, Debug)]
struct PageMeta {
    query: Query
}

#[derive(Deserialize, Debug)]
struct Query {
    pageids: Vec<String>
}

#[derive(Deserialize, Debug)]
struct Page {
    content: String
}

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

#[derive(Debug, Clone)]
struct Game {
    entry: Games,
    tab_name: String,
    variant: Option<String>
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Games {
    P3,
    P3J,
    P3A,
    P4,
    P4G
}

const P3_SELECTOR: &str = "[id^=Persona_3]";
const P4_SELECTOR: &str = "[id^=Persona_4]";

fn gen_table_selector(index: &usize) -> Selector {
    let sel_string = format!(
        "div:nth-child({}) > table > tbody > tr > td > table:nth-child(1) >
        tbody > tr > td > table:nth-child(2)",
        index + 1
    );
    Selector::parse(sel_string.as_str()).unwrap()
}

fn get_page(page_id: &isize) -> Result<Html, Box<dyn Error>> {
    let page_endpoint = format!(
        "https://megamitensei.fandom.com/api/v1/Articles/AsJson?id={}",
        page_id
    );
    let body: Page = reqwest::blocking::get(&page_endpoint)?.json()?;
    // println!("{}", body.content);

    let document = Html::parse_fragment(body.content.as_str());
    Ok(document)
}

fn get_table_node(doc: &Html, game: &Game) -> Result<Html, Box<dyn Error>> {
    let tabs = Selector::parse(".tabbertab").unwrap();
    let tab_selector = match doc
        .select(&tabs)
        .position(|t| t.value().attr("title")
        .unwrap() == game.variant.as_ref().unwrap()
    ) {
        Some(idx) => gen_table_selector(&idx),
        None => {
            match doc
                .select(&tabs)
                .position(|t| t.value().attr("title")
                .unwrap() == game.tab_name
            ) {
                Some(idx) => gen_table_selector(&idx),
                None => Selector::parse("table > tbody > tr > td > table:nth-child(1) >\
                tbody > tr > td > table:nth-child(2)").unwrap()
            }
        }
    };

    let table_node = doc.select(&tab_selector);
    let resistance_table = Html::parse_fragment(table_node.map(|n| n.html())
        .collect::<String>().as_str());
    Ok(resistance_table)
}

fn get_game_section(page: &Html, game: &Game) -> Result<Html, Box<dyn Error>> {
    let persona_selector = if game.entry == Games::P3 || game.entry == Games::P3J || game.entry == Games::P3A {
        P3_SELECTOR
    } else {
        P4_SELECTOR
    };

    let base_selector = Selector::parse(
        format!("{} + .tabber", persona_selector).as_str()
    ).unwrap();
    let mut subsection_sel = page.select(&base_selector);

    let mut size = 0;
    for _ in subsection_sel {
        size += 1;
    }

    let selector = if size == 0 {
        // case when no table tabs are present, probably when shadow was only in base 3/4
        Selector::parse(format!("{} + table", persona_selector).as_str()).unwrap()
    } else {
        base_selector.clone()
    };

    subsection_sel =  page.select(&selector);
    let subsection = Html::parse_fragment(subsection_sel.map(|n| n.html())
        .collect::<String>().as_str());

    Ok(subsection)
}

// Necessary to handle the seemingly random cases when resistance text
// is surrounded by some html tag, e.g. <span>Weak</span>
fn strip_html(cell: String) -> String {
    if cell.contains("Weak") {
       return String::from("Weak");
    } else if cell.contains("Strong") {
        return String::from("Strong");
    } else if cell.contains("Repel") {
        return String::from("Repel");
    } else if cell.contains("Null") {
        return String::from("Null");
    }

    String::from("Neutral")
}

fn extract_table_data(table_doc: Html) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let types = Selector::parse("tbody > tr:nth-child(1) > th").unwrap();
    let types_table: Vec<String> = table_doc.select(&types)
        .map(|t| t.inner_html()).collect();
    let resistances = Selector::parse("tbody > tr:nth-child(2) > td").unwrap();
    let mut resistance_info: HashMap<String, Vec<String>> = HashMap::new();

    for (idx, element) in table_doc.select(&resistances).enumerate() {
        let stripped = strip_html(element.inner_html());
        let res = stripped.trim().to_string();

        if resistance_info.get(&res).is_none() {
            resistance_info.insert(res.clone(), vec![]);
        }

        resistance_info.get_mut(&res).unwrap().push(types_table[idx].trim().to_string());
    }

    Ok(resistance_info)
}

fn get_page_id(shadow: &String) -> Result<isize, Box<dyn Error>> {
    // https://megamitensei.fandom.com/api/v1#!/Articles
    // https://megamitensei.fandom.com/api.php?format=json&action=query&redirect=1&titles=Intrepid_Knight
    // https://megamitensei.fandom.com/api/v1/Articles/AsJson?id=

    let page_id_endpoint = format!(
        "https://megamitensei.fandom.com/api.php?format=json&action=query&redirect=1&titles={}&indexpageids",
        titlecase(shadow.as_str())
    );
    const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

    let page_meta_encoded = utf8_percent_encode(page_id_endpoint.as_str(), FRAGMENT);
    let page_meta: String = page_meta_encoded.collect();

    let body: PageMeta = reqwest::blocking::get(&page_meta)?.json()?;

    Ok(body.query.pageids[0].parse::<isize>().unwrap())
}

fn print_resistances(table: &HashMap<String, Vec<String>>) {
    for (resistance, kinds) in table.iter() {
        match resistance.as_str() {
            "Strong" => {
                print!("{}", "STRONG: ".blue());
                for k in kinds {
                    print!("{} ", k);
                }
                println!();
            },
            "Weak" => {
                print!("{}", "WEAK: ".red());
                for k in kinds {
                    print!("{} ", k);
                }
                println!();
            },
            "Null" => {
                print!("{}", "NULL: ".green());
                for k in kinds {
                    print!("{} ", k);
                }
                println!();
            },
            "Repel" => {
                print!("{}", "REPEL: ".purple());
                for k in kinds {
                    print!("{} ", k);
                }
                println!();
            },
            "Neutral" => {
                print!("{}", "NEUTRAL: ".yellow());
                for k in kinds {
                    print!("{} ", k);
                }
                println!();
            },
            _ => { }
        }
    }
}

fn determine_game(game: &String) -> Game {
    match game.to_lowercase().as_str() {
        "3" => Game {
            entry: Games::P3,
            tab_name: "Persona 3".to_string(),
            variant: Some("Normal Encounter".to_string())
        },
        "3j" => Game {
            entry: Games::P3J,
            tab_name: "The Journey".to_string(),
            variant: Some("Normal Encounter".to_string())
        },
        "3a" => Game {
            entry: Games::P3A,
            tab_name: "The Answer".to_string(),
            variant: None
        },
        "4" => Game {
            entry: Games::P4,
            tab_name: "Persona 4".to_string(),
            variant: None
        },
        "4g" => Game {
            entry: Games::P4G,
            tab_name: "Golden".to_string(),
            variant: None
        },
        _ => Game {
            entry: Games::P3J,
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

    let page_id = match get_page_id(&opts.shadow) {
        Ok(id) => { id },
        Err(e) => panic!(e.to_string())
    };

    if page_id == -1 {
        eprintln!("Shadow not found.");
        return Err(PageParseError.into());
    }

    let page = match get_page(&page_id) {
        Ok(p) => { p },
        Err(e) => { panic!(e.to_string()) }
    };

    let subsection = match get_game_section(&page, &game) {
        Ok(s) => { s },
        Err(e) => { panic!(e.to_string()) }
    };

    let game_normalized_variant = normalize_variant(&opts.variant, game);

    let table_node = match get_table_node(&subsection, &game_normalized_variant) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e.to_string());
            std::process::exit(1);
        }
    };

    let table_data = match extract_table_data(table_node) {
        Ok(t) => { t },
        Err(_) => {
            eprintln!("found table, but failed to parse it. Maybe HTML changed?");
            std::process::exit(1);
        }
    };

    print_resistances(&table_data);

    Ok(())
}
