// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::fmt::format;

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    if (level == LogLevel::Info) {
        info(message)
    } else if (level == LogLevel::Warning) {
        warn(message)
    } else if (level == LogLevel::Debug) {
        debug(message)
    } else {
        error(message)
    }
}
pub fn info(message: &str) -> String {
    format!("[INFO]: {}", message)
}
pub fn warn(message: &str) -> String {
    format!("[WARNING]: {}", message)
}
pub fn error(message: &str) -> String {
    format!("[ERROR]: {}", message)
}
pub fn debug(message: &str) -> String {
    format!("[DEBUG]: {}", message)
}
