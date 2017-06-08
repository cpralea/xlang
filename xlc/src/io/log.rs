macro_rules! log {
    ($tag:expr) => (println!($tag));
    ($tag:expr, $fmt:expr) => (println!(concat!($tag, ": ", $fmt)));
    ($tag:expr, $fmt:expr, $($arg:tt)*) => (println!(concat!($tag, ": ", $fmt), $($arg)*));
}


macro_rules! error {
    () => (log!("[Error]: "));
    ($fmt:expr) => (log!("[Error]", $fmt));
    ($fmt:expr, $($arg:tt)*) => (log!("[Error]", $fmt, $($arg)*));
}


macro_rules! debug {
    () => (log!("[Debug]: "));
    ($fmt:expr) => (log!("[Debug]", $fmt));
    ($fmt:expr, $($arg:tt)*) => (log!("[Debug]", $fmt, $($arg)*));
}
