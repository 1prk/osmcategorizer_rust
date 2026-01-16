# OSM Categorizer

A Rust-based tool for analyzing OpenStreetMap data to categorize bicycle infrastructure and surface types from OSM PBF files.

## Description

This tool processes OpenStreetMap `.osm.pbf` files and analyzes highway features to:
- Classify bicycle infrastructure (lanes, paths, mixed ways, etc.)
- Categorize surface types
- Optionally export geometries as WKT linestrings

The tool uses parallel processing for efficient handling of large OSM datasets.

## Prerequisites

- Rust toolchain (rustc and cargo)
- Install from [https://rustup.rs/](https://rustup.rs/)

## Building

Build the project in release mode for optimal performance:

```bash
cargo build --release
```

The compiled binary will be located at `target/release/osmcategorizer_rust.exe` (Windows) or `target/release/osmcategorizer_rust` (Linux/macOS).

## Usage

### Basic Usage

```bash
osmcategorizer_rust -i <input.osm.pbf> -o <output.csv>
```

### With Geometry Export

To include WKT linestring geometries in the output:

```bash
osmcategorizer_rust -i <input.osm.pbf> -o <output.csv> -g
```

### Command Line Arguments

- `-i, --input-file <PATH>` - Path to the input OSM PBF file (required)
- `-o, --output-file <PATH>` - Path to the output CSV file (required)
- `-g, --export-line-strings` - Export way geometries as WKT linestrings (optional)

### Example

```bash
# Analyze Leipzig bicycle infrastructure
cargo run --release -- -i data/Leipzig_osm.osm.pbf -o leipzig_assessed.csv

# Analyze with geometries
cargo run --release -- -i data/sachsen-latest.osm.pbf -o sachsen_assessed.csv -g
```

## Output Format

The tool generates a CSV file with the following columns:

### Without geometries (`-g` flag omitted):
- `osm_id` - OpenStreetMap way ID
- `bicycle_infrastructure` - Categorized bicycle infrastructure type
- `surface_cat` - Surface category (numerical)

### With geometries (`-g` flag included):
- `osm_id` - OpenStreetMap way ID
- `bicycle_infrastructure` - Categorized bicycle infrastructure type
- `surface_cat` - Surface category (numerical)
- `WKT` - Way geometry as WKT LINESTRING

## Running Tests

```bash
cargo test
```

## Performance

The tool uses parallel processing to efficiently handle large OSM datasets. Processing time depends on file size and whether geometry export is enabled.

## License

[Add your license here]
