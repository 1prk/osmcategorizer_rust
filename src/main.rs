mod osm_conditions;
mod assessor;

use osmpbf::{ElementReader, Element, DenseNode};
use benchmark_rs::stopwatch::StopWatch;
use simple_logger::SimpleLogger;
use std::collections::HashMap;
use std::collections::HashSet;
use assessor::Assessor;
use crate::osm_conditions::Conditions;
use geo::{coord};
use csv::Writer;
use itertools::Itertools;

struct WayData {
    id: i64,
    refs: Vec<i64>,
    tags: HashMap<String, String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #![allow(warnings)]
    SimpleLogger::new().env().init().unwrap();
    log::info!("Started assessor");
    let mut stopwatch = StopWatch::new();
    stopwatch.start();
    let path = "./data/leipzig_osm.osm.pbf";

    // ähnlich einer dict mit der osm_id als integer_64, einer weiteren hashmap mit kv-pairs der attribute
    let mut node_bin: HashSet<i64> = HashSet::new();
    let mut ways = Vec::new();

    let reader = ElementReader::from_path(path)?;

    reader.for_each(|element| {
        if let Element::Way(way) = element {
            if !way.tags().any(|(k, _)| k == "highway") &&
                !way.tags().any(|(k, v)| k == "area" && v == "yes") &&
                !way.tags().any(|(k, v)| k == "highway" && v == "platform") {
                return;
            }

            let refs: Vec<i64> = way.refs().collect();
            let tags: HashMap<String, String> = way.tags()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();

            node_bin.extend(&refs);
            ways.push(WayData { id: way.id(), refs, tags });
        }
    })?;

    let reader = ElementReader::from_path(path)?;

    let mut node_coords: HashMap<i64, (f64, f64)> = HashMap::new();
    reader.for_each(|element| {
        if let Element::DenseNode(node) = element {
            if node_bin.contains(&node.id()) {
                let lat = node.lat();
                let lon = node.lon();
                node_coords.insert(node.id(), (lat, lon));
            }
        }
    }).expect("TODO: panic message");

    let mut wtr = Writer::from_path("./data/leipzig_assessed.csv")?;
    wtr.write_record(&["osm_id", "bicycle_infrastructure", "WKT"])?;

    // lese alle nodes aus um geometrien zu sammeln


    // lese alle ways aus mit ihren tags

    for mut way in ways {
        let mut assessor = Assessor { conditions: Conditions::new(&mut way.tags) };
        assessor.assess();

        let way_coords: Vec<_> = way.refs
            .iter()
            .filter_map(|node_id| node_coords.get(node_id).map(|&(lat, lon)| coord! { x: lon, y: lat }))
            .collect();

        if way_coords.is_empty() {
            log::warn!("No valid coordinates found for way_id {}", way.id);
            continue;
        }

        let way_coords_str = format!(
            "LINESTRING ({})",
            way_coords
                .iter()
                .map(|coord| format!("{} {}", coord.x, coord.y))
                .join(", ")
        );

        let infra = way.tags.get("bicycle_infrastructure").map(String::as_str).unwrap_or("");
        wtr.write_record(&[way.id.to_string(), infra.to_string(), way_coords_str])?;
    }

    wtr.flush()?;
    log::info!("Finished assessor, time: {}", stopwatch);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // test für die nossener brücke in DD
    #[test]
    fn assessor_osm() -> Result<(), Box<dyn std::error::Error>> {
        let osm_attributes: HashMap<i64, &str> = HashMap::from([
            (1057362466, "mixed_way_both"),
            (422033331, "bicycle_way_both"),
            (138438926, "bicycle_road"),
            (866753890, "bicycle_lane_right_mit_left"),
            (374714875, "bicycle_lane_both"),
            (994847195, "bicycle_way_right_mit_left"),
        ]);

        let path = "./data/Leipzig_osm.osm.pbf";
        let reader = ElementReader::from_path(path)?;
        let mut ways = Vec::new();
        let mut node_bin: HashSet<i64> = HashSet::new();


        reader.for_each(|element| {
            if let Element::Way(way) = element {
                if let Some(expected) = osm_attributes.get(&way.id()) {
                    let refs: Vec<i64> = way.refs().collect();
                    let tags = way.tags()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect();

                    node_bin.extend(&refs);
                    ways.push((WayData { id: way.id(), refs, tags }, *expected));
                }
            }
        })?;

        for (mut way, expected) in ways {
            let mut assessor = Assessor {
                conditions: Conditions::new(&mut way.tags),
            };
            assessor.assess();

            let actual = way.tags.get("bicycle_infrastructure").map(String::as_str);
            println!("way_id: {:?}, actual: {:?}, expected: {:?}", way.id, actual, expected);
            assert_eq!(actual, Some(expected), "Way {} failed", way.id);
        }

        Ok(())
    }

}
