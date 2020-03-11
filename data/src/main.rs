//tutorial-error-04.rs
extern crate csv;

use std::error::Error;
use std::io;
use std::process;
use std::collections::BTreeMap;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn check_not_blank(option: Option<&str>) -> Result<&str, Box<dyn Error>> {
    match option {
        Some("") => Err(From::from("Blank string")),
        Some(s) => Ok(s),
        None => Err(From::from("No string"))
    }
}

fn check_name(option: Option<&str>, valid: &[&str]) -> Result<String, Box<dyn Error>> {
    match option {
        Some("") => Err(From::from("Blank string")),
        Some(s) => {
            for prefix in valid {
                if s.starts_with(prefix) {
                    return Ok(s.to_string());
                }
            }
            return Err(From::from("Not edible"));
        },
        None => Err(From::from("No string"))
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let names = ["2015.csv", "2016.csv", "2017.csv", "2018.csv", "2019.csv"];
    let mut species = BTreeMap::new();
    let valid = ["Morchella", "Cantharellus", "Hydnum", "Grifola frondosa", "Hericium", "Calvatia gigantea", "Laetiporus sulphureus", "Craterellus cornucopioides", "Boletus", "Clitocybe nuda", "Lactarius", "Pleurotus ostreatus", "Hypomyces lactifluorum"];

    let mut writer = csv::Writer::from_path("output.csv")?;
    writer.write_record(&["date", "name", "latitude", "longitude"])?;
    for name in &names {
        let mut rdr = csv::Reader::from_path(name)?;
        let header_record = rdr.headers()?;
        // println!("{:?}", header_record);
        // Iterate through the header_record and find the column indices with the right title
        // "image_url", "scientific_name"

        // This isn't really ideal. enumerate() makes i into a usize, which means the starting values of the indices
        // cannot be < 0. 
        let mut observed_on_index = 0;
        let mut scientific_name_index = 0;
        let mut latitude_index = 0;
        let mut longitude_index = 0;

        for (i, header_name) in header_record.iter().enumerate() {
            if header_name == "observed_on" {
                observed_on_index = i;
            }
            else if header_name == "scientific_name" {
                scientific_name_index = i;
            }
            else if header_name == "latitude" {
                latitude_index = i;
            }
            else if header_name == "longitude" {
                longitude_index = i;
            }

        }


        for result in rdr.records() {

            // This is effectively the same code as our `match` in the
            // previous example. In other words, `?` is syntactic sugar.
            // It replaces the following commented out code.
            let record = result?;
            // let record = match result {
            //     Err(err) => return Err(From::from(err)),
            //     Ok(r) => r
            // };

            let mut finished = || -> Result<(), Box<dyn Error>> {
                let observed_on: &str = check_not_blank(record.get(observed_on_index))?;

                let scientific_name: &str = &check_name(record.get(scientific_name_index), &valid)?;

                let latitude: &str = check_not_blank(record.get(latitude_index))?;

                let longitude: &str = check_not_blank(record.get(longitude_index))?;

                // Need to use to_string to make a copy of the scientific name.
                // Otherwise 
                if species.contains_key(scientific_name) {
                    species.insert(scientific_name.to_string(), 1 + species[scientific_name]);
                }
                else {
                    species.insert(scientific_name.to_string(), 1);
                }
                writer.write_record(&[observed_on, scientific_name, latitude, longitude])?;
                Ok(())
            };
            if let Err(_err) = finished() {
                if _err.description() == "Blank string" {
                    continue;
                }
                else if _err.description() == "Not edible" {
                    continue;
                }
                else {
                    return Err(_err);
                }
            }
        }
    }

    for (k, v) in &species {
        println!("{}, {}", k, v);
    }

    Ok(())
}