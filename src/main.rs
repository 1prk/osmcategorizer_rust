use benchmark_rs::stopwatch::StopWatch;
use csv::Writer;
use geo::coord;
use itertools::Itertools;
use osmcategorizer_rust::Assessor;
use osmcategorizer_rust::OsmCategorizerCliArgs;
use osmpbf::{Element, ElementReader};
use simple_logger::SimpleLogger;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Mutex;

struct WayData {
    id: i64,
    refs: Vec<i64>,
    tags: HashMap<String, String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #![allow(warnings)]
    SimpleLogger::new().env().init().unwrap();

    let args: OsmCategorizerCliArgs = argh::from_env();

    log::info!("Started assessor");
    let mut stopwatch = StopWatch::new();
    stopwatch.start();

    if args.export_line_strings() {
        read_ways_with_geoms(args)?;
    } else {
        read_ways(args)?;
    }

    log::info!("Finished assessor, time: {}", stopwatch);
    Ok(())
}

// TODO: Code duplication with read_ways_with_geoms, but well..
fn read_ways(args: OsmCategorizerCliArgs) -> anyhow::Result<()> {
    let mut writer = Writer::from_path(args.output_file())?;
    writer.write_record(&["osm_id", "bicycle_infrastructure", "surface_cat"])?;

    let writer = Arc::new(Mutex::new(writer));

    let reader = ElementReader::from_path(args.input_file())?;

    reader.par_map_reduce(
        |element| {
            let wtr = writer.clone();
            if let Element::Way(way) = element {
                if !way.tags().any(|(k, _)| k == "highway")
                    && !way.tags().any(|(k, v)| k == "area" && v == "yes")
                    && !way.tags().any(|(k, v)| k == "highway" && v == "platform")
                {
                    return;
                }

                let tags: HashMap<String, String> = way
                    .tags()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect();

                let mut assessor = Assessor::new(&tags);
                assessor.assess();

                let mut lock = wtr.lock().expect("Unable to aquire lock");
                lock.write_record(&[
                    way.id().to_string().as_str(),
                    assessor.infrastructure(),
                    assessor.surface_cat().to_string().as_str(),
                ])
                .expect(&format!("Unable to write record for way id {}", way.id()));

                // wtr.write_record(&[way.id().to_string(), infra.to_string()])
                //     .expect(&format!("Unable to write record for way id {}", way.id()));
            }
        },
        || {},
        |_, _| {},
    )?;

    let mut lock = writer.lock().expect("Unable to aquire lock");
    lock.flush().expect("Unable to flush the writer");

    Ok(())
}

fn read_ways_with_geoms(args: OsmCategorizerCliArgs) -> anyhow::Result<()> {
    let mut node_bin: HashSet<i64> = HashSet::new();
    let mut ways = Vec::new();

    let reader = ElementReader::from_path(args.input_file())?;

    reader.for_each(|element| {
        if let Element::Way(way) = element {
            if !way.tags().any(|(k, _)| k == "highway")
                && !way.tags().any(|(k, v)| k == "area" && v == "yes")
                && !way.tags().any(|(k, v)| k == "highway" && v == "platform")
            {
                return;
            }

            let refs: Vec<i64> = way.refs().collect();
            let tags: HashMap<String, String> = way
                .tags()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();

            node_bin.extend(&refs);
            ways.push(WayData {
                id: way.id(),
                refs,
                tags,
            });
        }
    })?;

    let mut node_coords: HashMap<i64, (f64, f64)> = HashMap::new();
    fn insert_node(node_coords: &mut HashMap<i64, (f64, f64)>, id: i64, lat: f64, lon: f64) {
        node_coords.insert(id, (lat, lon));
    }

    let reader = ElementReader::from_path(args.input_file())?;
    reader.for_each(|element| match element {
        Element::Node(node) => {
            if node_bin.contains(&node.id()) {
                insert_node(&mut node_coords, node.id(), node.lat(), node.lon());
            }
        }
        Element::DenseNode(node) => {
            if node_bin.contains(&node.id()) {
                insert_node(&mut node_coords, node.id(), node.lat(), node.lon());
            }
        }

        _ => {}
    })?;

    let mut wtr = Writer::from_path(args.output_file())?;
    wtr.write_record(&["osm_id", "bicycle_infrastructure", "surface_cat", "WKT"])?;

    // lese alle nodes aus um geometrien zu sammeln
    // lese alle ways aus mit ihren tags

    for way in ways {
        let mut assessor = Assessor::new(&way.tags);
        assessor.assess();

        let way_coords: Vec<_> = way
            .refs
            .iter()
            .filter_map(|node_id| {
                node_coords
                    .get(node_id)
                    .map(|&(lat, lon)| coord! { x: lon, y: lat })
            })
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

        let infra = way
            .tags
            .get("bicycle_infrastructure")
            .map(String::as_str)
            .unwrap_or("");
        wtr.write_record(&[
            way.id.to_string().as_str(),
            infra,
            assessor.surface_cat().to_string().as_str(),
            way_coords_str.as_str(),
        ])?;
    }

    wtr.flush()?;

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
                    let tags = way
                        .tags()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect();

                    node_bin.extend(&refs);
                    ways.push((
                        WayData {
                            id: way.id(),
                            refs,
                            tags,
                        },
                        *expected,
                    ));
                }
            }
        })?;

        for (way, expected) in ways {
            let mut assessor = Assessor::new(&way.tags);
            assessor.assess();

            let actual = assessor.infrastructure();
            println!(
                "way_id: {:?}, actual: {:?}, expected: {:?}",
                way.id, actual, expected
            );
            assert_eq!(actual, expected, "Way {} failed", way.id);
        }

        Ok(())
    }
}
