use std::{error::Error, fs::File, io::BufReader, path::PathBuf};

use geojson::GeoJson;

/// Read the neighborhoods from the geojson file.
///
/// # Panics
/// Panics if the geojson file has an invalid format.
pub fn read_neighborhoods(file: &PathBuf) -> Result<Vec<geojson::Feature>, Box<dyn Error>> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let geojson = GeoJson::from_reader(reader)?;
    let neighborhoods = match geojson {
        GeoJson::FeatureCollection(collection) => collection.features,
        _ => panic!("Invalid geojson file"),
    };

    Ok(neighborhoods)
}
