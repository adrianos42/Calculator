use idl_internal::ffi::ffi_types::*;
pub type Command = i64;
impl From<super::idl_types::Command> for Command {
    fn from(value: super::idl_types::Command) -> Self {
        match value {
            super::idl_types::Command::Hex => 0,
            super::idl_types::Command::Dec => 1,
            super::idl_types::Command::Oct => 2,
            super::idl_types::Command::Bin => 3,
            super::idl_types::Command::Qword => 4,
            super::idl_types::Command::Dword => 5,
            super::idl_types::Command::Word => 6,
            super::idl_types::Command::Byte => 7,
        }
    }
}
impl From<Command> for super::idl_types::Command {
    fn from(value: Command) -> Self {
        match value {
            0 => super::idl_types::Command::Hex,
            1 => super::idl_types::Command::Dec,
            2 => super::idl_types::Command::Oct,
            3 => super::idl_types::Command::Bin,
            4 => super::idl_types::Command::Qword,
            5 => super::idl_types::Command::Dword,
            6 => super::idl_types::Command::Word,
            7 => super::idl_types::Command::Byte,
            _ => panic!(),
        }
    }
}
#[repr(C)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}
impl From<super::idl_types::Point> for Point {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::Point) -> Self {
        Self {
            x: { value.x } as i64,
            y: { value.y } as i64,
        }
    }
}
impl From<Point> for super::idl_types::Point {
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
impl From<super::idl_types::Size> for Size {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::Size) -> Self {
        Self {
            width: { value.width } as f64,
            height: { value.height } as f64,
        }
    }
}
impl From<Size> for super::idl_types::Size {
    #[allow(unused_braces)]
    fn from(value: Size) -> Self {
        Self {
            width: { value.width } as f64,
            height: { value.height } as f64,
        }
    }
}
