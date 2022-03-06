use sqlite3;
use std::fs;
use text_io::{read};
use std::collections::HashMap;
use colored::Colorize;
mod views;

pub struct Server{
    db: HashMap<String, String>,
}
impl Server {

    pub fn load_databases(&mut self) -> (){
        self.db.clear();
        let paths = fs::read_dir("src/databases").unwrap();
        for (i, p) in paths.enumerate() {
            self.db.insert(i.to_string(), p.unwrap().path().display().to_string());
        }
    }

    pub fn print_db(&mut self){
        self.load_databases();
        println!("{}", "--- DATABASES ---".bright_blue());
        for e in self.db.iter_mut() {
            println!("{} - {}", e.0.bright_red(), e.1.bright_yellow());
        }
        println!("{}", "-----------------".bright_blue());
    }
}

fn main() {
    let mut connection: sqlite3::Connection;
    let mut all_database = Server{
        db: HashMap::new(),
    };
    let mut ctrl: String;
    views::menu(1); 
    ctrl = read!("{}\n");
    while ctrl != "0"{
        match &ctrl[..]{
            "1" => {
                println!("{}", "--- New DB ---".bright_blue());
                println!("{}", "Type database name, To cancel type .".bright_yellow());
                ctrl = read!("{}\n");
                match &ctrl[..]{
                    "." => (),
                    _ => {
                        sqlite3::open(format!("src/databases/{}", &ctrl)).unwrap();
                        }
                    }
                },
            "2" => {
                println!("{}", "--- Use DB ---".bright_blue());
                println!("{}", "What is the number of database that you want to use, To cancel type .".bright_yellow());
                all_database.print_db();
                ctrl = read!("{}\n");
                match &ctrl[..] {
                    "." => continue,
                    _ => {
                        connection = sqlite3::open(all_database.db[&ctrl].clone()).unwrap();
                        views::menu(2);
                        ctrl = read!("{}\n");
                        while ctrl != "0"{
                            match &ctrl[..]{
                                "1" => { // Insert
                                    println!("{}", "Table name to insert:".bright_yellow());
                                    let table: String = read!("{}\n");
                                    println!("{}", "What fields do you want to add: (separated by comma) i.e: name,age,contry".bright_yellow());
                                    let fields: String = read!("{}\n");
                                    println!("{}", "Value to add: (separated by comma) i.e: 'Daniel',21,'Brazil'".bright_yellow());
                                    let values: String = read!("{}\n");
                                    match connection.execute(format!("INSERT INTO {} ({}) VALUES ({}) ", table, fields, values)){
                                        Ok(_) => println!("{}", "Successfully added".bright_green()),
                                        Err(e) => println!("{}: {}", "Err".red(), e)
                                    }
                                },
                                "2" => { // Select
                                    println!("{}", "Table name to show:".bright_yellow());
                                    let table: String = read!("{}\n");
                                    println!("{}", "What fields do you want to select: (separated by comma, * = all) i.e: name,age,contry".bright_yellow());
                                    let fields: String = read!("{}\n");
                                    match connection.iterate(format!("SELECT {} FROM {}", fields, table), |pairs| {
                                        println!("{}", "-------------------------------".bright_blue());
                                        for &(columns, value) in pairs.iter(){
                                            if value == None {continue;}
                                            println!("{} {} - {} \t {}", "|".bright_blue(), columns, value.unwrap(), "|".bright_blue());
                                        }
                                        println!("{}", "-------------------------------".bright_blue());
                                        true   
                                    }){
                                        Ok(_) => (),
                                        Err(x) => println!("{} {}", "Err:".red(), x)
                                    };
                                }
                                "3" => { // Update
                                    println!("{}", "Table name to update".bright_yellow());
                                    let table: String = read!("{}\n");
                                    println!("{}", "Fields and values to update. I.e: name = 'Daniel', age = 21".bright_yellow());
                                    let n_values: String = read!("{}\n");
                                    println!("{}", "Conditions to update. I.e: ID = 5".bright_yellow());
                                    let condition: String = read!("{}\n");
                                    match connection.execute(format!("UPDATE {} SET {} WHERE {}", table, n_values, condition)) {
                                        Ok(_) => println!("{}", "Sucessfully updated!".bright_green()),
                                        Err(e) => println!("{} {}", "Err:".red(), e)
                                    }
                                },
                                "4" => { // Delete
                                    println!("{}", "Table name to delete:".bright_yellow());
                                    let table: String = read!("{}\n");
                                    println!("{}", "Conditions do delete value. Ex: ID = 5".bright_yellow());
                                    let condition: String = read!("{}\n");
                                    match connection.execute(format!("DELETE FROM {} WHERE {}", table, condition)){
                                        Ok(_) => println!("{}", "Successfully deleted!".bright_green()),
                                        Err(e) => println!("{} {}", "Err:".red(), e)
                                    }

                                },
                                "5" => {// Options
                                    views::menu(3);
                                    ctrl = read!("{}\n");
                                    while ctrl != "0"{
                                        match &ctrl[..] {
                                            "1" => {
                                                println!("{}", "Table name to insert:".bright_yellow());
                                                let table: String = read!("{}\n");
                                                println!("{}", "Fields, in SQL style (separated by comma) i.e: Columm1 type, Columm2 type. E.g: ID INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, name VARCHAR(50)".bright_yellow());        
                                                let fields: String = read!("{}\n");
                                                match connection.execute(format!("CREATE TABLE {} ({});", table, fields)){
                                                    Ok(_) => println!("{}", "Successfully added".bright_green()),
                                                    Err(e) => println!("{} {}", "Err:".red(), e)
                                                }
                                            },
                                            "2" => {
                                                println!("{}", "-----------TABLES-------------".bright_blue());
                                                let _ = connection.iterate("SELECT name FROM sqlite_schema WHERE type='table' AND name NOT LIKE 'sqlite_%';", |table| {
                                                    for &(_, b) in table.iter(){
                                                        println!("{} {} \t {}", "|".white(), b.unwrap(), "|".white());
                                                    }
                                                    true
                                                });
                                                println!("{}", "-------------------------------".bright_blue());
                                            },
                                            "3" => {
                                                println!("{}", "Table name to delete: ".bright_yellow());
                                                let table: String = read!("{}\n");
                                                match connection.execute(format!("DROP TABLE IF EXISTS {}", table)) {
                                                    Ok(_) => println!("{}", "Successfully droped.".bright_green()),
                                                    Err(e) => println!("{} {}", "Err:".red(), e)
                                                }
                                            },
                                            "4" => {
                                                println!("{}", "Table name to see collumns: ".bright_yellow());
                                                let table: String = read!("{}\n");
                                                println!("{}", "-----------Columns-------------".bright_blue());
                                                let _ = connection.iterate(format!("SELECT sql FROM sqlite_master
                                                WHERE tbl_name = '{}' AND type = 'table'", table), |table| {
                                                    for &(_, b) in table.iter(){
                                                        println!("{}{} \t {}", "|".white(), b.unwrap(), "|".white());
                                                    }
                                                    true
                                                });
                                                println!("{}", "-------------------------------".bright_blue());
                                            },
                                            "0" => (),
                                            _ => {
                                                println!("{}", "Not a valid option!".red());
                                            }
                                        }
                                        views::menu(3);
                                        ctrl = read!("{}\n");
                                    }
    
                                },
                                "0" => (),
                                _ => println!("{}", "Not a valid option!".red())
                            }
                            views::menu(2);
                            ctrl = read!("{}\n");
                        }
                    }   
                }
            },
            "3" => {
                println!("{}", "--- Delete DB ---".bright_blue());
                println!("{}", "What is the number of database that you want to delete, if you wanna cancel type .".bright_yellow());
                all_database.print_db();
                ctrl = read!("{}\n");
                if ctrl == "." {continue;}
                fs::remove_file(all_database.db[&ctrl].clone()).expect("Database not found!");
            },
            "4" => all_database.print_db(),
            _ => println!("{}", "Not a valid option!".red())
        }
        views::menu(1);
        ctrl = read!("{}\n");
    }
}
