use std::collections::HashMap;
use std::iter::{IntoIterator, Iterator};
use chrono::{Datelike, Timelike, Utc, DateTime, NaiveTime, NaiveDateTime, NaiveDate, Local};
use serde::{Deserialize, Serialize};
use serde_json;


pub struct Puck;

impl Puck {

    pub fn register(&mut self, io: String, mut hour: String, mut date: String) {
        let mut data = UserData::new().expect("Erro");

        let now = Local::now();

        if hour.eq("") {
            hour.insert_str(
                0,
                format!(
                    "{:02}:{:02}", now.hour(), now.minute()
                ).as_str()
            );
        }

        if date.eq(&"".to_string()) {
            date.insert_str(
                0,
                format!(
                    "{:02}-{:02}-{:?}", now.day(), now.month(), now.year()
                ).as_str()
            );
        }
        data.registers.entry(date.clone()).or_insert(HashMap::new()).entry(hour).or_insert(io);

        println!("Teste: {:?}", data.registers);
        data.save();
    }

}

#[derive(Deserialize, Serialize)]
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

    fn save(&mut self) -> Result<(), std::io::Error> {
        // -> Result<UserData, std::io::Error>
        println!("Writing on file.");
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(".db.json")?;
        // write to file with actual data.
        serde_json::to_writer_pretty(file, &self);
        
        Ok(())
        
    }
}
