mod osm_conditions;
mod assessor;

use osmpbf::{ElementReader, Element};

use std::collections::HashMap;
use assessor::Assessor;
use crate::osm_conditions::Conditions;
use geo::{coord};
use csv::Writer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "data/Leipzig_osm.osm.pbf";

    // ähnlich einer dict mit der osm_id als integer_64, einer weiteren hashmap mit kv-pairs der attribute
    let mut node_coords: HashMap<i64, (f64, f64)> = HashMap::new();

    let reader = ElementReader::from_path(path)?;

    let mut wtr = Writer::from_path("./data/leipzig_assessed.csv")?;
    wtr.write_record(&["osm_id", "bicycle_infrastructure", "WKT"])?;

    // lese alle nodes aus um geometrien zu sammeln
    reader.for_each(|element| {
        if let Element::DenseNode(node) = element {
            let lat = node.lat();
            let lon = node.lon();
            node_coords.insert(node.id(), (lat, lon));
        }
    })?;

    // lese alle ways aus mit ihren tags
    let reader = ElementReader::from_path(path)?; // Re-open the reader

    reader.for_each(|element| {
        if let Element::Way(way) = element {
            if !way.tags().any(|(k, _)| k == "highway") &&
                !way.tags().any(|(k, v)| k == "area" && v == "yes") &&
                !way.tags().any(|(k, v)| k == "highway" && v == "platform"){
                return;
            }


            // way-tags sammeln
            let mut tags: HashMap<_, _> = way.tags()
                .map(|(key, value)| (key.to_owned(), value.to_owned()))
                .collect();

            // assessor initialisieren und starten
            let mut assessor = Assessor { conditions: Conditions { tags: &mut tags }};
            assessor.assess();

            // sammel referenznodes der ways und gib die koordinaten des ways an
            let mut way_coords = Vec::new();
            for node_id in way.refs() {
                if let Some(&(lat, lon)) = node_coords.get(&node_id) {
                    // bereite node-koordinaten in LineString-Format vor
                    way_coords.push(coord! { x: lon, y: lat });
                } else {
                    // fallback
                    println!("ref node_id {} not found for way_id {}", node_id, way.id());
                }
            }
            // wandel die koordinaten in eine LineString-Repräsentation für csv-ausgabe aus
            let way_coords_str = format!(
                "LINESTRING ({})",
                way_coords
                    .iter()
                    .map(|coord| format!("{} {}", coord.x, coord.y))
                    .collect::<Vec<_>>()
                    .join(", ")
            );

            // wandel die assesste infrastruktur in einen string um. falls None, dann leeres zeichen.
            let infra = if let Some(infrastructure) = tags.get("bicycle_infrastructure") {
                infrastructure.clone()
            } else {
                "".to_string()
            };
            // schreibe das ganze in eine csv.
            wtr.write_record(&[way.id().to_string(), infra.to_string(), way_coords_str]);

        }
    })?;
    // lösche den writer-zwischenspeicher
    wtr.flush()?;
    Ok(())
}
