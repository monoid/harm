use core::convert::From;
use core::fmt;

use aarchmrs_types::BitValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitError {
    Overflow { significant_bits: u8, align: u8 },
    SignOutRange,
    Alignment { align: u8 },
}

impl core::error::Error for BitError {}

impl fmt::Display for BitError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            BitError::Overflow {
                significant_bits,
                align,
            } => {
                write!(f, "Value overflow for {} bits", significant_bits + align)
            }
            BitError::SignOutRange => {
                write!(f, "Cannot represent the value as signed/unsigned")
            }
            BitError::Alignment { align } => {
                write!(f, "Value not aligned to {align} bits")
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct UBitValue<const SIGNIFICANT_BITS: u32, const ALIGN: u32 = 0>(u32);

impl<const SIGNIFICANT_BITS: u32, const ALIGN: u32> UBitValue<SIGNIFICANT_BITS, ALIGN> {
    const CHECK_MIN: () = assert!(SIGNIFICANT_BITS > 0, "BITS must be greater than 0");
    const CHECK_MAX: () = assert!(
        SIGNIFICANT_BITS + ALIGN <= 32,
        "SIGNIFICANT_BITS + ALIGN must be <= 32"
    );

    pub const fn new(value: u32) -> Result<Self, BitError> {
        let () = Self::CHECK_MIN;
        let () = Self::CHECK_MAX;

        let shifted_value = {
            if value == (value >> ALIGN) << ALIGN {
                value >> ALIGN
            } else {
                return Err(BitError::Alignment { align: ALIGN as _ });
            }
        };
        let check_upper_bits = u32::BITS - SIGNIFICANT_BITS;
        let there_and_back_again = (shifted_value << check_upper_bits) >> check_upper_bits;
        if shifted_value == there_and_back_again {
            Ok(Self(shifted_value))
        } else {
            // TODO Frodo not being the same after there and back again shoudn't be an error.
            Err(BitError::Overflow {
                significant_bits: SIGNIFICANT_BITS as _,
                align: ALIGN as _,
            })
        }
    }

    pub fn new_i32(value: i32) -> Result<Self, BitError> {
        match value.try_into() {
            Ok(unsigned) => Self::new(unsigned),
            Err(_) => Err(BitError::SignOutRange),
        }
    }

    pub const fn bits(self) -> u32 {
        self.0
    }
}

impl<const SIGNIFICANT_BITS: u32, const ALIGN: u32> TryFrom<u32>
    for UBitValue<SIGNIFICANT_BITS, ALIGN>
{
    type Error = BitError;

    #[inline]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<const SIGNIFICANT_BITS: u32, const ALIGN: u32> TryFrom<i32>
    for UBitValue<SIGNIFICANT_BITS, ALIGN>
{
    type Error = BitError;

    #[inline]
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::new_i32(value)
    }
}

impl From<bool> for UBitValue<1> {
    fn from(value: bool) -> Self {
        Self(value as u32)
    }
}

impl<const SIGNIFICANT_BITS: u32, const ALIGN: u32> From<UBitValue<SIGNIFICANT_BITS, ALIGN>>
    for BitValue<SIGNIFICANT_BITS>
{
    fn from(value: UBitValue<SIGNIFICANT_BITS, ALIGN>) -> Self {
        Self(value.0)
    }
}

impl<const SIGNIFICANT_BITS: u32, const ALIGN: u32> TryFrom<u32>
    for SBitValue<SIGNIFICANT_BITS, ALIGN>
{
    type Error = BitError;

    #[inline]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new_u32(value)
    }
}

