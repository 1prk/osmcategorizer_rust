use std::path::{Path, PathBuf};

use argh::FromArgs;

#[derive(FromArgs)]
/// Reach new heights.
pub struct OsmCategorizerCliArgs {
    /// path to the input osm.pbf file
    #[argh(option, short = 'i')]
    input_file: PathBuf,

    /// path to the output csv file
    #[argh(option, short = 'o')]
    output_file: PathBuf,

    /// export way geometries as linestrings
    #[argh(switch, short = 'g')]
    export_line_strings: bool,
}

impl OsmCategorizerCliArgs {
    /// Path to the input osm.pbf file
    pub fn input_file(&self) -> &Path {
        &self.input_file
    }

    /// Path to the output csv file
    pub fn output_file(&self) -> &Path {
        &self.output_file
    }

    /// Export way geometries as linestrings
    pub fn export_line_strings(&self) -> bool {
        self.export_line_strings
    }
}
