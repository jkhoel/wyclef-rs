use std::env;
use std::io;

struct ProgramArguments {
    file_path: String,
}

fn main() -> Result<(), io::Error> {
    let args = &env::args().collect::<Vec<String>>();
    let program_args = get_program_args(args);

    match program_args {
        Ok(program_args) => print_eventlog(&program_args),
        Err(error) => {
            panic!("{}", error)
        }
    }

    Ok(())
}

fn print_eventlog(program_args: &ProgramArguments) {
    let eventlog = wyclef_rs::CompactLogEventsFormatFile::new(&program_args.file_path).unwrap();
    eventlog.print();
}

fn get_program_args(args: &[String]) -> Result<ProgramArguments, &str> {
    let file_path = resolve_file_path_from_args(args);

    match file_path {
        Ok(file_path) => Ok(create_program_args(file_path)),
        Err(error) => Err(error),
    }
}

fn create_program_args(file_path: &str) -> ProgramArguments {
    ProgramArguments {
        file_path: file_path.to_string(),
    }
}

fn resolve_file_path_from_args(args: &[String]) -> Result<&String, &str> {
    match &args[1].is_empty() {
        true => Err("Please provide a file path!"),
        false => Ok(&args[1]),
    }
}

#[cfg(test)]
mod tests {
    use crate::resolve_file_path_from_args;

    #[test]
    fn throws_error_when_resolving_file_path_from_args_with_empty_string() {
        let args = vec!["not-used-for-this-test".to_string(), "".to_string()];
        let result = resolve_file_path_from_args(&args);
        assert!(result.is_err());
    }

    #[test]
    fn can_resolve_file_path_from_args() {
        let args = vec![
            "not-used-for-this-test".to_string(),
            "test.jlog".to_string(),
        ];

        let result = resolve_file_path_from_args(&args);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test.jlog");
    }

    #[test]
    fn can_build_program_args() {
        let file_path = "test.jlog";
        let program_args = super::create_program_args(file_path);

        assert_eq!(program_args.file_path, file_path);
    }
}
