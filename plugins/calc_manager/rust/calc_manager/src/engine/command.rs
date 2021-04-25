use super::{radix_type::RadixType, AngleType, NumWidth};
use crate::engine_strings::EngineStringsId;

#[derive(PartialEq, Eq)]
pub enum CommandType {
    Unany,
    Binary,
    Operand,
    Parentheses,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Command {
    Nop = 0,
    Hex = 313,
    Dec = 314,
    Oct = 315,
    Bin = 316,
    Deg = 321,
    Rad = 322,
    Grad = 323,
    Degrees = 324,
    Qword = 317,
    Dword = 318,
    Word = 319,
    Byte = 320,
    Sign = 80,
    Clear = 81,
    Centr = 82,
    Back = 83,
    Pnt = 84,
    EmptyString = 85,
    And = 86,
    Or = 87,
    Xor = 88,
    Lshf = 89,
    Rshf = 90,
    Div = 91,
    Mul = 92,
    Add = 93,
    Sub = 94,
    Mod = 95,
    Root = 96,
    Pwr = 97,
    Chop = 98,
    Rol = 99,
    Ror = 100,
    Com = 101,
    Sin = 102,
    Cos = 103,
    Tan = 104,
    Sinh = 105,
    Cosh = 106,
    Tanh = 107,
    Ln = 108,
    Log = 109,
    Sqrt = 110,
    Sqr = 111,
    Cub = 112,
    Fac = 113,
    Rec = 114,
    Dms = 115,
    CubeRoot = 116,
    Pow10 = 117,
    Percent = 118,
    Fe = 119,
    Pi = 120,
    Equ = 121,
    MClear = 122,
    Recall = 123,
    Store = 124,
    MPlus = 125,
    MMinus = 126,
    Exp = 127,
    OpenP = 128,
    CloseP = 129,
    N0 = 130,
    N1 = 131,
    N2 = 132,
    N3 = 133,
    N4 = 134,
    N5 = 135,
    N6 = 136,
    N7 = 137,
    N8 = 138,
    N9 = 139,
    NA = 140,
    NB = 141,
    NC = 142,
    ND = 143,
    NE = 144,
    NF = 145,
    Inv = 146,
    SetResult = 147,

    Asin = 202,
    Acos = 203,
    Atan = 204,
    Powe = 205,
    Asinh = 206,
    Acosh = 207,
    Atanh = 208,

    Sec = 400,
    Asec = 401,
    Csc = 402,
    Acsc = 403,
    Cot = 404,
    Acot = 405,
    Sech = 406,
    Asech = 407,
    Csch = 408,
    Acsch = 409,
    Coth = 410,
    Acoth = 411,
    Pow2 = 412,
    Abs = 413,
    Floor = 414,
    Ceil = 415,
    Rolc = 416,
    Rorc = 417,
    LogBaseY = 500,
    Nand = 501,
    Nor = 502,
    Rshfl = 505,
    Rand = 600,
    Euler = 601,
    BinPos0 = 700,
    BinPos1 = 701,
    BinPos2 = 702,
    BinPos3 = 703,
    BinPos4 = 704,
    BinPos5 = 705,
    BinPos6 = 706,
    BinPos7 = 707,
    BinPos8 = 708,
    BinPos9 = 709,
    BinPos10 = 710,
    BinPos11 = 711,
    BinPos12 = 712,
    BinPos13 = 713,
    BinPos14 = 714,
    BinPos15 = 715,
    BinPos16 = 716,
    BinPos17 = 717,
    BinPos18 = 718,
    BinPos19 = 719,
    BinPos20 = 720,
    BinPos21 = 721,
    BinPos22 = 722,
    BinPos23 = 723,
    BinPos24 = 724,
    BinPos25 = 725,
    BinPos26 = 726,
    BinPos27 = 727,
    BinPos28 = 728,
    BinPos29 = 729,
    BinPos30 = 730,
    BinPos31 = 731,
    BinPos32 = 732,
    BinPos33 = 733,
    BinPos34 = 734,
    BinPos35 = 735,
    BinPos36 = 736,
    BinPos37 = 737,
    BinPos38 = 738,
    BinPos39 = 739,
    BinPos40 = 740,
    BinPos41 = 741,
    BinPos42 = 742,
    BinPos43 = 743,
    BinPos44 = 744,
    BinPos45 = 745,
    BinPos46 = 746,
    BinPos47 = 747,
    BinPos48 = 748,
    BinPos49 = 749,
    BinPos50 = 750,
    BinPos51 = 751,
    BinPos52 = 752,
    BinPos53 = 753,
    BinPos54 = 754,
    BinPos55 = 755,
    BinPos56 = 756,
    BinPos57 = 757,
    BinPos58 = 758,
    BinPos59 = 759,
    BinPos60 = 760,
    BinPos61 = 761,
    BinPos62 = 762,
    BinPos63 = 763,

    ModeBasic = 1000,
    ModeScientific = 1001,
    ModeProgrammer = 1002,
}

type Error = &'static str;

impl Command {
    pub fn digit_from_integer(value: isize) -> Option<Self> {
        match value {
            0x0 => Some(Command::N0),
            0x1 => Some(Command::N1),
            0x2 => Some(Command::N2),
            0x3 => Some(Command::N3),
            0x4 => Some(Command::N4),
            0x5 => Some(Command::N5),
            0x6 => Some(Command::N6),
            0x7 => Some(Command::N7),
            0x8 => Some(Command::N8),
            0x9 => Some(Command::N9),
            0xA => Some(Command::NA),
            0xB => Some(Command::NB),
            0xC => Some(Command::NC),
            0xD => Some(Command::ND),
            0xE => Some(Command::NE),
            0xF => Some(Command::NF),
            _ => None,
        }
    }

