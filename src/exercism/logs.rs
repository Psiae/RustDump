use std::fmt::format;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let l = match level {
        LogLevel::Info => "INFO",
        LogLevel::Warning => "WARNING",
        LogLevel::Error => "ERROR"
    };
    return format!("[{}]: {1}", l, message)
}
pub fn info(message: &str) -> String {
    return log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    return log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    return log(LogLevel::Error, message)
}
