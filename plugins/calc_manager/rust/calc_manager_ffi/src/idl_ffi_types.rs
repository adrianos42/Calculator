use calc_manager_types::idl_types;
use idl_internal::ffi::ffi_types::*;
pub type Command = i64;
impl From<idl_types::Command> for Command {
    fn from(value: idl_types::Command) -> Self {
        match value {
            idl_types::Command::Hex => 0,
            idl_types::Command::Dec => 1,
            idl_types::Command::Oct => 2,
            idl_types::Command::Bin => 3,
            idl_types::Command::Qword => 4,
            idl_types::Command::Dword => 5,
            idl_types::Command::Word => 6,
            idl_types::Command::Byte => 7,
        }
    }
}
impl From<Command> for idl_types::Command {
    fn from(value: Command) -> Self {
        match value {
            0 => idl_types::Command::Hex,
            1 => idl_types::Command::Dec,
            2 => idl_types::Command::Oct,
            3 => idl_types::Command::Bin,
            4 => idl_types::Command::Qword,
            5 => idl_types::Command::Dword,
            6 => idl_types::Command::Word,
            7 => idl_types::Command::Byte,
            _ => panic!(),
        }
    }
}
#[repr(C)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}
impl From<idl_types::Point> for Point {
    #[allow(unused_braces)]
    fn from(value: idl_types::Point) -> Self {
        Self {
            x: { value.x } as i64,
            y: { value.y } as i64,
        }
    }
}
impl From<Point> for idl_types::Point {
    #[allow(unused_braces)]
    fn from(value: Point) -> Self {
        Self {
            x: { value.x } as i64,
            y: { value.y } as i64,
        }
    }
}
#[repr(C)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}
impl From<idl_types::Size> for Size {
    #[allow(unused_braces)]
    fn from(value: idl_types::Size) -> Self {
        Self {
            width: { value.width } as f64,
            height: { value.height } as f64,
        }
    }
}
impl From<Size> for idl_types::Size {
    #[allow(unused_braces)]
    fn from(value: Size) -> Self {
        Self {
            width: { value.width } as f64,
            height: { value.height } as f64,
        }
    }
}
