#![allow(dead_code)]
#![allow(clippy::unnecessary_unwrap)]
#![allow(clippy::let_and_return)]

//use chrono::{DateTime, Datelike, Local, Utc};
use chrono::{ Datelike, Local};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct PersonRecord {
    name: String,
    year: i32,
    location: String
}

impl PersonRecord {

    fn age(&self) -> i32 {
        let _t = Local::now();
        (_t.year() - self.year) as i32
    }

}

fn show_personrecord(data: Vec<PersonRecord>, _searchedname: String ) /*-> Vec<String>*/ {
    let len = data.len();
    let mut _age: i32 = 0;
    println!("PersonRecord of {}", _searchedname);
    println!("No Name     Age      Location    since");
    for i in 0..len {
        let entry = data[i].clone();
        //println!("{:8}, {:8}, {:?}", entry.name, _searchedname, entry.name.eq(&_searchedname));
        if entry.name == _searchedname && _age == 0 {
            _age = data[i].age();
        }
        if entry.name == _searchedname {
            println!("{:2} {:8} {:3}      {:9}   {:4}", i, entry.name, _age, entry.location, entry.year);
        }
    }
}

fn main() {
    
    let mut _peoplein: Vec<PersonRecord> = Vec::new();
    let mut _names: Vec<String> = Vec::new();

    // initial data for different persons
    let _sam: PersonRecord = PersonRecord { name: "Sam".to_string(), year: 1973, location: "Koeln".to_string() };
    let _varad = PersonRecord { name: "Varad".to_string(), year: 1996, location: "Augsburg".to_string() };
    let _bernd = PersonRecord { name: "Bernd".to_string(), year: 1977, location: "Wuerzburg".to_string() };
    _names.push(_sam.name.clone());
    _names.push(_varad.name.clone());
    _names.push(_bernd.name.clone());

    // add some history for each person
    let mut _sam_copy = _sam.clone();
    let mut _varad_copy = _varad.clone();
    let mut _bernd_copy = _bernd.clone();
    _peoplein.push(_sam);
    _peoplein.push(_varad);
    _varad_copy.location = "Manching".to_string();
    _varad_copy.year = 1998;
    _peoplein.push(_bernd);
    _sam_copy.location = "Ulm".to_string();
    _sam_copy.year = 2023;
    _bernd_copy.location = "Muenchen".to_string();
    _bernd_copy.year = 1996;
    _peoplein.push(_sam_copy.clone());
    _peoplein.push(_varad_copy.clone());
    _peoplein.push(_bernd_copy.clone());
    _varad_copy.location = "Neuburg".to_string();
    _varad_copy.year = 2007;
    _peoplein.push(_varad_copy.clone());
    
    // sort peoplein_ by name and print
    _peoplein.sort_by_key(|k| k.name.to_string());
    let len = _peoplein.len();
    println!("Number of total entries in peoplein_ : {}", len);
    // display available persons
    println!("You can search for:");
    for entry in _names{
        println!("{}", entry);
    }

    let mut _run: bool = true;
    while _run {
        // ask user for search person
        let mut _searchedname = String::new();
        println!("Which person do you search?");
        let _= std::io::stdout().flush();
        std::io::stdin().read_line(&mut _searchedname).expect("error: unable to read user input");
        _searchedname = _searchedname.trim_end().to_string();
        println!("");
    
        // show entries of searched person
        show_personrecord(_peoplein.clone(), _searchedname.clone());
        println!("");
        // ask user for interaction
        let mut _input: String = String::new();
        println!("Do you want to add [1] or remove [2] and entry?");
        let _= std::io::stdout().flush();
        std::io::stdin().read_line(&mut _input).expect("error: unable to read user input");
        _input = _input.trim_end().to_string();
        println!("");
        
        match _input.as_str() {
            "1" => {  println!("Add entry - Enter location:");
                      let mut _loc: String = String::new();
                      std::io::stdin().read_line(&mut _loc).expect("error: unable to read user input");
                      _loc = _loc.trim_end().to_string();
                      println!("Add entry - Enter year:");
                      let mut _yyyy: String = String::new();
                      std::io::stdin().read_line(&mut _yyyy).expect("error: unable to read user input");
                      _yyyy = _yyyy.trim_end().to_string();
                      let _int_year: i32 = _yyyy.parse().expect("Failed to parse string to integer");
                      let newentry: PersonRecord = PersonRecord { name: _searchedname.clone(), year: _int_year, location: _loc };
                      _peoplein.push(newentry);
                      show_personrecord(_peoplein.clone(), _searchedname.clone());
                    },
            "2" => {  println!("Remove entry - Enter number (No) to be deleted:");
                      let mut _pos: String = String::new();
                      std::io::stdin().read_line(&mut _pos).expect("error: unable to read user input");
                      _pos = _pos.trim_end().to_string();
                      let no: usize = _pos.parse().expect("Failed to parse string to integer");
                      _peoplein.remove(no);
                      show_personrecord(_peoplein.clone(), _searchedname.clone());
                  },
            _ => {  println!("Exit");
                    _run = false;
                }
        }
        println!("Do you want to continue y|n?");
        let mut _continue: String = String::new();
        std::io::stdin().read_line(&mut _continue).expect("error: unable to read user input");
        _continue = _continue.trim_end().to_string();
        match _continue.as_str() {
            "y" => _run = true,
            "n" => _run = false,
            _ => _run = false
        }
    }
    
}
