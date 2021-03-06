use super::*;
use scraper::node::Element;

#[test]
fn get_shadow_page_id_ok_shadow() {
    let shadow = "Intrepid Knight".to_string();
    let known_id = 10968;

    let id = get_shadow_page_id(&shadow);
    assert!(id.is_ok());
    assert_eq!(known_id, id.unwrap());
}

#[test]
fn get_shadow_page_id_bad_shadow() {
    let shadow = "the Hedgehog".to_string();
    let id = get_shadow_page_id(&shadow);
    assert!(id.is_ok());

    assert_eq!(-1, id.unwrap());
}

#[test]
fn page_html_ok_id() {
    let shadow_page_id = 10968 as isize;
    let document = page_html(&shadow_page_id);
    assert!(document.is_ok());

    let known_document_size = 1755;
    assert_eq!(known_document_size, document.unwrap().tree.nodes().count());
}

#[test]
fn page_html_bad_id() {
    let bad_id = -1 as isize;
    assert!(page_html(&bad_id).is_err(), true);
}

// should return correct section for:
// 1. normal tab structure
// 2. game heading, no tabs: https://megamitensei.fandom.com/wiki/Primitive_Idol
// 3. Journey/Answer only, no game heading, no tabs: https://megamitensei.fandom.com/wiki/Conceited_Maya
// 4. Journey AND Answer, game heading, no tabs, separate tables: https://megamitensei.fandom.com/wiki/Indolent_Maya
fn game_section_wrapper(shadow_page_id: isize, expected_tabs: u8, game: &Game, shadow_name: String) {
    let document = page_html(&shadow_page_id).unwrap();
    let section = game_section(&document, &game, shadow_name);
    assert!(section.is_ok());

    let tabs = section.unwrap().tree.nodes().map(|n| {
        match n.value().as_element() {
            Some(e) => {
                if e.attr("class").is_some() && e.attr("class").unwrap() == "tabbertab" {
                    return true;
                }

                false
            }
            None => false
        }
    }).fold(0, |mut a, b| {
        if b {
            a += 1;
        }

        a
    });

    assert_eq!(tabs, expected_tabs);
}

#[test]
fn game_section_ok_with_tabs() {
    let game = Game {
        entry: PersonaTitle::P3J,
        entry_text: "Persona 3".to_string(),
        tab_names: vec!["The Journey".to_string(), "Persona 3".to_string()],
        variant: Some("Normal".to_string())
    };

    game_section_wrapper(10968 as isize, 2, &game, "Intrepid Knight".to_string());
}

#[test]
fn game_section_ok_no_tabs() {
    let game = Game {
        entry: PersonaTitle::P3J,
        entry_text: "Persona 3 FES".to_string(),
        tab_names: vec!["The Journey".to_string()],
        variant: Some("Normal Encounter".to_string())
    };

    game_section_wrapper(11014 as isize, 0, &game, "Primitive Idol".to_string());
}

#[test]
fn game_section_single_no_heading_no_tabs() {
    let game = Game {
        entry: PersonaTitle::P3J,
        entry_text: "Persona 3 FES".to_string(),
        tab_names: vec!["The Answer".to_string()],
        variant: Some("Normal Encounter".to_string())
    };

    game_section_wrapper(11023 as isize, 0, &game, "Conceited Maya".to_string());
}

#[test]
fn game_section_double_heading_no_tabs() {
    let game = Game {
        entry: PersonaTitle::P3J,
        entry_text: "Persona 3 FES".to_string(),
        tab_names: vec!["The Journey".to_string()],
        variant: Some("Normal Encounter".to_string())
    };

    game_section_wrapper(14533 as isize, 0, &game, "Indolent Maya".to_string());
}

// should return correct table structure for:
// 1. nested tabs: https://megamitensei.fandom.com/wiki/Green_Sigil#Persona%203
// 2. by variant tab name: https://megamitensei.fandom.com/wiki/Intrepid_Knight
// 3. by game tab name: https://megamitensei.fandom.com/wiki/Killing_Hand
// 4. no tabs: https://megamitensei.fandom.com/wiki/Silent_Book
// 5. 'Persona 3' and 'The Answer' tabs, instead of 'The Journey' but does appear in FES: https://megamitensei.fandom.com/wiki/Laughing_Table
// 6. appears in Journey/Answer and has sub-boss variant: https://megamitensei.fandom.com/wiki/Crying_Table
fn game_table_wrapper(shadow_page_id: isize, game: &Game, shadow_name: String) -> anyhow::Result<Element> {
    let document = page_html(&shadow_page_id)?;
    let section = game_section(&document, &game, shadow_name.clone())?;
    let table_nodes = game_table(&section)?;
    let (table, _variant) = table_nodes.first().unwrap();

    let third = match table.tree.nodes().nth(3) {
        Some(n) => n,
        None => return Err(errors::NoShadowError {
            name: shadow_name.clone(),
            game: game.entry_text.clone()
        }.into())
    };
    Ok(third.value().as_element().cloned().unwrap())
}

