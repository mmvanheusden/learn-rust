#![allow(non_snake_case)]
/// Learning enums
enum IpAddrKind {
    V4,
    V6
}
struct IpAddr {
    kind: IpAddrKind,
    address: String
}


enum IpAddress {
    V4(u8,u8,u8,u8),
    V6(String)
}


// We put the different types of logging in here.
enum LogLevel {
    Info,
    Error,
    Debug
}
// We create the struct. This is where the values go through
struct LogMessage {
    level: LogLevel,
    message: String,
}

struct Logger;

// Implement the functionality behind the Logger.
impl Logger {
    // Create a function for easy logging.
    fn log(&self, level: LogLevel, message: &str) {
        // create a LogMessage with the provided parameters.
        let log_message = LogMessage {
            level,
            message: message.to_string()
        };
        // Then, proceed with printing the given log message
        self.output(log_message);
    }

    // Print the log message
    fn output(&self, message: LogMessage) {
        match message.level {
            LogLevel::Info => println!("[INFO] {}", message.message),
            LogLevel::Debug => println!("[DEBUG] {}", message.message),
            LogLevel::Error => println!("[ERROR] {}", message.message),
        }
    }

    // "shortcuts"
    fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }
    fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }
    fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
}

// This returns either None, or Some.
// In rust, there is no 'null'. This is an alternative.
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn main() {
    // This is inefficient
    let localhost = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("0.0.0.0")
    };

    // This method is better, because it is simpler.
    let gateway = IpAddress::V4(192,168,178,1);

    Logger.log(LogLevel::Info, "hey!");
    Logger.error("Oh no!");

    let calculation_1 = divide(2.0, 3.0);
    let calculation_2 = divide(5.0, 0.0);
    match calculation_1 {
        Some(x) => println!("result is {x}!"),
        None => Logger.error("Calculation failed!")
    };
    match calculation_2 {
        Some(x) => println!("result is {x}!"),
        None => Logger.error("Calculation failed!")
    };
}