impl<const SIGNIFICANT_BITS: u32, const ALIGN: u32> TryFrom<i32>
    for SBitValue<SIGNIFICANT_BITS, ALIGN>
{
    type Error = BitError;

    #[inline]
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct SBitValue<const SIGNIFICANT_BITS: u32, const ALIGN: u32 = 0>(u32);

impl<const SIGNIFICANT_BITS: u32, const ALIGN: u32> SBitValue<SIGNIFICANT_BITS, ALIGN> {
    pub const fn new(value: i32) -> Result<Self, BitError> {
        let shifted_value = value >> ALIGN;
        if value != shifted_value << ALIGN {
            return Err(BitError::Alignment { align: ALIGN as _ });
        }
        // TODO comparing ranges may be cleaner, and produces exactly same code.
        let upper_bits = i32::BITS - SIGNIFICANT_BITS;
        let there_and_back_again = (shifted_value << upper_bits) >> upper_bits;
        if shifted_value == there_and_back_again {
            let mask = u32::MAX >> upper_bits;
            let nested = (shifted_value as u32) & mask;
            Ok(Self(nested))
        } else {
            Err(BitError::Overflow {
                significant_bits: SIGNIFICANT_BITS as _,
                align: ALIGN as _,
            })
        }
    }

    pub fn new_u32(value: u32) -> Result<Self, BitError> {
        match value.try_into() {
            Ok(signed) => Self::new(signed),
            Err(_) => Err(BitError::SignOutRange),
        }
    }

    pub const fn bits(self) -> u32 {
        self.0
    }
}

impl<const SIGNIFICANT_BITS: u32, const ALIGN: u32> From<SBitValue<SIGNIFICANT_BITS, ALIGN>>
    for BitValue<SIGNIFICANT_BITS>
{
    fn from(value: SBitValue<SIGNIFICANT_BITS, ALIGN>) -> Self {
        BitValue(value.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ubitvalue_new_valid() {
        type U4 = UBitValue<4>;
        let val = U4::new(0b1010).unwrap();
        assert_eq!(val.bits(), 0b1010);
    }

    #[test]
    fn test_ubitvalue_new_with_align_valid() {
        type U4A1 = UBitValue<4, 1>;
        let val = U4A1::new(0b1010).unwrap(); // 0b1010 >> 1 = 0b0101
        assert_eq!(val.bits(), 0b0101);
    }

    #[test]
    fn test_ubitvalue_new_overflow() {
        type U3 = UBitValue<3>;
        let result = U3::new(0b1000); // 8 doesn't fit into 3 bits
        assert!(matches!(
            result,
            Err(BitError::Overflow {
                significant_bits: 3,
                align: 0
            })
        ));
    }

    #[test]
    fn test_ubitvalue_new_alignment_error() {
        type U4A2 = UBitValue<4, 2>;
        let result = U4A2::new(0b0111); // not aligned to 2 bits
        assert!(matches!(result, Err(BitError::Alignment { align: 2 })));
    }

    #[test]
    fn test_ubitvalue_from_bool() {
        let val_true = UBitValue::<1>::from(true);
        let val_false = UBitValue::<1>::from(false);
        assert_eq!(val_true.bits(), 1);
        assert_eq!(val_false.bits(), 0);
    }

    #[test]
    fn test_ubitvalue_into_bitvalue0() {
        type U5 = UBitValue<5, 0>;
        let val = U5::new(0b10101).unwrap();
        let bv: BitValue<5> = val.into();
        assert_eq!(bv.0, 0b10101);
    }

    #[test]
    fn test_ubitvalue_into_bitvalue() {
        type U5 = UBitValue<5, 2>;
        let val = U5::new(0b1010100).unwrap();
        let bv: BitValue<5> = val.into();
        assert_eq!(bv.0, 0b10101);
    }

    #[test]
    fn test_sbitvalue_new_valid_positive() {
        type S4 = SBitValue<4>;
        let val = S4::new(7).unwrap(); // fits in 4 signed bits
        assert_eq!(val.bits(), 0b0111);
    }

    #[test]
    fn test_sbitvalue_new_valid_negative() {
        type S4 = SBitValue<4>;
        let val = S4::new(-8).unwrap(); // -8 fits in 4 signed bits
        assert_eq!(val.bits(), 0b1000); // sign extended and masked
    }

    #[test]
    fn test_sbitvalue_new_overflow() {
        type S4 = SBitValue<4>;
        let result = S4::new(8); // 8 does not fit in 4 signed bits
        assert!(matches!(
            result,
            Err(BitError::Overflow {
                significant_bits: 4,
                ..
            })
        ));
    }

    #[test]
    fn test_sbitvalue_new_alignment_error() {
        type S4A2 = SBitValue<4, 2>;
        let result = S4A2::new(3); // not aligned to 2 bits
        assert!(matches!(result, Err(BitError::Alignment { align: 2 })));
    }

    #[test]
    fn test_sbitvalue_into_bitvalue() {
        type S5 = SBitValue<5>;
        let val = S5::new(-7).unwrap();
        let bv: BitValue<5> = val.into();
        assert_eq!(bv.0, val.bits());
    }

    #[test]
    fn test_ubitvalue_new_i32() {
        let res = UBitValue::<7, 1>::new_i32(42);
        assert_eq!(res, UBitValue::<7, 1>::new(42))
    }

    #[test]
    fn test_ubitvalue_new_i32_overflow_error() {
        let res = UBitValue::<7, 1>::new_i32(256);
        assert_eq!(
            res,
            Err(BitError::Overflow {
                significant_bits: 7,
                align: 1
            })
        );
    }

    #[test]
    fn test_ubitvalue_new_i32_sign_error() {
        let res = UBitValue::<7, 1>::new_i32(-1);
        assert_eq!(res, Err(BitError::SignOutRange));
    }

    #[test]
    fn test_sbitvalue_new_u32() {
        let res = SBitValue::<7, 1>::new_u32(42);
        assert_eq!(res, SBitValue::<7, 1>::new(42))
    }

    #[test]
    fn test_sbitvalue_new_u32_overflow_error() {
        let res = SBitValue::<9>::new_u32(256);
        assert_eq!(
            res,
            Err(BitError::Overflow {
                significant_bits: 9,
                align: 0
            })
        );
    }

    #[test]
    fn test_sbitvalue_new_u32_sign_error() {
        let res = SBitValue::<8>::new_u32(u32::MAX);
        assert_eq!(res, Err(BitError::SignOutRange));
    }
}
