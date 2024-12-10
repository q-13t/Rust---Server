use chrono::{DateTime, Utc};
use std::time::SystemTime;

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub enum LogLevel {
    Error = 0,
    Info = 1,
    Debug = 2,
}
/// Creates a new logger
/// <br>
/// Levels are cascaded like so:
/// <ul>
/// <li>Error</li>
/// <li>Info</li>
/// <li>Debug</li>
/// </ul>
///
/// If the log level is higher than debug, it will be ignored.
/// <br>
/// # Arguments
/// * `c_name` - The name of the logger (The passed value will be printed to console) : &str
/// * `level` - The level of the logger : LogLevel
///
pub struct Logger {
    pub c_name: &'static str,
    pub level: LogLevel,
}

#[allow(unused)]
impl Logger {
    /// Creates a new logger
    /// <br>
    /// Levels are cascaded like so:
    /// <ul>
    /// <li>Error</li>
    /// <li>Info</li>
    /// <li>Debug</li>
    /// </ul>
    ///
    /// If the log level is higher than debug, it will be ignored.
    /// <br>
    /// # Arguments
    /// * `c_name` - The name of the logger (The passed value will be printed to console) : &str
    /// * `level` - The level of the logger : LogLevel
    ///
    pub fn new(c_name: &'static str, level: LogLevel) -> Logger {
        if level > LogLevel::Debug {
            panic!("Invalid log level");
        }
        Logger { c_name, level }
    }

    fn get_time() -> String {
        let dt: DateTime<Utc> = SystemTime::now().clone().into();
        dt.format("%d-%m-%Y %H:%M:%S:%3f").to_string()
    }

    /// Prints an error message
    /// <br>
    /// Levels lower than error are ignored
    /// # Arguments
    /// * `err` - The error to print : &str
    /// * `message` - The message to print : &[&str]
    pub fn error(&self, err: &str, message: &[&str]) {
        if self.level >= LogLevel::Error {
            println!(
                "[ {}\t ERROR\t [{}]\t]: {} \n {}",
                Logger::get_time(),
                self.c_name,
                message.join(" "),
                err,
            );
        }
    }

    /// Prints an info message
    /// <br>
    /// Levels lower than info are ignored
    /// # Arguments
    /// * `message` - The message to print : &[&str]
    pub fn info(&self, message: &[&str]) {
        if self.level >= LogLevel::Info {
            println!(
                "[ {}\t INFO\t [{}]\t]: {}",
                Logger::get_time(),
                self.c_name,
                message.join(" "),
            );
        }
    }

    /// Prints a debug message
    /// <br>
    /// Levels lower than debug are ignored
    /// # Arguments
    /// * `message` - The message to print : &[&str]
    pub fn debug(&self, message: &[&str]) {
        if self.level >= LogLevel::Debug {
            println!(
                "[ {}\t DEBUG\t [{}\t]]: {} ",
                Logger::get_time(),
                self.c_name,
                message.join(" "),
            );
        }
    }
}
