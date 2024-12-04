use chrono::{DateTime, Utc};
use std::time::SystemTime;

#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
    Error = 0,
    Info = 1,
    Debug = 2,
}

pub struct Logger {
    pub c_name: &'static str,
    pub level: LogLevel,
}
#[allow(unused)]
impl Logger {
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

    pub fn error(&self, err: &str, message: &[&str]) {
        if self.level >= LogLevel::Error {
            println!(
                "[ {}\t ERROR\t [{}]]: {} \n {}",
                Logger::get_time(),
                self.c_name,
                message.join(" "),
                err,
            );
        }
    }
    pub fn info(&self, message: &[&str]) {
        if self.level >= LogLevel::Info {
            println!(
                "[ {}\t INFO\t [{}]]: {}",
                Logger::get_time(),
                self.c_name,
                message.join(" "),
            );
        }
    }

    pub fn debug(&self, message: &[&str]) {
        if self.level >= LogLevel::Debug {
            println!(
                "[ {}\t DEBUG\t [{}]]: {} ",
                Logger::get_time(),
                self.c_name,
                message.join(" "),
            );
        }
    }
}
