use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use std::error::Error;
use std::{io, process};


#[derive(Serialize, Deserialize, Clone)]
struct MixShoesData {
    email: String,
    age: String,
    interested: bool,
    know: bool,
    preference: i32,
    cities: HashSet<String>,
    marks: HashSet<String>,
    colors: HashSet<String>,
    sizes: HashSet<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
struct MixShoesDataCommonCriteria {
    email: String,
    marks: Vec<String>,
    cities: Vec<String>,
    colors: Vec<String>,
    common_marks: Vec<String>,
    common_cities: Vec<String>,
    common_colors: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct MixShoesDataSame {
    email: String,
    marks: Vec<String>,
    cities: Vec<String>,
    colors: Vec<String>,
    common_criteria: Vec<MixShoesDataCommonCriteria>,
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let json_data : Vec<MixShoesData> = serde_json::from_reader(io::stdin()).expect("JSON was not well-formatted");
    let json_data_inloop = json_data.clone();
    let mut results : Vec<MixShoesDataSame> = Vec::new();

    for current_data in json_data.iter().filter(|cd| cd.interested).collect::<Vec<&MixShoesData>>() {
        let mut current_ok = MixShoesDataSame { 
            email: current_data.email.clone(),
            marks: current_data.marks.iter().cloned().collect(),
            cities: current_data.cities.iter().cloned().collect(),
            colors: current_data.colors.iter().cloned().collect(),
            common_criteria:Vec::new() 
        };

        for other_data in json_data_inloop.iter().filter(|od| od.email != current_data.email && od.interested).collect::<Vec<&MixShoesData>>() {
            let common_sizes = current_data.sizes.intersection(&other_data.sizes).collect::<Vec<_>>();
            let common_marks = current_data.marks.intersection(&other_data.marks).collect::<Vec<_>>();
            let common_cities = current_data.cities.intersection(&other_data.cities).collect::<Vec<_>>();
            let common_colors = current_data.colors.intersection(&other_data.colors).collect::<Vec<_>>();

            if common_sizes.len() > 0 {
                let other_ok = MixShoesDataCommonCriteria {
                    email: other_data.email.clone(),
                    marks: other_data.marks.iter().cloned().collect(),
                    cities: other_data.cities.iter().cloned().collect(),
                    colors: other_data.colors.iter().cloned().collect(),
                    common_marks: common_marks.iter().map(|s| String::from(*s)).collect(),
                    common_cities: common_cities.iter().map(|s| String::from(*s)).collect(),
                    common_colors: common_colors.iter().map(|s| String::from(*s)).collect()
                };

                current_ok.common_criteria.push(other_ok);
                println!("\t- {:?}\n\t\tMarques : {:?}\n\t\tVilles : {:?}", other_data.email, common_marks, common_cities);
            }
        }
        results.push(current_ok);
    }


    std::fs::write(
        "result_common.json",
        serde_json::to_string_pretty(&results).unwrap(),
    )?;

    Ok(())
}
