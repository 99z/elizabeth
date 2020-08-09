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

impl fmt::Display for PageParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "table data not found")
    }
}

impl std::error::Error for PageParseError {}

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

const P3_SELECTOR: &str = "#Persona_3_2";
const P4_SELECTOR: &str = "#Persona_4_2";

fn gen_table_selector(index: &usize) -> Selector {
    let sel_string = format!("div:nth-child({}) > table > tbody > tr > td > table:nth-child(1) > tbody > tr > td > table:nth-child(2)", index);
    Selector::parse(sel_string.as_str()).unwrap()
}

fn get_page(page_id: &i32) -> Result<Html, Box<dyn Error>> {
    let page_endpoint = format!("https://megamitensei.fandom.com/api/v1/Articles/AsJson?id={}", page_id);
    let body: Page = reqwest::blocking::get(&page_endpoint)?.json()?;
    // println!("{}", body.content);

    let document = Html::parse_fragment(body.content.as_str());
    Ok(document)
}

fn get_table_node(doc: &Html) -> Result<Html, Box<dyn Error>> {
    let tabs = Selector::parse(".tabbertab").unwrap();
    let tab_idx = doc.select(&tabs).position(|t| t.value().attr("title").unwrap() == "Normal Encounter").unwrap();
    let tab_selector = gen_table_selector(&tab_idx);
    let table_node = doc.select(&tab_selector);

    let resistance_table = Html::parse_fragment(table_node.map(|n| n.html()).collect::<String>().as_str());
    Ok(resistance_table)
}

fn get_game_section(page: &Html) -> Result<Html, Box<dyn Error>> {
    let selector = Selector::parse(format!("{} + .tabber", P3_SELECTOR).as_str()).unwrap();
    let subsection_sel = page.select(&selector);
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
            _ => { println!("") }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = args().collect();
    let mut opts = getopts::Options::new();
    opts.reqopt("s", "shadow", "Name of shadow", "SHADOW")
        .reqopt("p", "persona", "Series number", "PERSONA");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()) }
    };

    let persona = matches.opt_str("p").unwrap();
    let shadow = matches.opt_str("s").unwrap();

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

    let subsection = match get_game_section(&page) {
        Ok(s) => { s },
        Err(e) => { panic!(e.to_string()) }
    };

    let table_node = match get_table_node(&subsection) {
        Ok(t) => { t },
        Err(e) => { panic!(e.to_string()) }
    };

    let table_data = match extract_table_data(table_node) {
        Ok(t) => { t },
        Err(e) => { panic!(e.to_string()) }
    };

    print_resistances(&table_data);

    Ok(())
}
