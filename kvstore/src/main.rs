// follow these videos: https://www.youtube.com/watch?v=WnWGO-tLtLA | https://www.youtube.com/watch?v=lLWchWTUFOQ

use std::collections::HashMap;
use std::result::Result;
use std::path::Path;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("Key was not there");
    let value = arguments.next().expect("Value was not there");
    println!("The key is '{}' and the value is '{}'", key, value);
    let contents = format!("{}\t{}\n", key, value);
    std::fs::write("kv.db", contents).unwrap();

    let database = Database::new().expect("Creating db failed");
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let database_name = "kv.db";

        // check if the file exists otherwise created it
        if !Path::new(database_name).exists() {
            std::fs::write(database_name, "")?;
        }
        
        // read the kv.db file
        // let contents = match std::fs::read_to_string(database_name) {
        //     Ok(c) => c,
        //     Err(error) => {
        //         return Err(error);
        //     }
        // };
        let contents = std::fs::read_to_string(database_name)?;

        let mut map = HashMap::new();
        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupt database: no key, no value");
            map.insert(key.to_owned(), value.to_owned());
        }

        return Ok(Database{ map: map });
    }
}