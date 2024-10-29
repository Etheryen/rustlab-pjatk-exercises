use std::fmt::Display;

pub enum LogLevel {
    Info,
    Warning,
    Error,
}

pub struct LogMessage<'a> {
    level: LogLevel,
    content: &'a str,
}

impl<'a> LogMessage<'a> {
    pub fn new(level: LogLevel, content: &'a str) -> Self {
        Self { level, content }
    }
}

impl Display for LogMessage<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.level {
            LogLevel::Info => write!(f, "[INFO]: {}", self.content),
            LogLevel::Warning => write!(f, "[WARNING]: {}", self.content),
            LogLevel::Error => write!(f, "[ERROR]: {}", self.content),
        }
    }
}

impl LogMessage<'_> {
    pub fn log(&self) {
        println!("{}", self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logs_info() {
        let log_message = LogMessage::new(LogLevel::Info, "some info");
        assert_eq!(log_message.to_string(), "[INFO]: some info");
        log_message.log();
    }

    #[test]
    fn logs_warning() {
        let log_message = LogMessage::new(LogLevel::Warning, "warning!!!");
        assert_eq!(log_message.to_string(), "[WARNING]: warning!!!");
        log_message.log();
    }

    #[test]
    fn logs_error() {
        let log_message = LogMessage::new(LogLevel::Error, "error occured!");
        assert_eq!(log_message.to_string(), "[ERROR]: error occured!");
        log_message.log();
    }
}
