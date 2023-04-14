use geo::algorithm::contains::Contains;
use geo_types::{Coord, Geometry, LineString, Polygon};
use geojson::{Feature, GeoJson};
use std::{collections::HashMap, error::Error, fs::File, io::BufReader, path::PathBuf};

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

pub const BASE_32: [char; 32] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'j', 'k',
    'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub fn children(gh: &String) -> Vec<String> {
    BASE_32.iter().map(|c| format!("{}{}", gh, c)).collect()
}

pub fn bbox(gh: &str) -> Option<Polygon<f64>> {
    if gh == "" {
        let min = Coord::<f64>::from((-180.0, -90.0));
        let max = Coord::<f64>::from((180.0, 90.0));
        return Some(geo_types::Rect::new(min, max).to_polygon());
    }
    match geohash::decode_bbox(gh) {
        Ok(rect) => {
            let bl = rect.min();
            let tr = rect.max();
            let outer = LineString(vec![
                Coord::from((bl.x, bl.y)),
                Coord::from((tr.x, bl.y)),
                Coord::from((tr.x, tr.y)),
                Coord::from((bl.x, tr.y)),
                Coord::from((bl.x, bl.y)),
            ]);
            Some(Polygon::new(outer, Vec::new()))
        }
        _ => None,
    }
}

pub fn contains(outer: &Polygon<f64>, inner: &Geometry<f64>) -> bool {
    match *inner {
        Geometry::Point(ref g) => outer.contains(g),
        Geometry::Line(ref g) => outer.contains(g),
        Geometry::LineString(ref g) => outer.contains(g),
        Geometry::Polygon(ref g) => outer.contains(g),
        Geometry::Rect(ref g) => outer.contains(&g.to_polygon()),
        Geometry::Triangle(ref g) => outer.contains(&g.to_polygon()),
        Geometry::MultiPoint(ref mp) => mp.0.iter().all(|p| outer.contains(p)),
        Geometry::MultiLineString(ref mls) => mls.0.iter().all(|ls| outer.contains(ls)),
        Geometry::MultiPolygon(ref mp) => mp.0.iter().all(|poly| outer.contains(poly)),
        Geometry::GeometryCollection(ref gc) => gc.0.iter().all(|geom| contains(outer, geom)),
    }
}

pub fn covering(geom: &Geometry<f64>, level: usize) -> Vec<String> {
    use geo::algorithm::intersects::Intersects;
    let mut ghs: Vec<String> = vec![];
    let mut queue: Vec<String> = vec!["".to_string()];
    while !queue.is_empty() {
        let gh = queue.pop().unwrap();
        match bbox(&gh) {
            Some(poly) => {
                if contains(&poly, &geom) || poly.intersects(geom) {
                    if gh.len() < level {
                        queue.extend(children(&gh));
                    } else {
                        ghs.push(gh);
                    }
                }
            }
            None => (),
        }
    }
    ghs
}

/// Get a map of geohashes for each neighborhood.
///
/// # Panics
/// Panics if a feature has an invalid geometry.
pub fn get_geohashes_map_from_features(features: &Vec<Feature>) -> HashMap<String, Vec<String>> {
    let mut geohashes_map: HashMap<String, Vec<String>> = HashMap::new();
    for feature in features {
        let geometry = Geometry::try_from(&feature.geometry.clone().unwrap()).unwrap();
        let covering_geohashes = covering(&geometry, 6);
        geohashes_map.insert(
            feature
                .properties
                .clone()
                .unwrap()
                .get("NAME")
                .unwrap()
                .to_string(),
            covering_geohashes,
        );
    }
    geohashes_map
}
