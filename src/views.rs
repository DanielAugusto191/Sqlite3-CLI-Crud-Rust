use colored::Colorize;

pub fn main_menu(){
    println!("------------");
    println!("--- ACTIONS ---");
    println!("1. Create new database.");
    println!("2. Use a database.");
    println!("3. Delete a database.");
    println!("4. Show databases.");
    println!("0. Exit.");
    println!("------------");
}

pub fn selected_table(s: String){
    println!("------------");
    println!("--- ACTIONS FOR {} ---", s.green());
    println!("1. Insert on a table");
    println!("2. Select on a table");
    println!("3. Update on a table");
    println!("4. Delete on a table");
    println!("5. Tables options");
    println!("0. Back.");
    println!("------------");
}
pub fn table_option(){
    println!("------------");
    println!("--- ACTIONS ---");
    println!("1. Add table.");
    println!("2. Show tables on DB");
    println!("3. Alter table");
    println!("4. Delete table");
    println!("0. Back.");
    println!("------------");
}
