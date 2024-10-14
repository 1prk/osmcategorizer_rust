mod osm_conditions;
mod assessor;

use osm_io::osm::model::element::Element;
use osm_io::osm::pbf;
use osm_io::osm::pbf::compression_type::CompressionType;
use osm_io::osm::pbf::file_info::FileInfo;

use std::collections::HashMap;
use assessor::Assessor;
use crate::osm_conditions::Conditions;

use std::path::PathBuf;

use anyhow;
// use benchmark_rs::stopwatch::StopWatch;
// use simple_logger::SimpleLogger;

pub fn main() -> Result<(), anyhow::Error> {
    // SimpleLogger::new().init()?;
    log::info!("Started pbf io pipeline");
    // let mut stopwatch = StopWatch::new();
    // stopwatch.start();
    let input_path = PathBuf::from("./data/Leipzig_osm.osm.pbf");
    let output_path = PathBuf::from("./data/Leipzig_osm_bicycle_infra.osm.pbf");
    let reader = pbf::reader::Reader::new(&input_path)?;
    let mut file_info = FileInfo::default();
    file_info.with_writingprogram_str("pbf-io-example");
    let mut writer = pbf::writer::Writer::from_file_info(
        output_path,
        file_info,
        CompressionType::Zlib,
    )?;

    writer.write_header()?;

    for element in reader.elements()? {
        if let Element::Way { way } = &element {
            if way.tags().any(|tag| tag.k() == "highway") {
                let mut tags: HashMap<String, String> = way.tags().
                    map(|tag| (tag.k().to_owned(), tag.v().to_owned())).
                    collect();

                let mut assessor = Assessor {
                    conditions: Conditions { tags: &mut tags },
                };
                assessor.assess();

                writer.write_element(Element::Way { way: way })?;
                continue;
            }
        }
        writer.write_element(element)?;
    }

    writer.close()?;

    // log::info!("Finished pbf io pipeline, time: {}", stopwatch);
    Ok(())
}