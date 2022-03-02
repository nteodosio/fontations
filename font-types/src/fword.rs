//! 16-bit signed and unsigned font-units

/// 16-bit signed quantity in font design units.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FWord(i16);

/// 16-bit unsigned quantity in font design units.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UfWord(u16);

impl FWord {
    pub fn new(raw: i16) -> Self {
        Self(raw)
    }
}

impl UfWord {
    pub fn new(raw: u16) -> Self {
        Self(raw)
    }
}

crate::newtype_scalar!(FWord, [u8; 2]);
crate::newtype_scalar!(UfWord, [u8; 2]);
//TODO: we can add addition/etc as needed
