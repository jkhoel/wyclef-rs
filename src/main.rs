use ratatui::{backend::CrosstermBackend, Terminal};
use std::env;
use std::io::{stdout, Error, ErrorKind};

struct ProgramArguments {
    file_path: String,
}

fn main() -> Result<(), Error> {
    let args = &env::args().collect::<Vec<String>>();
    let program_args = get_program_args(args);

    let execution_result = match program_args {
        Ok(program_args) => run_terminal_with_args(&program_args),
        Err(error) => {
            Err(Error::new(ErrorKind::Other, error))
        }
    };

    match execution_result.is_err() {
        true => {
            eprintln!("Wyclef encountered an error: {}", execution_result.err().unwrap());
            Ok(())
        },
        false => Ok(()),
    }
}

fn run_terminal_with_args(_program_args: &ProgramArguments) -> Result<(), Error> {
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);

    let mut _terminal = Terminal::new(backend)?;

    Ok(())
}

/*fn run_terminal_with_args(program_args: &ProgramArguments) {
    let eventlog = wyclef_rs::CompactLogEventsFormatFile::new(&program_args.file_path).unwrap();
    eventlog.print();
}*/

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
    let error_message = "Please provide a path to a log file!";
    match args.len() > 1 {
        true => match args[1].is_empty() {
            true => Err(error_message),
            false => Ok(&args[1]),
        },
        false => Err(error_message),
    }
}

#[cfg(test)]
mod tests {
    use crate::resolve_file_path_from_args;

    #[test]
    fn throws_error_when_file_name_args_is_missing() {
        let args = vec!["not-used-for-this-test".to_string()];
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
