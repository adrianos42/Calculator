#[derive(Debug, Copy, Clone)]
pub enum Command {
    Nop,
    ModeBasic,
    ModeScientific,
    ModeProgrammer,
    Hex,
    Dec,
    Oct,
    Bin,
    Deg,
    Rad,
    Grad,
    Degrees,
    Qword,
    Dword,
    Word,
    Byte,
    Sign,
    Clear,
    Centr,
    Back,
    Pnt,
    EmptyString,
    And,
    Or,
    Xor,
    Lshf,
    Rshf,
    Div,
    Mul,
    Add,
    Sub,
    Mod,
    Root,
    Pwr,
    Chop,
    Rol,
    Ror,
    Com,
    Sin,
    Cos,
    Tan,
    Sinh,
    Cosh,
    Tanh,
    Ln,
    Log,
    Sqrt,
    Sqr,
    Cub,
    Fac,
    Rec,
    Dms,
    CubeRoot,
    Pow10,
    Percent,
    Fe,
    Pi,
    Equ,
    MClear,
    Recall,
    Store,
    MPlus,
    MMinus,
    Exp,
    OpenP,
    CloseP,
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    NA,
    NB,
    NC,
    ND,
    NE,
    NF,
    Inv,
    SetResult,
    Asin,
    Acos,
    Atan,
    Powe,
    Asinh,
    Acosh,
    Atanh,
    Sec,
    Asec,
    Csc,
    Acsc,
    Cot,
    Acot,
    Sech,
    Asech,
    Csch,
    Acsch,
    Coth,
    Acoth,
    Pow2,
    Abs,
    Floor,
    Ceil,
    Rolc,
    Rorc,
    LogBaseY,
    Nand,
    Nor,
    Rshfl,
    Rand,
    Euler,
    BinPos0,
    BinPos1,
    BinPos2,
    BinPos3,
    BinPos4,
    BinPos5,
    BinPos6,
    BinPos7,
    BinPos8,
    BinPos9,
    BinPos10,
    BinPos11,
    BinPos12,
    BinPos13,
    BinPos14,
    BinPos15,
    BinPos16,
    BinPos17,
    BinPos18,
    BinPos19,
    BinPos20,
    BinPos21,
    BinPos22,
    BinPos23,
    BinPos24,
    BinPos25,
    BinPos26,
    BinPos27,
    BinPos28,
    BinPos29,
    BinPos30,
    BinPos31,
    BinPos32,
    BinPos33,
    BinPos34,
    BinPos35,
    BinPos36,
    BinPos37,
    BinPos38,
    BinPos39,
    BinPos40,
    BinPos41,
    BinPos42,
    BinPos43,
    BinPos44,
    BinPos45,
    BinPos46,
    BinPos47,
    BinPos48,
    BinPos49,
    BinPos50,
    BinPos51,
    BinPos52,
    BinPos53,
    BinPos54,
    BinPos55,
    BinPos56,
    BinPos57,
    BinPos58,
    BinPos59,
    BinPos60,
    BinPos61,
    BinPos62,
    BinPos63,
}