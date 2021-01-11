#[derive(Debug, Copy, Clone)]
pub enum Command {
    Hex,
    Dec,
    Oct,
    Bin,
    Qword,
    Dword,
    Word,
    Byte,
}
#[derive(Debug, Clone)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}
#[derive(Debug, Clone)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}
