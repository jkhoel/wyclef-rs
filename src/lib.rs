use anyhow::Context;

pub mod app;
pub mod models;

#[derive(Debug, PartialEq)]
pub struct AppArguments {
    pub file_path: String,
}

pub fn get_app_arguments_from_program_args(args: &[String]) -> anyhow::Result<AppArguments> {
    let file_path = (args.len() > 1)
        .then(|| &args[1])
        .context("Please provide a path to a log file!")?;

    Ok(AppArguments {
        file_path: file_path.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn throws_error_when_file_name_args_is_missing() {
        let args = vec!["not-used-for-this-test".to_string()];
        let result = get_app_arguments_from_program_args(&args);
        assert!(result.is_err());
    }

    #[test]
    fn can_resolve_file_path_from_args() {
        let args = vec![
            "not-used-for-this-test".to_string(),
            "test.jlog".to_string(),
        ];

        let result = get_app_arguments_from_program_args(&args);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().file_path, "test.jlog");
    }
}
