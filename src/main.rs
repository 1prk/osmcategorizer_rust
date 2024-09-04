mod osm_conditions;
mod assessor;

use osmpbf::{ElementReader, Element};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use osm_conditions::Conditions;
use assessor::Assessor;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "data/Leipzig_osm.osm.pbf";

    // ähnlich einer dict mit der osm_id als integer_64, einer weiteren hashmap mit kv-pairs der attribute
    let mut osm_ways: HashMap<i64, HashMap<String, String>> = HashMap::new();


    let reader = ElementReader::from_path(path)?;

    reader.for_each(|element| {
        if let Element::Way(way) = element {
            if !way.tags().any(|(key, _)| key == "highway") {
                return;
            }
            // hier werden die key-value-paare gespeichert
            let mut tags: HashMap<_, _> = way.tags()
                .map(|(key, value)| (key.to_owned(), value.to_owned()))
                .collect();
            let conditions = Conditions { tags: &mut tags };
            let mut assessor = Assessor { conditions, tags: &mut tags };
            let assessor_infra = assessor.assess(conditions);

            // for (key, value) in way.tags() {
            //     tags.insert(key.to_owned(), value.to_owned());
            // }
            //osm_ways.insert(way.id(), tags);
            //let segregated = conditions.is_segregated();
            //let footpath = conditions.is_footpath();
            //let bikepath_left = conditions.is_bikepath("left");
            //let bikepath_right = conditions.is_bikepath("right");
            //if footpath || segregated {
            //    tags.insert("bicycle_infrastructure".to_string(), "bicycle_way".to_string());
            //}
            //else {
            //    tags.insert("bicycle_infrastructure".to_string(), "None".to_string());
            //}
            //println!("osm_id {} is segregated: {}, is footpath: {}, is bikepath left: {}, is bikepath right: {}", way.id(), segregated, footpath, bikepath_left, bikepath_right);
            println!("assigned {:?} to osm_id {}", assessor_infra, way.id());
        }

    })?;


    // let to_find = [135754172, 135754148];
    // for &osm_id in &to_find {
    //     match osm_ways.get(&osm_id) {
    //         Some(value) => println!("osm_id: {}, key: {}, value: {}", osm_id, value.0, value.1),
    //         None => println!("No key-value pairs!")
    //     }
    // }


    Ok(())
}
