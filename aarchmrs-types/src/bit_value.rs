/// A wrapper for restricted bit-sized value with debug assert.
///
/// The idea is to make it validate in debug only, in the release it is zero-cost, and the caller is responsible
/// for the validity of values.
pub struct BitValue<const BITS: u32>(pub u32);

impl<const BITS: u32> BitValue<BITS> {
    #[inline]
    pub fn new_u32(nested: u32) -> Self {
        debug_assert!(nested < (1 << BITS));

        Self(nested)
    }

    #[inline]
    pub fn new_i32(nested: i32) -> Self {
        debug_assert_eq!(
            {
                let bits = i32::BITS - BITS;
                (nested << bits) >> bits
            },
            nested
        );

        let mask = (1 << BITS) - 1;
        let nested = nested as u32;
        Self(nested & mask)
    }
}

impl<const BITS: u32> From<BitValue<BITS>> for u32 {
    #[inline]
    fn from(value: BitValue<BITS>) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_u32() {
        let _ = BitValue::<8>::new_u32(255);
    }

    #[test]
    fn test_valid_u32_1() {
        let _ = BitValue::<8>::new_u32(42);
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic)]
    fn test_invalid_u32() {
        let _ = BitValue::<8>::new_u32(256);
    }

    #[test]
    fn test_valid_i32() {
        let _ = BitValue::<8>::new_i32(42);
    }

    #[test]
    fn test_valid_i32_max() {
        let _ = BitValue::<8>::new_i32(i8::MAX.into());
    }

    #[test]
    fn test_valid_i32_min() {
        let _ = BitValue::<8>::new_i32(i8::MIN.into());
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic)]
    fn test_invalid_i32_max() {
        let max: i32 = i8::MAX.into();
        let _ = BitValue::<8>::new_i32(max + 1);
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic)]
    fn test_invalid_i32_min() {
        let min: i32 = i8::MIN.into();
        let _ = BitValue::<8>::new_i32(min - 1);
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic)]
    fn test_invalid_i32_i16max() {
        let max: i32 = i16::MAX.into();
        let _ = BitValue::<8>::new_i32(max);
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic)]
    fn test_invalid_i32_i16min() {
        let min: i32 = i16::MIN.into();
        let _ = BitValue::<8>::new_i32(min);
    }
}
