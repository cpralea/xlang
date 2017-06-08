use std::iter;


pub const TAB: &str = "  ";
pub const NL: &str = "\n";


pub fn take(times: usize, string: &str) -> String {
    iter::repeat(string).take(times).collect::<String>()
}
