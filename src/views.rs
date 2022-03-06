use colored::Colorize;

pub fn menu(ID: usize){ 
    let mut msg: Vec<&str> = Vec::new();
    match ID {
        1 => { // Initial
            msg.push("Create new database");
            msg.push("Use a database");
            msg.push("Delete a database");
            msg.push("Show databases");
        }
        2 => { // Use Database
            msg.push("Insert in a table");
            msg.push("Select in a table");
            msg.push("Update in a table");
            msg.push("Delete in a table");
            msg.push("Tables options");
        },
        3 => { // Table Option
            msg.push("Add table");
            msg.push("Show tables in DB");
            msg.push("Delete table");
            msg.push("Show columns in a table");
        },
        _ => ()
    }
    let n = msg.len();
    println!("{}", "--- ACTIONS ---".bright_blue());
    for i in 0..n {
        println!("{} {}", (i+1).to_string().white(), msg[i].bright_yellow());    
    }
    println!("{} {}", "0".white(), "Back".bright_yellow());
    println!("{}", "------------".bright_blue());
}