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
        println!("--- DATABASES ---");
        for e in self.db.iter_mut() {
            println!("{} - {}", e.0, e.1);
        }
    }
}

fn main() {
    let mut connection: sqlite3::Connection;
    let mut all_database = Server{
        db: HashMap::new(),
    };
    let mut ctrl: String;
    views::main_menu(); 
    ctrl = read!("{}\n");
    while ctrl != "0"{
        match &ctrl[..]{
            "1" => {
                println!("--- New DB ---");
                println!("Type database name, To cancel type .");
                ctrl = read!("{}\n");
                match &ctrl[..]{
                    "." => (),
                    _ => {
                        sqlite3::open(format!("src/databases/{}", &ctrl)).unwrap();
                        }
                    }
                },
            "2" => {
                println!("--- Use DB ---");
                println!("What is the number of database that you want to use, To cancel type .");
                all_database.print_db();
                ctrl = read!("{}\n");
                match &ctrl[..] {
                    "." => break,
                    _ => {
                        connection = sqlite3::open(all_database.db[&ctrl].clone()).unwrap();
                        views::selected_table(ctrl[..].split("/").last().unwrap().to_string());
                        ctrl = read!("{}\n");
                        while ctrl != "0"{
                            match &ctrl[..]{
                                "1" => { // Insert
                                    println!("Table name: (. to cancel)");
                                    let table: String = read!("{}\n");
                                    if table == "." {break;}
                                    println!("What fields do you want to add: (separated by comma) i.e: name,age,contry");
                                    let fields: String = read!("{}\n");
                                    println!("Value to add: (separated by comma) i.e: Daniel,21,Brazil");
                                    let values: String = read!("{}\n");
                                    match connection.execute(format!("INSERT INTO {} ({}) VALUES {} ", table, fields, values)){
                                        Ok(_) => println!("Successfully added"),
                                        Err(e) => println!("Err {}", e)
                                    }
                                },
                                "2" => {
                                    println!("Table name: (To cancel type .)");
                                    let table: String = read!("{}\n");
                                    println!("What fields do you want to select: (separated by comma) i.e: name,age,contry");
                                    let fields: String = read!("{}\n");
                                    match connection.iterate(format!("SELECT ({}) from {}", fields, table), |pairs| {
                                        println!("-------------------------------");
                                        for &(columns, value) in pairs.iter(){
                                            println!("| {} - {} \t |", columns, value.unwrap());
                                        }
                                        println!("-------------------------------");
                                        true   
                                    }){
                                        Ok(_) => (),
                                        Err(x) => println!("Err: {}", x)
                                    };
                                }
                                "3" => (),// Update
                                "4" => (),// Delete
                                "5" => {// Options
                                    views::table_option();
                                    ctrl = read!("{}\n");
                                    while ctrl != "0"{
                                        match &ctrl[..] {
                                            "1" => {
                                                println!("Table name:");
                                                let table: String = read!("{}\n");
                                                println!("Fields, on SQL style (separated by comma) i.e: Columm1 type, Columm2 type. E.g: ID INTEGER primary_key, name VARCHAR(50)");        
                                                let fields: String = read!("{}\n");
                                                match connection.execute(format!("CREATE TABLE {} ({})", table, fields)){
                                                    Ok(_) => println!("Successfully added"),
                                                    Err(e) => println!("Err {}", e)
                                                }
                                            },
                                            "2" => (),
                                            "3" => (),
                                            "4" => (),
                                            "0" => (),
                                            _ => {
                                                println!("Invalid Option");
                                            }
                                        }
                                        views::table_option();
                                        ctrl = read!("{}\n");
                                    }
    
                                },
                                "0" => (),
                                _ => println!("Opção Invalida!")
                            }
                            views::selected_table(ctrl[..].split("/").last().unwrap().to_string());
                            ctrl = read!("{}\n");
                        }
                    }   
                }
            },
            "3" => {
                println!("--- Delete DB ---");
                println!("What is the number of database that you want to delete, if you wanna cancel type *");
                all_database.print_db();
                ctrl = read!("{}\n");
                fs::remove_file(all_database.db[&ctrl].clone()).expect("Db not found!");
            },
            "4" => all_database.print_db(),
            _ => println!("{}", "Not a valid option!".red())
        }
        views::main_menu();
        ctrl = read!("{}\n");
    }
}