    pub fn binary_index(self) -> usize {
        self as usize - BIN_EDIT_START as usize
    }

    pub fn get_integer(self) -> u64 {
        match self {
            Command::N0 => 0x0,
            Command::N1 => 0x1,
            Command::N2 => 0x2,
            Command::N3 => 0x3,
            Command::N4 => 0x4,
            Command::N5 => 0x5,
            Command::N6 => 0x6,
            Command::N7 => 0x7,
            Command::N8 => 0x8,
            Command::N9 => 0x9,
            Command::NA => 0xA,
            Command::NB => 0xB,
            Command::NC => 0xC,
            Command::ND => 0xD,
            Command::NE => 0xE,
            Command::NF => 0xF,
            _ => 0,
        }
    }

    pub fn get_radix_type(self) -> Result<RadixType, Error> {
        match self {
            Command::Hex => Ok(RadixType::Hexadecimal),
            Command::Dec => Ok(RadixType::Decimal),
            Command::Oct => Ok(RadixType::Octal),
            Command::Bin => Ok(RadixType::Binary),
            _ => Err("Invalid command for RadixType."),
        }
    }

    pub fn get_num_width(self) -> Result<NumWidth, Error> {
        match self {
            Command::Qword => Ok(NumWidth::Qword),
            Command::Dword => Ok(NumWidth::Dword),
            Command::Word => Ok(NumWidth::Word),
            Command::Byte => Ok(NumWidth::Byte),
            _ => Err("Invalid command for NumWidth."),
        }
    }

    pub fn get_angle_type(self) -> Result<AngleType, Error> {
        match self {
            Command::Dec => Ok(AngleType::Degress),
            Command::Rad => Ok(AngleType::Radians),
            Command::Grad => Ok(AngleType::Gradians),
            _ => Err("Invalid command for AngleType"),
        }
    }

