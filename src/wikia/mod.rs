#[cfg(test)]
mod test;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use titlecase::titlecase;
use serde::{Deserialize, Serialize};
use scraper::{Selector, Html};
use std::collections::HashMap;
use crate::{utils, errors};
use crate::errors::NoVariantError;

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
pub struct Game {
    pub entry: PersonaTitle,
    pub entry_text: String,
    pub tab_names: Vec<String>,
    pub variant: Option<String>
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum PersonaTitle {
    P3J,
    P3A,
    P4,
    P4G
}

#[derive(Serialize, Debug)]
pub struct ShadowInfo {
    pub game: String,
    pub version: String,
    pub resistances: HashMap<String, Vec<String>>
}

#[derive(Serialize, Debug)]
pub struct Shadow {
    pub name: String,
    pub info: Vec<ShadowInfo>
}

const P3_SELECTOR: &str = "[id^=Persona_3]";
const P4_SELECTOR: &str = "[id^=Persona_4]";

const P3_ALL_SHADOWS: isize = 2807;
const P4_ALL_SHADOWS: isize = 12686;

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
    let appearances_section = Selector::parse("[id^=Appe] > ul > li > i").unwrap();

    let mut all_appearances = "".to_string();
    for element in page.select(&appearances_section) {
        let mut appearance = element.text().collect::<String>();
        appearance.retain(|c| !c.is_whitespace());
        all_appearances += &appearance;
    }
    all_appearances = all_appearances.replace("/", "");

    let entry_trimmed = entry.entry_text.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    if all_appearances.contains(&entry_trimmed) {
        return Ok(true);
    }

    Ok(false)
}

fn get_other_version(entry: &PersonaTitle) -> Game {
    if entry == &PersonaTitle::P3J {
        utils::determine_game("3a")
    } else if entry == &PersonaTitle::P3A {
        utils::determine_game("3j")
    } else if entry == &PersonaTitle::P4 {
        utils::determine_game("4g")
    } else if entry == &PersonaTitle::P4G {
        utils::determine_game("4")
    } else {
        utils::determine_game("default")
    }
}

pub fn arcana_sections(game: &Game) -> anyhow::Result<Vec<Shadow>> {
    let table_selector = Selector::parse(".table > tbody > tr td:nth-child(1)").unwrap();
    let page_id = if game.entry == PersonaTitle::P3J || game.entry == PersonaTitle::P3A {
        P3_ALL_SHADOWS
    } else {
        P4_ALL_SHADOWS
    };
    let page = page_html(&page_id)?;

    let other_game = get_other_version(&game.entry);
    let mut games = vec![game.clone(), other_game];

    let mut all_shadows: Vec<Shadow> = vec![];

    for element in page.select(&table_selector) {
        let shadow_name = &element.text().collect::<String>();
        let page_id = get_shadow_page_id(&shadow_name)?;
        let page_html = page_html(&page_id)?;

        let mut current_shadow = Shadow {
            name: shadow_name.clone(),
            info: vec![]
        };

        for g in &mut games {
            let appears_in = appears_in(&page_html, g)?;
            if !appears_in {
                let no_shadow_err = errors::NoShadowError {
                    name: shadow_name.clone(),
                    game: game.entry_text.clone()
                };
                eprintln!("{}", no_shadow_err);

                continue;
            }

            let subsection = match game_section(&page_html, g, shadow_name.clone()) {
                Ok(s) => s,
                Err(e) => {
                    let no_var_err = errors::NoVariantError {
                        shadow_name: shadow_name.clone(),
                        game: game.entry_text.clone(),
                        variant: game.tab_names.clone()
                    };
                    eprintln!("{}", no_var_err);

                    continue;
                }
            };

            let table_node = match game_table(&subsection, g, shadow_name.clone()) {
                Ok(t) => t,
                Err(e) => {
                    let no_var_err = errors::NoVariantError {
                        shadow_name: shadow_name.clone(),
                        game: game.entry_text.clone(),
                        variant: game.tab_names.clone()
                    };
                    eprintln!("{}", no_var_err);

                    continue;
                }
            };

            current_shadow.info.push(extract_table_data(&table_node, g)?);
        }

        all_shadows.push(current_shadow);
        // println!("{}", serde_json::to_string(&current_shadow)?);
    }

    Ok(all_shadows)
}

