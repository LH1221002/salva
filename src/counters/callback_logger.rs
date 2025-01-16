/// A logger that stores a callback as a trait object for dynamic dispatch.
pub struct CallbackLogger {
    callback: Box<dyn Fn(&str) + Send + Sync>,
}

impl CallbackLogger {
    /// Create a new logger with the given callback.
    pub fn new(callback: impl Fn(&str) + Send + Sync + 'static) -> Self {
        Self {
            callback: Box::new(callback),
        }
    }

    /// Call the stored callback with the `message`.
    pub fn log(&self, message: &str) {
        (self.callback)(message);
    }
}

// Example usage:
// fn main() {
//     // Use a closure as a callback:
//     let logger = CallbackLogger::new(|msg| println!("(Closure) Got message: {}", msg));
//     logger.log("Hello from a closure-based logger!");
//
//     // Or use a function pointer as a callback:
//     fn my_logger(msg: &str) {
//         println!("(Function) Got message: {}", msg);
//     }
//     let logger_fn = CallbackLogger::new(my_logger);
//     logger_fn.log("Hello from a function-based logger!");
// }