    pub fn get_string_id(self) -> Result<EngineStringsId, Error> {
        match self {
            Command::Sign => Ok(EngineStringsId::PlusMinus),
            Command::Clear => Ok(EngineStringsId::C),
            Command::Centr => Ok(EngineStringsId::Ce),
            Command::Back => Ok(EngineStringsId::Backspace),
            Command::Pnt => Ok(EngineStringsId::DecimalSeparator),
            Command::EmptyString => Ok(EngineStringsId::EmptyString),
            Command::And => Ok(EngineStringsId::And),
            Command::Or => Ok(EngineStringsId::Or),
            Command::Xor => Ok(EngineStringsId::Xor),
            Command::Lshf => Ok(EngineStringsId::Lsh),
            Command::Rshf => Ok(EngineStringsId::Rsh),
            Command::Div => Ok(EngineStringsId::Divide),
            Command::Mul => Ok(EngineStringsId::Multiply),
            Command::Add => Ok(EngineStringsId::Plus),
            Command::Sub => Ok(EngineStringsId::Minus),
            Command::Mod => Ok(EngineStringsId::Mod),
            Command::Root => Ok(EngineStringsId::YRoot),
            Command::Pwr => Ok(EngineStringsId::PowHat),
            Command::Chop => Ok(EngineStringsId::Int),
            Command::Rol => Ok(EngineStringsId::Rol),
            Command::Ror => Ok(EngineStringsId::Ror),
            Command::Com => Ok(EngineStringsId::Not),
            Command::Sin => Ok(EngineStringsId::Sin),
            Command::Cos => Ok(EngineStringsId::Cos),
            Command::Tan => Ok(EngineStringsId::Tan),
            Command::Sinh => Ok(EngineStringsId::Sinh),
            Command::Cosh => Ok(EngineStringsId::Cosh),
            Command::Tanh => Ok(EngineStringsId::Tanh),
            Command::Ln => Ok(EngineStringsId::Ln),
            Command::Log => Ok(EngineStringsId::Log),
            Command::Sqrt => Ok(EngineStringsId::Sqrt),
            Command::Sqr => Ok(EngineStringsId::XPow2),
            Command::Cub => Ok(EngineStringsId::XPow3),
            Command::Fac => Ok(EngineStringsId::Nfactorial),
            Command::Rec => Ok(EngineStringsId::Reciprocal),
            Command::Dms => Ok(EngineStringsId::Dms),
            Command::CubeRoot => Ok(EngineStringsId::Powten),
            Command::Percent => Ok(EngineStringsId::Percent),
            Command::Fe => Ok(EngineStringsId::ScientificNotation),
            Command::Pi => Ok(EngineStringsId::Pi),
            Command::Equ => Ok(EngineStringsId::Equal),
            Command::MClear => Ok(EngineStringsId::Mc),
            Command::Recall => Ok(EngineStringsId::Mr),
            Command::Store => Ok(EngineStringsId::Ms),
            Command::MPlus => Ok(EngineStringsId::Mplus),
            Command::MMinus => Ok(EngineStringsId::Mminus),
            Command::Exp => Ok(EngineStringsId::Exp),
            Command::OpenP => Ok(EngineStringsId::OpenParen),
            Command::CloseP => Ok(EngineStringsId::CloseParen),
            Command::N0 => Ok(EngineStringsId::N0),
            Command::N1 => Ok(EngineStringsId::N1),
            Command::N2 => Ok(EngineStringsId::N2),
            Command::N3 => Ok(EngineStringsId::N3),
            Command::N4 => Ok(EngineStringsId::N4),
            Command::N5 => Ok(EngineStringsId::N5),
            Command::N6 => Ok(EngineStringsId::N6),
            Command::N7 => Ok(EngineStringsId::N7),
            Command::N8 => Ok(EngineStringsId::N8),
            Command::N9 => Ok(EngineStringsId::N9),
            Command::NA => Ok(EngineStringsId::Na),
            Command::NB => Ok(EngineStringsId::Nb),
            Command::NC => Ok(EngineStringsId::Nc),
            Command::ND => Ok(EngineStringsId::Nd),
            Command::NE => Ok(EngineStringsId::Ne),
            Command::NF => Ok(EngineStringsId::Nf),
            Command::Inv => Ok(EngineStringsId::Frac),
            _ => Err("Invalid command for EngineStringsId"),
        }
    }

    pub fn is_in_range(self, x: Command, y: Command) -> bool {
        ((self as isize) >= (x as isize)) && ((self as isize) <= (y as isize))
    }

    pub fn is_binary(self) -> bool {
        
        self.is_in_range(Command::And, Command::Pwr)
            || self.is_in_range(BINARY_EXTENDED_FIRST, BINARY_EXTENDED_LAST)
    }

    // WARNING: Command::Sign is a special unary op but still this doesn't catch this. Caller has to be aware
    // of it and catch it themselves or not needing this
    pub fn is_unary(self) -> bool {
        self.is_in_range(UNARY_FIRST, UNARY_LAST)
            || self.is_in_range(UNARY_EXTENDED_FIRST, UNARY_EXTENDED_LAST)
    }

    pub fn is_digit(self) -> bool {
        self.is_in_range(Command::N0, Command::NF)
    }

    pub fn is_gui_setting(self) -> bool {
        self.is_in_range(Command::Hex, Command::Bin)
            || self.is_in_range(Command::Qword, Command::Byte)
            || self.is_in_range(Command::Deg, Command::Grad)
            || match self {
                Command::Inv
                | Command::Fe
                | Command::MClear
                | Command::Back
                | Command::Exp
                | Command::Store
                | Command::MPlus
                | Command::MMinus => true,
                _ => false,
            }
    }
}

pub const FIRST_CONTROL: Command = Command::Sign;
pub const UNARY_FIRST: Command = Command::Chop;
pub const UNARY_EXTENDED_FIRST: Command = STRING_MAPPED_VALUES;
pub const UNARY_EXTENDED_LAST: Command = Command::Rorc;
pub const LAST_CONTROL: Command = Command::Ceil;
pub const BINARY_EXTENDED_LAST: Command = Command::Rshfl;
pub const UNARY_LAST: Command = Command::Percent;
pub const STRING_MAPPED_VALUES: Command = Command::Sec;
pub const BIN_EDIT_START: Command = Command::BinPos0;
pub const BIN_EDIT_END: Command = Command::BinPos63;
pub const BINARY_EXTENDED_FIRST: Command = Command::LogBaseY;
