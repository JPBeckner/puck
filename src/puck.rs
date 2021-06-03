use std::collections::HashMap;
use std::iter::{IntoIterator, Iterator};
use chrono::{Datelike, Timelike, Utc, DateTime, NaiveTime, NaiveDateTime, NaiveDate};
use serde::{Deserialize};


pub struct Puck;

impl Puck {

    pub fn register(&mut self, io: String, hour: String, mut date: String) {
        let mut data = UserData::new().expect("Erro");

        // data.registers.insert(date, HashMap::from_iter());
        // let date_like = NaiveDateTime::new(date: NaiveDate::from_str(s: &str)"), time: NaiveTime);
        if date.eq(&"".to_string()) {
            date.insert_str(0, "01-06-2021");
        }
        // let new_register: HashMap<String, String> = [(hour, io)].iter().cloned().collect();
        data.registers.entry(date.clone()).or_insert(HashMap::new()).entry(hour).or_insert(io);

        // if !data.registers.contains_key(&date) {
        //     let new_register: HashMap<String, String> = [(hour, io)].iter().cloned().collect();
        //     data.registers.insert(date.clone(), HashMap::new());
        // }
        
        // if data.registers.get(&date).unwrap_or(&HashMap::default()).eq(&HashMap::default()) {
        //     data.registers.insert(date.clone(), HashMap::default());
        // }
        // data.registers.get(&date.to_string()).unwrap();
        println!("Teste: {:?}", data.registers);
        // println!("io: {:?}, hour: {:?}, user {:?}, some register: {:?}", io, hour, data.user, data.registers["01-06-2021"]);
    }

}

#[derive(Deserialize)]
struct UserData {
    user: String,
    registers: HashMap<String, HashMap<String, String>>
}

impl UserData {
    fn new() -> Result<UserData, std::io::Error> {
        // open db.json
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(".db.json")?;

        let full_data: UserData = serde_json::from_reader(file).unwrap();
        Ok(full_data)

    }

    fn save(&mut self, key: String, value: String) -> Result<UserData, std::io::Error> {
        println!("Writing on file.");
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(".db.json")?;
        
        Ok()
        
    }
}
