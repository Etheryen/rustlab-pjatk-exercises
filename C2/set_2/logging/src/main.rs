use std::fs;

#[derive(PartialEq, PartialOrd)]
enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

struct Logger<'a> {
    min_level: LogLevel,
    out_path: &'a str,
}

impl<'a> Logger<'a> {
    fn new(min_level: LogLevel, out_path: &'a str) -> Self {
        Self {
            min_level,
            out_path,
        }
    }

    fn log(&self, level: LogLevel, message: &str) {
        if level < self.min_level {
            return;
        }

        let log_message = match level {
            LogLevel::Debug => format!("[DEBUG]: {}", message),
            LogLevel::Info => format!("[INFO]: {}", message),
            LogLevel::Warning => format!("[WARNING]: {}", message),
            LogLevel::Error => format!("[ERROR]: {}", message),
        };

        let mut content = fs::read_to_string(self.out_path).unwrap_or_default();
        content.push_str(&log_message);
        content.push('\n');
        fs::write(self.out_path, content).unwrap();
    }
}

fn main() {
    let logger = Logger::new(LogLevel::Info, "app.log");
    logger.log(LogLevel::Warning, "To jest ostrzeżenie!");
    logger.log(LogLevel::Debug, "Debugowanie danych..."); // Nie zapisze, ponieważ poziom to Info
    logger.log(LogLevel::Error, "Ważny błąd!!!");
    logger.log(LogLevel::Info, "Przyjęto zapytanie");
}
