use std::env::args;
use std::collections::HashMap;
use serde::{Deserialize};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use titlecase::titlecase;
use std::process::exit;
use select::document::Document;
use select::predicate::{Predicate, Attr, Class, Name};
use core::fmt;

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

fn get_page(page_id: i32) -> Result<String, Box<dyn std::error::Error>> {
    let page_endpoint = format!("https://megamitensei.fandom.com/api/v1/Articles/AsJson?id={}", page_id);
    let body: Page = reqwest::blocking::get(&page_endpoint)?.json()?;

    let document = Document::from(body.content.as_str());
    let table_node = document.find(Attr("id", "Persona_3_2"))
        .next()
        .and_then(|n| n.parent())
        .and_then(|n| n.find(Class("tabbertab")).next())
        .and_then(|n| n.find(Name("table")).take(1).next())
        .and_then(|n| n.find(Name("table")).take(1).next())
        .and_then(|n| n.find(Name("table")).nth(2));

    let table_node = if let Some(table_node) = table_node {
        table_node
    } else {
        return Err(PageParseError.into());
    };

    let mut weaknesses_resistances: HashMap<String, String> = HashMap::new();
    for (idx, kind) in table_node.find(Name("th")).enumerate() {
        // TODO use if let syntax here instead of unwrap
        let res = table_node.find(Name("td")).nth(idx).unwrap();
        println!("{} -> {}", kind.text(), res.text());
    }

    // println!("{}", node);

    Ok(body.content)
}

fn get_page_id(persona: String, shadow: String) -> Result<i32, Box<dyn std::error::Error>> {
    // https://megamitensei.fandom.com/api/v1#!/Articles
    // https://megamitensei.fandom.com/api.php?format=json&action=query&redirect=1&titles=Intrepid_Knight
    // https://megamitensei.fandom.com/api/v1/Articles/AsJson?id=

    let page_id_endpoint = format!("https://megamitensei.fandom.com/api.php?format=json&action=query&redirect=1&titles={}&indexpageids", titlecase(shadow.as_str()));
    const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

    let page_meta_encoded = utf8_percent_encode(page_id_endpoint.as_str(), FRAGMENT);
    let page_meta: String = page_meta_encoded.collect();

    let body: PageMeta = reqwest::blocking::get(&page_meta)?
        .json()?;

    Ok(body.query.pageids[0].parse::<i32>().unwrap())
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
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

    let res = match get_page_id(persona, shadow) {
        Ok(id) => { id },
        Err(e) => panic!(e.to_string())
    };

    if res == -1 {
        eprintln!("Shadow not found.");
        return Err(PageParseError.into());
    }

    let res = get_page(res)?;
    Ok(())

    // println!("{}", res);
}
