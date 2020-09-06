use super::*;

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

fn game_section_wrapper(shadow_page_id: isize, expected_tabs: u8) {
    let document = page_html(&shadow_page_id).unwrap();
    let game = Game {
        entry: PersonaTitle::P3J,
        tab_name: "The Journey",
        variant: Some("Normal Encounter")
    };

    let section = game_section(&document, &game);
    assert!(section.is_ok());

    let tabs = section.unwrap().tree.nodes().map(|n| {
        match n.value().as_element() {
            Some(e) => {
                if e.attr("class").is_some() && e.attr("class").unwrap() == "tabber" {
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
    game_section_wrapper(10968 as isize, 1);
}

#[test]
fn game_section_ok_no_tabs() {
    game_section_wrapper(11014 as isize, 0);
}

// game_table() tests. should return correct table structure for:
// 1. nested tabs: https://megamitensei.fandom.com/wiki/Green_Sigil#Persona%203
// 2. by variant tab name: https://megamitensei.fandom.com/wiki/Intrepid_Knight
// 3. by game tab name: https://megamitensei.fandom.com/wiki/Killing_Hand
// 4. no tabs: https://megamitensei.fandom.com/wiki/Silent_Book
fn game_table_wrapper(shadow_page_id: isize) {
    let document = page_html(&shadow_page_id).unwrap();
    let game = Game {
        entry: PersonaTitle::P3J,
        tab_name: "The Journey",
        variant: Some("Normal Encounter")
    };
    let section = game_section(&document, &game).unwrap();
    let table = game_table(&section, &game).unwrap();

    let third = table.tree.nodes().nth(3).unwrap();
    assert!(third.value().is_element());

    let table_element = third.value().as_element().unwrap();
    assert!(table_element.attr("class").is_some());
    assert_eq!(table_element.attr("class").unwrap(), "customtable");
}

#[test]
fn game_table_nested_tabs() {
    game_table_wrapper(31809 as isize);
}

#[test]
fn game_table_variant_name() {
    game_table_wrapper(5302 as isize);
}

#[test]
fn game_table_game_name() {
    game_table_wrapper(5301 as isize);
}

#[test]
fn game_table_no_tabs() {
    game_table_wrapper(31995 as isize);
}

fn extract_table_data_wrapper(shadow_page_id: isize, truth: HashMap<String, Vec<String>>) {
    let document = page_html(&shadow_page_id).unwrap();
    let game = Game {
        entry: PersonaTitle::P3J,
        tab_name: "The Journey",
        variant: Some("Sub-boss")
    };
    let section = game_section(&document, &game).unwrap();
    let table = game_table(&section, &game).unwrap();
    let data = extract_table_data(&table).unwrap();

    assert_eq!(data.len(), truth.len());
    assert!(truth.keys().all(|k| data.contains_key(k)));
    assert!(truth.keys().all(|k| {
        truth.get(k).unwrap().len() == truth.get(k).unwrap().iter().zip(data.get(k).unwrap()).filter(|&(a, b)| a == b).count()
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

    extract_table_data_wrapper(5302 as isize, known_data);
}