use web_sys::console;

pub fn log<T: std::fmt::Debug>(message: T) {
    console::log_1(&format!("{:?}", message).into());
}
