use logging::{LogLevel, LogMessage};

mod logging;

fn main() {
    LogMessage::new(LogLevel::Info, "info").log();
    LogMessage::new(LogLevel::Warning, "warning").log();
    LogMessage::new(LogLevel::Error, "error").log();
}
