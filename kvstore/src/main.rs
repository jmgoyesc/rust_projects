// follow these videos: https://www.youtube.com/watch?v=WnWGO-tLtLA | https://www.youtube.com/watch?v=lLWchWTUFOQ

use std::collections::HashMap;
use std::result::Result;
use std::path::Path;

const DATASOURCE: &str = "kv.db";

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("Key was not there");
    let value = arguments.next().expect("Value was not there");
    println!("The key is '{}' and the value is '{}'", key, value);
    // let contents = format!("{}\t{}\n", key, value);
    // std::fs::write("kv.db", contents).unwrap();

    let mut database = Database::new().expect("Creating db failed");
    database.insert(key.clone(), value.clone());
    database.insert(key.to_uppercase(), value);
    // database.flush().expect("Unable to flush database to file");
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // check if the file exists otherwise created it
        if !Path::new(DATASOURCE).exists() {
            std::fs::write(DATASOURCE, "")?;
        }
        
        // read the kv.db file
        let contents = std::fs::read_to_string(DATASOURCE)?;

        let mut map = HashMap::new();
        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupt database: no key, no value");
            map.insert(key.to_owned(), value.to_owned());
        }

        return Ok(Database{ map });
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

}

impl Drop for Database {
    fn drop(&mut self) {
        let mut contents = String::new();
        for (key, value) in &self.map {
            // let kvpair = format!("{}\t{}\n", key, value);
            // contents.push_str(&kvpair);
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }
        let _ = std::fs::write(DATASOURCE, contents);
    }
}