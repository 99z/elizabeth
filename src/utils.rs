use std::collections::HashMap;
use colored::*;

// Necessary to handle the seemingly random cases when resistance text
// is surrounded by some html tag, e.g. <span>Weak</span>
pub fn strip_cell_tags(cell: String) -> String {
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

pub fn print_resistances(table: &HashMap<String, Vec<String>>) {
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