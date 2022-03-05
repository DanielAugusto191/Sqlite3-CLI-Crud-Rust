use sqlite3;
use text_io::{read};
use colored::*;

fn main() {
    let line: String = read!("{}\n");
    println!("{}", line.blue());
    let connection = sqlite3::open("new").unwrap();
    match connection.execute("CREATE TABLE IF NOT EXISTS users (name TEXT, age INTEGER primary_key)",){
        Ok(_) => println!("Criada com Sucesso!"),
        Err(_) => println!("Algo deu errado ao criar a tabela!")
    }
    connection.iterate("SELECT * FROM users WHERE age > 50", |pairs| {
        for &(column, value) in pairs.iter() {
            println!("{} = {}", column, value.unwrap());
        }
        true
    }).unwrap();
    connection.iterate("SELECT * FROM users WHERE age > 50", |pairs| {
        for &(column, value) in pairs.iter() {
            println!("{} = {}", column, value.unwrap());
        }
        true
    }).unwrap();
    let mut v: Vec<String> = Vec::new();
    let k = connection.iterate("SELECT * FROM users WHERE age > 50", |pairs| {
        for &(column, value) in pairs.iter() {
            v.push(format!("{} {}", column, value.unwrap()));
        }
        true
    });
}
