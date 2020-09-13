use std::collections::HashMap;
use colored::*;

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
            "Drain" => {
                print!("{}", "DRAIN: ".bright_green());
                for k in kinds {
                    print!("{}", k);
                }
                println!();
            }
            _ => { }
        }
    }
}