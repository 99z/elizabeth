use colored::*;
use crate::wikia::{Game, PersonaTitle, Shadow};

pub fn determine_game(game: &str) -> Game {
    match game.to_lowercase().as_str() {
        "3" | "3j" => Game {
            entry: PersonaTitle::P3J,
            entry_text: "Persona 3".to_string(),
            tab_names: vec!["The Journey".to_string(), "Persona 3".to_string()],
            variant: Some("Normal".to_string())
        },
        "3a" => Game {
            entry: PersonaTitle::P3A,
            entry_text: "Persona 3".to_string(),
            tab_names: vec!["The Answer".to_string()],
            variant: None
        },
        "4" | "4g" => Game {
            entry: PersonaTitle::P4G,
            entry_text: "Persona 4".to_string(),
            tab_names: vec!["Golden".to_string()],
            variant: None
        },
        _ => Game {
            entry: PersonaTitle::P3J,
            entry_text: "Persona 3".to_string(),
            tab_names: vec!["The Journey".to_string(), "Persona 3".to_string()],
            variant: Some("Normal".to_string())
        },
    }
}

// Necessary to handle the seemingly random cases when resistance text
// is surrounded by some html tag, e.g. <span>Weak</span>
pub fn strip_cell_tags(cell: String) -> String {
    if cell.contains("Weak") {
        return "Weak".to_string();
    } else if cell.contains("Strong") {
        return "Strong".to_string();
    } else if cell.contains("Repel") {
        return "Repel".to_string();
    } else if cell.contains("Null") {
        return "Null".to_string();
    } else if cell.contains("Drain") {
        return "Drain".to_string();
    }

    "Neutral".to_string()
}

pub fn print_resistances(shadow: &Shadow) {
    println!("{}", shadow.name);
    println!();

    for tab in &shadow.info {
        println!("{}", tab.variant);

        for (resistance, kinds) in &tab.resistances {
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
                "Drain" => {
                    print!("{}", "DRAIN: ".bright_green());
                    for k in kinds {
                        print!("{}", k);
                    }
                    println!();
                },
                _ => {}
            }
        }

        println!();
    }
}