pub fn game_section(page: &Html, game: &Game, shadow_name: String) -> anyhow::Result<Html> {
    let persona_selector = if game.entry == PersonaTitle::P3J || game.entry == PersonaTitle::P3A {
        P3_SELECTOR
    } else {
        P4_SELECTOR
    };

    let mut base_selector = Selector::parse(format!("{} + .tabber", persona_selector).as_str()).unwrap();

    // TODO there's gotta be a better way to do this
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
                                        0 => return Err(NoVariantError {
                                            shadow_name,
                                            game: game.entry_text.clone(),
                                            variant: game.tab_names.clone()
                                        }.into()),
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
        _ => {
            let version_selector = Selector::parse(format!("{} + .tabber > .tabbertab", persona_selector).as_str()).unwrap();
            if page
                .select(&version_selector)
                .any(|t| t.value().attr("title").unwrap_or("").to_string() == "Portable".to_string()) {
                Selector::parse(format!("{} + .tabber > .tabbertab > .tabber", persona_selector).as_str()).unwrap()
            } else {
                base_selector
            }
        }
    };

    let subsection_sel =  page.select(&selector);
    let subsection = Html::parse_fragment(subsection_sel.map(|n| n.html())
        .collect::<String>().as_str());

    Ok(subsection)
}

pub fn game_table(doc: &Html, game: &Game, shadow_name: String) -> anyhow::Result<Html> {
    // this silliness is required because, sometimes, RARELY, there's nested tabs
    // even within a game's section
    // https://megamitensei.fandom.com/wiki/Green_Sigil#Persona%203
    let mut tabs = Selector::parse(".tabbertab > .tabber > .tabbertab").unwrap();

    // if the nested tabs don't exist, set selector properly
    if doc.select(&tabs).count() < 1 {
        tabs = Selector::parse(".tabbertab").unwrap();
    }

    let tab_selector = match doc
        .select(&tabs)
        .position(|t| t.value().attr("title").unwrap().contains( match &game.variant {
            Some(v) => v.as_str(),
            None => "Normal Encounter"
        })
        ) {
        Some(idx) => gen_table_selector(&idx),
        None => {
            match doc
                .select(&tabs)
                .position(|t| game.tab_names.contains(&t.value().attr("title").unwrap().to_string())
                ) {
                Some(idx) => gen_table_selector(&idx),
                None => {
                    // tabs exist but none matched variant + game
                    if doc.select(&tabs).count() >= 1 {
                        return Err(NoVariantError {
                            shadow_name,
                            game: game.entry_text.clone(),
                            variant: game.tab_names.clone()
                        }.into())
                    }

                    // no tabs exist, default selector
                    Selector::parse("table > tbody > tr > td > table:nth-child(1) >\
                    tbody > tr > td > table:nth-child(2)").unwrap()
                }
            }
        }
    };

    let table_node = doc.select(&tab_selector);
    let resistance_table = Html::parse_fragment(table_node.map(|n| n.html())
        .collect::<String>().as_str());
    Ok(resistance_table)
}

pub fn extract_table_data(table_doc: &Html, game: &Game) -> anyhow::Result<ShadowInfo> {
    let types = Selector::parse("tbody > tr:nth-child(1) > th").unwrap();
    let types_table: Vec<String> = table_doc.select(&types)
        .map(|t| t.inner_html().trim().to_string()).collect();

    let resistances = Selector::parse("tbody > tr:nth-child(2) > td").unwrap();

    let mut shadow_info = ShadowInfo {
        game: game.entry_text.clone(),
        version: game.tab_names[0].clone(),
        resistances: HashMap::new(),
    };

    for (idx, element) in table_doc.select(&resistances).enumerate() {
        let stripped = utils::strip_cell_tags(element.inner_html());
        let res = stripped.to_string();

        if shadow_info.resistances.get(&res).is_none() {
            shadow_info.resistances.insert(res.clone(), vec![]);
        }

        shadow_info.resistances.get_mut(&res).unwrap().push(types_table[idx].to_string());
    }

    Ok(shadow_info)
}

fn gen_table_selector(index: &usize) -> Selector {
    let sel_string = format!(
        "div:nth-child({}) > table > tbody > tr > td > table:nth-child(1) > tbody > tr > td > table:nth-child(2)",
        index + 1
    );

    Selector::parse(sel_string.as_str()).unwrap()
}
