#[derive(Debug, Deserialize, Serialize)]
struct Message {
    id: String,
    lat: f64,
    lon: f64,
    time: u64,
    speed: f64,
}

fn main() {
    let file = File::open("data.csv")?;
    let reader = BufReader::new(file);

    let mut headers = vec![];
    let mut rows = vec![];
}
