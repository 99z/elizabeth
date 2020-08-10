use std::env::args;
use std::collections::HashMap;
use serde::{Deserialize};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use titlecase::titlecase;
use core::fmt;
use scraper::{Selector, Html};
use std::error::Error;
use colored::*;

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

#[derive(PartialEq)]
enum Games {
    P3J,
    P3A,
    P4,
    P4G
}

const P3_SELECTOR: &str = "#Persona_3_2";
const P4_SELECTOR: &str = "#Persona_4_2";

fn gen_table_selector(index: &usize) -> Selector {
    let sel_string = format!("div:nth-child({}) > table > tbody > tr > td > table:nth-child(1) > tbody > tr > td > table:nth-child(2)", index + 1);
    Selector::parse(sel_string.as_str()).unwrap()
}

fn get_page(page_id: &i32) -> Result<Html, Box<dyn Error>> {
    let page_endpoint = format!("https://megamitensei.fandom.com/api/v1/Articles/AsJson?id={}", page_id);
    let body: Page = reqwest::blocking::get(&page_endpoint)?.json()?;
    // println!("{}", body.content);

    let document = Html::parse_fragment(body.content.as_str());
    Ok(document)
}

fn get_table_node(doc: &Html, variant: &String) -> Result<Html, Box<dyn Error>> {
    let tabs = Selector::parse(".tabbertab").unwrap();
    let tab_selector = match doc.select(&tabs).position(|t| t.value().attr("title").unwrap() == variant) {
        Some(idx) => gen_table_selector(&idx),
        None => Selector::parse("table > tbody > tr > td > table:nth-child(1) > tbody > tr > td > table:nth-child(2)").unwrap()
    };
    let table_node = doc.select(&tab_selector);

    let resistance_table = Html::parse_fragment(table_node.map(|n| n.html()).collect::<String>().as_str());
    Ok(resistance_table)
}

fn get_game_section(page: &Html, game: Games) -> Result<Html, Box<dyn Error>> {
    let persona_selector = if game == Games::P3J {
        P3_SELECTOR
    } else {
        P4_SELECTOR
    };

    let base_selector = Selector::parse(format!("{} + .tabber", persona_selector).as_str()).unwrap();
    let mut subsection_sel = page.select(&base_selector);

    let mut size = 0;
    for _ in subsection_sel {
        size += 1;
    }

    let selector = if size == 0 { Selector::parse(format!("{} + table", persona_selector).as_str()).unwrap() } else { base_selector.clone() };
    subsection_sel =  page.select(&selector);
    let subsection = Html::parse_fragment(subsection_sel.map(|n| n.html()).collect::<String>().as_str());

    Ok(subsection)
}

fn extract_table_data(table_doc: Html) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let types = Selector::parse("tbody > tr:nth-child(1) > th").unwrap();
    let types_table: Vec<String> = table_doc.select(&types).map(|t| t.inner_html()).collect();
    let resistances = Selector::parse("tbody > tr:nth-child(2) > td").unwrap();
    let mut resistance_info: HashMap<String, Vec<String>> = HashMap::new();

    for (idx, element) in table_doc.select(&resistances).enumerate() {
        let res = if element.inner_html().trim() == "-" {
            "Neutral".to_string()
        } else {
            element.inner_html().trim().to_string()
        };

        if resistance_info.get(&res).is_none() {
            resistance_info.insert(res.clone(), vec![]);
        }

        resistance_info.get_mut(&res).unwrap().push(types_table[idx].trim().to_string());
    }

    Ok(resistance_info)
}

fn get_page_id(persona: &String, shadow: &String) -> Result<i32, Box<dyn Error>> {
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

    let body: PageMeta = reqwest::blocking::get(&page_meta)?
        .json()?;

    Ok(body.query.pageids[0].parse::<i32>().unwrap())
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
            "Neutral" => {
                print!("{}", "NEUTRAL: ".black());
                for k in kinds {
                    print!("{} ", k);
                }
                println!();
            },
            _ => { }
        }
    }
}

fn determine_game(game: &String) -> Games {
    match game.to_lowercase().as_str() {
        "3" => Games::P3J,
        "3j" => Games::P3J,
        "3a" => Games::P3A,
        "4" => Games::P4,
        "4g" => Games::P4G,
        _ => Games::P3J
    }
}

fn normalize_variant(variant: &str) -> String {
    match variant {
        "normal" => "Normal Encounter".to_string(),
        "sub" => "Sub-boss".to_string(),
        _ => "Normal Encounter".to_string()
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = args().collect();
    let mut opts = getopts::Options::new();
    opts.reqopt("s", "shadow", "Name of shadow", "SHADOW")
        .reqopt("p", "persona", "Series number", "PERSONA")
        .optopt("v", "variant", "Enemy variant", "VARIANT");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()) }
    };

    let persona = match matches.opt_str("p") {
        Some(p) => { p },
        None => { "p3".to_string() }
    };
    let shadow = match matches.opt_str("s") {
        Some(s) => { s },
        None => {
            eprintln!("Must specify shadow name.");
            return Err(NoShadowError.into())
        }
    };
    let variant = match matches.opt_str("v") {
        Some(v) => { normalize_variant(&v) },
        None => { "Normal Encounter".to_string() }
    };

    let game = determine_game(&persona);

    let page_id = match get_page_id(&persona, &shadow) {
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

    let subsection = match get_game_section(&page, game) {
        Ok(s) => { s },
        Err(e) => { panic!(e.to_string()) }
    };

    let table_node = match get_table_node(&subsection, &variant) {
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
