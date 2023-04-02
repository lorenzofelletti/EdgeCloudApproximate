mod config;
mod kafka_producer;

fn main() {
    println!("Hello, world!");
}

// read from kafka topic
// as many msgs as possible
// process data
// write to another topic
// commit offset
// sleep for window minutes
// repeat
