// The debug version
#[cfg(debug_assertions)]
macro_rules! debug_log {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

// Non-debug version
#[cfg(not(debug_assertions))]
macro_rules! debug_log {
    ($( $args:expr ),*) => {};
}
