use std::env;
use wyclef_rs::*;

fn main() {
    // Get the file_path argument passed to the program
    let args: Vec<String> = env::args().collect();
    let file_path = match &args[1].is_empty() {
        true => panic!("Please provide a file path!"),
        false => &args[1],
    };

    let log = CompactLogEventsFormatFile::new(file_path).unwrap();

    clear_terminal();
    log.print();
}

fn clear_terminal() {
    print!("{}[2J", 27 as char);
}
