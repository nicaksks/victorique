use std::fs::OpenOptions;
use std::io::{BufWriter, ErrorKind, Write};
use std::{fmt::Display, fs};

use crate::logger::datetime::Datetime;
use crate::utils::terminal::Colorize;
pub struct Logger;
pub enum Levels {
    Fatal,
    Error,
    Warn,
    Info,
    Debug,
}
pub trait Constructor {
    fn level(&self, level: Levels) -> String;
    fn fatal<T: Display>(&self, content: T);
    fn error<T: Display>(&self, content: T);
    fn warn<T: Display>(&self, content: T);
    fn info<T: Display>(&self, content: T);
    fn debug<T: Display>(&self, content: T);
    fn execute<T: Display>(&self, path: &str, level: Levels, content: T);

    fn already_exist(&self, name: &str, content: String) {
        match fs::read_dir(format!("./log/{}/{}", name, Datetime.date())) {
            Ok(_) => self.create_file(name, content.clone()),
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    self.create_dir(name, content)
                }
            }
        };
    }

    fn create_dir(&self, name: &str, content: String) {
        match fs::create_dir_all(format!("./log/{}/{}", name, Datetime.date())) {
            Ok(_) => self.create_file(name, content),
            Err(e) => eprintln!("Error creating directory: {}", e),
        };
    }

    fn create_file(&self, name: &str, content: String) {
        let format = format!(
            "./log/{}/{}/{}.log",
            name,
            Datetime.date(),
            Datetime.time(false)
        );

        match OpenOptions::new().append(true).create(true).open(&format) {
            Ok(file) => {
                let mut buffer = BufWriter::new(file);
                if let Err(e) = writeln!(buffer, "{}", content) {
                    eprintln!("{}", e);
                }
                println!("{}", content);
            }
            Err(e) => eprintln!("Error save log in file: {}", e),
        }
    }
}

impl Constructor for Logger {
    fn level(&self, level: Levels) -> String {
        match level {
            Levels::Fatal => "FATAL".magenta(),
            Levels::Error => "ERROR".red(),
            Levels::Warn => "WARN".yellow(),
            Levels::Info => "INFO".green(),
            Levels::Debug => "DEBUG".cyan(),
        }
    }

    fn fatal<T: Display>(&self, content: T) {
        self.execute("fatal", Levels::Fatal, content);
    }

    fn error<T: Display>(&self, content: T) {
        self.execute("error", Levels::Error, content);
    }

    fn warn<T: Display>(&self, content: T) {
        self.execute("warn", Levels::Warn, content);
    }

    fn info<T: Display>(&self, content: T) {
        self.execute("info", Levels::Info, content);
    }

    fn debug<T: Display>(&self, content: T) {
        self.execute("debug", Levels::Debug, content);
    }

    fn execute<T: Display>(&self, path: &str, level: Levels, content: T) {
        self.already_exist(
            path,
            format!(
                "{} - <{}>: {}",
                Datetime.formatted_datetime(),
                self.level(level),
                content
            ),
        );
    }
}
