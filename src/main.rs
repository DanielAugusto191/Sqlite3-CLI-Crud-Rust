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
    views::menu(1); 
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
                    "." => continue,
                    _ => {
                        connection = sqlite3::open(all_database.db[&ctrl].clone()).unwrap();
                        views::menu(2);
                        ctrl = read!("{}\n");
                        while ctrl != "0"{
                            match &ctrl[..]{
                                "1" => { // Insert
                                    println!("Table name: (. to cancel)");
                                    let table: String = read!("{}\n");
                                    if table == "." {continue;}
                                    println!("What fields do you want to add: (separated by comma) i.e: name,age,contry");
                                    let fields: String = read!("{}\n");
                                    println!("Value to add: (separated by comma) i.e: 'Daniel',21,'Brazil'");
                                    let values: String = read!("{}\n");
                                    println!("{}", format!("INSERT INTO {} ({}) VALUES ({}) ", table, fields, values));
                                    match connection.execute(format!("INSERT INTO {} ({}) VALUES ({}) ", table, fields, values)){
                                        Ok(_) => println!("Successfully added"),
                                        Err(e) => println!("Err {}", e)
                                    }
                                },
                                "2" => { // Select
                                    println!("Table name: (To cancel type .)");
                                    let table: String = read!("{}\n");
                                    println!("What fields do you want to select: (separated by comma, * = all) i.e: name,age,contry");
                                    let fields: String = read!("{}\n");
                                    println!("SELECT {} from {}", fields, table); //DBG
                                    match connection.iterate(format!("SELECT {} FROM {}", fields, table), |pairs| {
                                        println!("-------------------------------");
                                        for &(columns, value) in pairs.iter(){
                                            if value == None {continue;}
                                            println!("| {} - {} \t |", columns, value.unwrap());
                                        }
                                        println!("-------------------------------");
                                        true   
                                    }){
                                        Ok(_) => (),
                                        Err(x) => println!("Err: {}", x)
                                    };
                                }
                                "3" => { // Update
                                    println!("Table name to update");
                                    let table: String = read!("{}\n");
                                    println!("Fields and values to update. I.e: name = 'Daniel', age = 21");
                                    let n_values: String = read!("{}\n");
                                    println!("Conditions to update. I.e: ID = 5");
                                    let condition: String = read!("{}\n");
                                    match connection.execute(format!("UPDATE {} SET {} WHERE {}", table, n_values, condition)) {
                                        Ok(_) => println!("Sucessfully updated!"),
                                        Err(e) => println!("Err: {}", e)
                                    }
                                },
                                "4" => { // Delete
                                    println!("Table name:");
                                    let table: String = read!("{}\n");
                                    println!("Conditions do delete value. Ex: ID = 5");
                                    let condition: String = read!("{}\n");
                                    match connection.execute(format!("DELETE FROM {} WHERE {}", table, condition)){
                                        Ok(_) => println!("Successfully deleted {}", table),
                                        Err(e) => println!("Failed to delete :(, Err: {}", e)
                                    }

                                },
                                "5" => {// Options
                                    views::menu(3);
                                    ctrl = read!("{}\n");
                                    while ctrl != "0"{
                                        match &ctrl[..] {
                                            "1" => {
                                                println!("Table name:");
                                                let table: String = read!("{}\n");
                                                println!("Fields, in SQL style (separated by comma) i.e: Columm1 type, Columm2 type. E.g: ID INTEGER PRIMARY KEY NOT NULL AUTOINCREMENT, name VARCHAR(50)");        
                                                let fields: String = read!("{}\n");
                                                match connection.execute(format!("CREATE TABLE {} ({});", table, fields)){
                                                    Ok(_) => println!("Successfully added"),
                                                    Err(e) => println!("Err {}", e)
                                                }
                                            },
                                            "2" => {
                                                println!("-----------TABLES-------------");
                                                let _ = connection.iterate("SELECT name FROM sqlite_schema WHERE type='table' AND name NOT LIKE 'sqlite_%';", |table| {
                                                    for &(_, b) in table.iter(){
                                                        println!("|{} \t |", b.unwrap());
                                                    }
                                                    true
                                                });
                                                println!("-------------------------------");
                                            },
                                            "3" => {
                                                println!("Table name to delete: ");
                                                let table: String = read!("{}\n");
                                                match connection.execute(format!("DROP TABLE IF EXISTS {}", table)) {
                                                    Ok(_) => println!("{} droped.", table),
                                                    Err(e) => println!("Err: {}", e)
                                                }
                                            },
                                            "4" => {
                                                println!("Column name to delete: ");
                                                let table: String = read!("{}\n");
                                                println!("-----------Columns-------------");
                                                let _ = connection.iterate(format!("SELECT sql FROM sqlite_master
                                                WHERE tbl_name = '{}' AND type = 'table'", table), |table| {
                                                    for &(_, b) in table.iter(){
                                                        println!("|{} \t |", b.unwrap());
                                                    }
                                                    true
                                                });
                                                println!("-------------------------------");
                                            },
                                            "0" => (),
                                            _ => {
                                                println!("Not a valid option!");
                                            }
                                        }
                                        views::menu(3);
                                        ctrl = read!("{}\n");
                                    }
    
                                },
                                "0" => (),
                                _ => println!("Not a valid option!")
                            }
                            views::menu(2);
                            ctrl = read!("{}\n");
                        }
                    }   
                }
            },
            "3" => {
                println!("--- Delete DB ---");
                println!("What is the number of database that you want to delete, if you wanna cancel type .");
                all_database.print_db();
                ctrl = read!("{}\n");
                if ctrl == "." {continue;}
                fs::remove_file(all_database.db[&ctrl].clone()).expect("Db not found!");
            },
            "4" => all_database.print_db(),
            _ => println!("{}", "Not a valid option!".red())
        }
        views::menu(1);
        ctrl = read!("{}\n");
    }
}