fn got_table(e: Element) -> bool {
    e.attr("class").is_some() && e.attr("class").unwrap() == "customtable"
}

#[test]
fn game_table_nested_tabs() {
    let game = Game {
        entry: PersonaTitle::P3J,
        entry_text: "Persona 3".to_string(),
        tab_names: vec!["The Journey".to_string()],
        variant: Some("Normal".to_string())
    };

    let element = game_table_wrapper(31809 as isize, &game, "Green Sigil".to_string());
    assert!(element.is_ok());

    let element = element.unwrap();
    assert!(got_table(element));
}

#[test]
fn game_table_variant_name() {
    let game = Game {
        entry: PersonaTitle::P3J,
        entry_text: "Persona 3".to_string(),
        tab_names: vec!["The Journey".to_string()],
        variant: Some("Normal".to_string())
    };

    let element = game_table_wrapper(5302 as isize, &game, "Liberating Idol".to_string());
    assert!(element.is_ok());

    let element = element.unwrap();
    assert!(got_table(element));
}

#[test]
fn game_table_game_name() {
    let game = Game {
        entry: PersonaTitle::P3J,
        entry_text: "Persona 3".to_string(),
        tab_names: vec!["The Journey".to_string()],
        variant: Some("Normal".to_string())
    };

    let element = game_table_wrapper(5301 as isize, &game, "Killing Hand".to_string());
    assert!(element.is_ok());

    let element = element.unwrap();
    assert!(got_table(element));
}

#[test]
fn game_table_no_tabs() {
    let game = Game {
        entry: PersonaTitle::P3J,
        entry_text: "Persona 3".to_string(),
        tab_names: vec!["The Journey".to_string()],
        variant: Some("Normal".to_string())
    };

    let element = game_table_wrapper(31995 as isize, &game, "Silent Book".to_string());
    assert!(element.is_ok());

    let element = element.unwrap();
    assert!(got_table(element));
}

#[test]
fn game_table_ok_p3_answer_tabs() {
    let game = Game {
        entry: PersonaTitle::P3J,
        entry_text: "Persona 3".to_string(),
        tab_names: vec!["The Answer".to_string()],
        variant: Some("Normal".to_string())
    };

    let element = game_table_wrapper(24131 as isize, &game, "Laughing Table".to_string());
    assert!(element.is_ok());

    let element = element.unwrap();
    assert!(got_table(element));
}

#[test]
fn game_table_two_games_variant() {
    let game = Game {
        entry: PersonaTitle::P3J,
        entry_text: "Persona 3".to_string(),
        tab_names: vec!["The Journey".to_string()],
        variant: Some("Normal".to_string())
    };

    let element = game_table_wrapper(10965 as isize, &game, "Crying Table".to_string());
    assert!(element.is_ok());

    let element = element.unwrap();
    assert!(got_table(element));
}

fn extract_table_data_wrapper(shadow_page_id: isize, truth: HashMap<String, Vec<String>>, shadow_name: String) {
    let document = page_html(&shadow_page_id).unwrap();
    let game = Game {
        entry: PersonaTitle::P3J,
        entry_text: "Persona 3 FES".to_string(),
        tab_names: vec!["The Journey".to_string()],
        variant: Some("Sub-boss".to_string())
    };

    let mut shadow_info = vec![];
    let section = game_section(&document, &game, shadow_name.clone()).unwrap();
    let table_nodes = game_table(&section).unwrap();
    for (table, variant) in table_nodes {
        shadow_info.push(extract_table_data(&table, &variant, &game).unwrap());
    }

    let boss_table = shadow_info.first().unwrap();
    assert_eq!(boss_table.resistances.len(), truth.len());
    assert!(truth.keys().all(|k| boss_table.resistances.contains_key(k)));
    assert!(truth.keys().all(|k| {
        truth.get(k).unwrap().len() == truth.get(k).unwrap().iter().zip(boss_table.resistances.get(k).unwrap()).filter(|&(a, b)| a == b).count()
    }));
}

#[test]
fn extract_table_data_ok() {
    let mut known_data: HashMap<String, Vec<String>> = HashMap::new();
    known_data.insert("Weak".to_string(), vec!["Ice".to_string()]);
    known_data.insert("Neutral".to_string(), vec![
        "Slash".to_string(),
        "Strike".to_string(),
        "Pierce".to_string(),
        "Elec".to_string(),
        "Light".to_string(),
        "Dark".to_string(),
        "Almi".to_string()
    ]);
    known_data.insert("Repel".to_string(), vec!["Wind".to_string()]);
    known_data.insert("Null".to_string(), vec!["Fire".to_string()]);

    extract_table_data_wrapper(5302 as isize, known_data, "Liberating Idol".to_string());
}
