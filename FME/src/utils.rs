use std::path::Path;
use serde::de::DeserializeOwned;
use csv::Reader;

pub fn read_file<T:DeserializeOwned>(path: &str) -> Vec<T> {
    let file_path = Path::new(path);
    let mut rdr = Reader::from_path(file_path).expect("File should exist");
    let mut records: Vec<T> = Vec::new();

    for result in rdr.deserialize() {
        let record: T = result.expect("File should be formatted correctly");
        records.push(record);
    }
    return records
}

