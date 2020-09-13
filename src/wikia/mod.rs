#[cfg(test)]
mod test;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use titlecase::titlecase;
use serde::{Deserialize};
use scraper::{Selector, Html};
use std::collections::HashMap;
use crate::utils;
use crate::errors::{SelectorParseError, NoVariantError, NoShadowError};

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

#[derive(Debug, Clone)]
pub struct Game<'a> {
    pub entry: PersonaTitle,
    pub entry_text: &'a str,
    pub tab_names: Vec<String>,
    pub variant: Option<&'a str>
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum PersonaTitle {
    P3,
    P3J,
    P3A,
    P4,
    P4G
}

const P3_SELECTOR: &str = "[id^=Persona_3]";
const P4_SELECTOR: &str = "[id^=Persona_4]";

pub fn get_shadow_page_id(shadow: &String) -> anyhow::Result<isize> {
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
    let id = body.query.pageids[0].parse::<isize>()?;

    Ok(id)
}

pub fn page_html(page_id: &isize) -> anyhow::Result<Html> {
    let page_endpoint = format!(
        "https://megamitensei.fandom.com/api/v1/Articles/AsJson?id={}",
        page_id
    );
    let body: Page = reqwest::blocking::get(&page_endpoint)?.json()?;
    // println!("{:#?}", body.content);

    let document = Html::parse_fragment(body.content.as_str());
    Ok(document)
}

// determine if shadow appears only in 1 game, changing the base selector
// yea, seriously, this was the best way I could think of given the html that comes back
pub fn appears_in(page: &Html, entry: &Game) -> anyhow::Result<bool> {
    // weird selector right?
    // https://megamitensei.fandom.com/wiki/Bigoted_Maya
    // note the 'Appearaces' section
    // in addition to having an unpredictable page structure, seems i can't even get
    // a guarantee things will be spelled correctly
    // but wait! it gets better:
    // https://megamitensei.fandom.com/wiki/Desirous_Maya
    let appearances_section = match Selector::parse("[id^=Appe] > ul > li > i") {
        Ok(s) => s,
        Err(_) => return Err(NoShadowError.into())
    };

    for element in page.select(&appearances_section) {
        if entry.entry_text.contains(&element.text().collect::<String>()) {
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn game_section(page: &Html, game: &Game) -> anyhow::Result<Html> {
    let persona_selector = if game.entry == PersonaTitle::P3 || game.entry == PersonaTitle::P3J || game.entry == PersonaTitle::P3A {
        P3_SELECTOR
    } else {
        P4_SELECTOR
    };

    let mut base_selector = Selector::parse(format!("{} + .tabber", persona_selector).as_str()).unwrap();

    let selector = match page.select(&base_selector).count() {
        0 => {
            // case when no tabs
            base_selector = Selector::parse(format!("{} + table", persona_selector).as_str()).unwrap();

            match page.select(&base_selector).count() {
                0 => {
                    // case when multiple versions exist (Journey/Answer), but are separate
                    // tables instead of tabbed
                    // https://megamitensei.fandom.com/wiki/Indolent_Maya
                    base_selector = Selector::parse(
                        format!(
                            "{} ~ #{} + table",
                            persona_selector,
                            game.tab_names.first().unwrap().replace(" ", "_")).as_str()
                    ).unwrap();

                    match page.select(&base_selector).count() {
                        // case when shadow only has appearance in one game, and the html
                        // does not have sections for games
                        0 => {
                            base_selector = Selector::parse(".tabber").unwrap();

                            match page.select(&base_selector).count() {
                                0 => {
                                    base_selector = Selector::parse(
                                        format!(
                                            "[id^={}] + table",
                                            game.tab_names.first().unwrap().replace(" ", "_")).as_str()
                                    ).unwrap();

                                    match page.select(&base_selector).count() {
                                        0 => return Err(NoVariantError.into()),
                                        _ => base_selector
                                    }
                                },
                                _ => base_selector
                            }
                        },
                        _ => base_selector
                    }
                },
                _ => base_selector
            }
        },
        _ => base_selector
    };

    let subsection_sel =  page.select(&selector);
    let subsection = Html::parse_fragment(subsection_sel.map(|n| n.html())
        .collect::<String>().as_str());

    Ok(subsection)
}

pub fn game_table(doc: &Html, game: &Game) -> anyhow::Result<Html> {
    // this silliness is required because, sometimes, RARELY, there's nested tabs
    // even within a game's section
    // https://megamitensei.fandom.com/wiki/Green_Sigil#Persona%203
    let mut tabs = match Selector::parse(".tabbertab > .tabber > .tabbertab") {
        Ok(s) => s,
        Err(_) => return Err(SelectorParseError.into())
    };

    // if the nested tabs don't exist, set selector properly
    if doc.select(&tabs).count() < 1 {
        tabs = match Selector::parse(".tabbertab") {
            Ok(s) => s,
            Err(_) => return Err(SelectorParseError.into())
        }
    }

    let tab_selector = match doc
        .select(&tabs)
        .position(|t| t.value().attr("title").unwrap().contains( game.variant.unwrap())
        ) {
        Some(idx) => gen_table_selector(&idx)?,
        None => {
            match doc
                .select(&tabs)
                .position(|t| game.tab_names.contains(&t.value().attr("title").unwrap().to_string())
                ) {
                Some(idx) => gen_table_selector(&idx)?,
                None => {
                    // tabs exist but none matched variant + game
                    if doc.select(&tabs).count() >= 1 {
                        return Err(NoVariantError.into())
                    }

                    // no tabs exist, default selector
                    match Selector::parse("table > tbody > tr > td > table:nth-child(1) >\
                    tbody > tr > td > table:nth-child(2)") {
                        Ok(s) => s,
                        Err(_) => return Err(SelectorParseError.into())
                    }
                }
            }
        }
    };

    let table_node = doc.select(&tab_selector);
    let resistance_table = Html::parse_fragment(table_node.map(|n| n.html())
        .collect::<String>().as_str());
    Ok(resistance_table)
}

pub fn extract_table_data(table_doc: &Html) -> anyhow::Result<HashMap<String, Vec<String>>> {
    let types = match Selector::parse("tbody > tr:nth-child(1) > th") {
        Ok(s) => s,
        Err(_) => return Err(SelectorParseError.into())
    };
    let types_table: Vec<String> = table_doc.select(&types)
        .map(|t| t.inner_html().trim().to_string()).collect();

    let resistances = match Selector::parse("tbody > tr:nth-child(2) > td") {
        Ok(s) => s,
        Err(_) => return Err(SelectorParseError.into())
    };
    let mut resistance_info: HashMap<String, Vec<String>> = HashMap::new();

    for (idx, element) in table_doc.select(&resistances).enumerate() {
        let stripped = utils::strip_cell_tags(element.inner_html());
        let res = stripped.to_string();

        if resistance_info.get(&res).is_none() {
            resistance_info.insert(res.clone(), vec![]);
        }

        resistance_info.get_mut(&res).unwrap().push(types_table[idx].to_string());
    }

    Ok(resistance_info)
}

fn gen_table_selector(index: &usize) -> anyhow::Result<Selector> {
    let sel_string = format!(
        "div:nth-child({}) > table > tbody > tr > td > table:nth-child(1) >
        tbody > tr > td > table:nth-child(2)",
        index + 1
    );
    let res = match Selector::parse(sel_string.as_str()) {
        Ok(s) => Ok(s),
        Err(_) => return Err(SelectorParseError.into())
    };

    res
}
