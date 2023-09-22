use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = resolve_file_path_from_args(&args);

    let log = wyclef_rs::CompactLogEventsFormatFile::new(&file_path).unwrap();

    log.print();
}

fn resolve_file_path_from_args(args: &[String]) -> String {
    let file_path = match &args[1].is_empty() {
        true => panic!("Please provide a file path!"),
        false => &args[1],
    };

    file_path.to_string()
}

#[cfg(test)]
mod tests {
    use crate::resolve_file_path_from_args;

    #[test]
    fn can_resolve_file_path_from_args() {
        let args = vec![
            "not-used-for-this-test".to_string(),
            "test.jlog".to_string(),
        ];
        let result = resolve_file_path_from_args(&args);
        assert_eq!(result, "test.jlog".to_string());
    }
}
