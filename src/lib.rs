use colored::Colorize;
use regex::Regex;
use serde_json::Value;
use std::{fmt, fs};

/// # Log levels, based on Serilog levels
#[derive(Debug)]
pub enum Level {
    Verbose,
    Debug,
    Information,
    Warning,
    Error,
    Fatal,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let level = match self {
            Level::Verbose => "VERBOSE",
            Level::Debug => "DEBUG",
            Level::Information => "INFORMATION",
            Level::Warning => "WARNING",
            Level::Error => "ERROR",
            Level::Fatal => "FATAL",
        };

        write!(f, "{}", level)
    }
}

/// # A log event
///
/// ## From the CLEF spec:
/// @t Timestamp An ISO 8601 timestamp
/// @m Message A fully-rendered message describing the event
/// @mt Message template Alternative to Message; specifies a message template over the event’s properties that provides for rendering into a textual description of the event
/// @l Level An implementation-specific level or severity identifier (string or number) Absence implies “informational”
/// @x Exception A language-dependent error representation potentially including backtrace
/// @i Event id An implementation specific event id, identifying the type of the event (string or number)
/// @r Renderings If @mt includes tokens with programming-language-specific formatting, an array of pre-rendered values for each such token. May be omitted; if present, the count of renderings must match the count of formatted tokens exactly
///
#[derive(Debug)]
pub struct ClefEvent {
    pub timestamp: String,
    pub message: String,
    pub template: String,
    pub level: Level,
    pub exception: String,
    pub event_id: String,
    pub renderings: Vec<Rendering>,
}

#[derive(Debug)]
pub struct Rendering {
    pub key: String,
    pub value: String,
}

impl fmt::Display for Rendering {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.key, self.value)
    }
}

impl ClefEvent {
    pub fn new(event: &str) -> ClefEvent {
        let event_as_json: Value =
            serde_json::from_str(event).expect("Error occurred when parsing event!");

        ClefEvent {
            timestamp: event_as_json["@t"].to_string(),
            message: event_as_json["@m"].to_string(),
            template: event_as_json["@mt"].to_string(),
            exception: event_as_json["@x"].to_string(),
            event_id: event_as_json["@i"].to_string(),
            level: ClefEvent::get_level(&event_as_json["@l"].to_string()),
            renderings: ClefEvent::get_renderings(&event_as_json),
        }
    }

    /// Extracts all keys from the parsed (JSON) event log that are not in the ClefEvent struct, as these will then be renderings
    fn get_renderings(event_as_json: &Value) -> Vec<Rendering> {
        let rendering_keys = event_as_json
            .as_object()
            .unwrap()
            .keys()
            .filter(|key| !matches!(key.as_str(), "@t" | "@m" | "@mt" | "@l" | "@x" | "@i"))
            .collect::<Vec<&String>>();

        rendering_keys
            .iter()
            .map(|key| Rendering {
                key: key.to_string(),
                value: event_as_json[key].to_string(),
            })
            .collect()
    }

    fn get_level(level: &str) -> Level {
        let filtered_level = Regex::new(r"[^A-Za-z0-9]")
            .unwrap()
            .replace_all(level, "")
            .to_string()
            .to_lowercase();

        match filtered_level.as_str() {
            "verbose" => Level::Verbose,
            "debug" => Level::Debug,
            "information" => Level::Information,
            "warning" => Level::Warning,
            "error" => Level::Error,
            "fatal" => Level::Fatal,

            "1" => Level::Verbose,
            "2" => Level::Debug,
            "3" => Level::Information,
            "4" => Level::Warning,
            "5" => Level::Error,
            "6" => Level::Fatal,

            _ => Level::Information,
        }
    }
}

#[derive(Debug)]
pub struct CompactLogEventsFormatFile {
    pub events: Vec<ClefEvent>,
}

impl CompactLogEventsFormatFile {
    pub fn new(file_path: &str) -> Result<CompactLogEventsFormatFile, std::io::Error> {
        let file_content = fs::read_to_string(file_path)?;
        let file_events: Vec<String> = file_content.lines().map(|line| line.to_string()).collect();

        let events: Vec<ClefEvent> = file_events
            .iter()
            .map(|event| ClefEvent::new(event))
            .collect();

        Ok(CompactLogEventsFormatFile { events })
    }

    pub fn print(&self) {
        self.events.iter().for_each(|event| {
            let message = match event.template.is_empty() {
                false => &event.template,
                true => match event.message.is_empty() {
                    false => &event.message,
                    true => "No message",
                },
            };

            // Find bracketed objects in the message and replace them with the corresponding rendering
            let mut message = message.to_string();
            for rendering in event.renderings.iter() {
                let bracketed_object = format!("{{{}}}", rendering.key);
                message = message.replace(&bracketed_object, &rendering.value);
            }

            let formatted_message = format!(
                "{}: [{}] {}",
                event.timestamp.replace('"', ""),
                event.level,
                message
            );

            match event.level {
                Level::Verbose => println!("{}", formatted_message.white()),
                Level::Debug => println!("{}", formatted_message.cyan()),
                Level::Information => println!("{}", formatted_message.green()),
                Level::Warning => println!("{}", formatted_message.yellow()),
                Level::Error => println!("{}", formatted_message.red()),
                Level::Fatal => println!("{}", formatted_message.red().bold()),
            }
        });
    }
}
