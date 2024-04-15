#[cfg(windows)]
pub const LINE_ENDING : &str = "\r\n";
#[cfg(not(windows))]
pub const LINE_ENDING : &str = "\n";

#[cfg(windows)]
pub const TWO_LINE_ENDING: &str = "\r\n\r\n";
#[cfg(not(windows))]
pub const TWO_LINE_ENDING : &str = "\n\n";