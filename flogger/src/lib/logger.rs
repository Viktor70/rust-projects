use chrono::{Local, SecondsFormat};
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

pub struct FakeLogger {
    file: Mutex<Option<Box<File>>>,
}

impl FakeLogger {
    pub fn new(file_path: &str) -> io::Result<Self> {
        if file_path.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Path cannot be empty",
            ));
        }
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;

        Ok(FakeLogger {
            file: Mutex::new(Some(Box::new(file))),
        })
    }

    fn log(&self, message: &str) -> io::Result<()> {
        let timestamp = Local::now().to_rfc3339_opts(SecondsFormat::Millis, false);
        if let Some(ref mut file) = *self.file.lock().unwrap() {
            writeln!(file, "[{}] {}", timestamp, message)?;
        }
        Ok(())
    }

    pub fn start(&self, period: u16) {
        let mut counter: u64 = 0;
        loop {
            counter += 1;
            let message = format!("Log entry #{}", counter);

            if let Err(e) = self.log(&message) {
                eprintln!("Failed to write to log: {}", e);
            }

            thread::sleep(Duration::from_millis(period.into()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;
    use std::path::Path;

    #[test]
    fn test_logger_creation() {
        let temp_file = "temp_test.log";
        let logger = FakeLogger::new(temp_file);

        assert!(logger.is_ok());
        assert!(Path::new(temp_file).exists());

        // Cleanup
        fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_logging_message() {
        let temp_file = "temp_test_logging.log";
        let logger = FakeLogger::new(temp_file).unwrap();

        let test_message = "Test log message";
        let result = logger.log(test_message);

        assert!(result.is_ok());

        // Verify the content
        let mut content = String::new();
        let mut file = fs::File::open(temp_file).unwrap();
        file.read_to_string(&mut content).unwrap();

        assert!(content.contains(test_message));
        assert!(content.starts_with('['));
        assert!(content.contains(']'));

        // Cleanup
        fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_concurrent_logging() {
        let temp_file = "temp_test_concurrent.log";
        let logger = std::sync::Arc::new(FakeLogger::new(temp_file).unwrap());

        let logger1 = logger.clone();
        let logger2 = logger.clone();

        let handle1 = thread::spawn(move || {
            logger1.log("Thread 1 message").unwrap();
        });

        let handle2 = thread::spawn(move || {
            logger2.log("Thread 2 message").unwrap();
        });

        handle1.join().unwrap();
        handle2.join().unwrap();

        // Verify the content
        let content = fs::read_to_string(temp_file).unwrap();
        assert!(content.contains("Thread 1 message"));
        assert!(content.contains("Thread 2 message"));

        // Cleanup
        fs::remove_file(temp_file).unwrap();
    }
}
