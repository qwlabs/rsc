extern crate clap;

fn main() {
    use clap::{App, load_yaml};
    let yaml = load_yaml!("./cli.yaml");
    let matches = App::from(yaml).get_matches();
    println!("---start---");
    if matches.is_present("fnf") {
        println!("'fire_and_forget' was run.");
    }
    if matches.is_present("rc") {
        println!("'request_channel' was run.");
    }
    if matches.is_present("rr") {
        println!("'request_response' was run.");
    }
    if matches.is_present("rs") {
        println!("'request_stream' was run.");
    }
    if matches.is_present("mp") {
        println!("'metadata_push' was run.");
    }
}
