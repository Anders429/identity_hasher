use std::hash::Hasher;

#[derive(Clone, Debug, Default)]
pub struct IdentityHasher {
    hash: u64,
    #[cfg(debug_assertions)]
    used: bool,
}

macro_rules! debug_assert_unused {
    ($self:ident) => {
        #[cfg(debug_assertions)]
        {
            assert!(!$self.used, "IdentityHasher can only write a single time.");
            $self.used = true;
        }
    };
}

macro_rules! write_integer {
    ($fn:ident, $int_type:ty) => {
        fn $fn(&mut self, i: $int_type) {
            debug_assert_unused!(self);
            self.hash = i as u64;
        }
    };
}

macro_rules! write_integer_unavailable {
    ($fn:ident, $int_type:ty) => {
        fn $fn(&mut self, _i: $int_type) {
            panic!("IdentityHasher cannot hash an {}.", stringify!($int_type));
        }
    };
}

impl Hasher for IdentityHasher {
    fn write(&mut self, bytes: &[u8]) {
        debug_assert_unused!(self);

        assert!(
            bytes.len() <= 8,
            "IdentityHasher cannot write {} bytes. Maximum is 8.",
            bytes.len()
        );

        let mut u64_bytes = [0; 8];
        // SAFETY: Asserted above that `bytes.len()` was not more than 8.
        unsafe { u64_bytes.get_unchecked_mut(..bytes.len()) }.copy_from_slice(bytes);
        self.hash = u64::from_ne_bytes(u64_bytes);
    }

    write_integer!(write_u8, u8);
    write_integer!(write_u16, u16);
    write_integer!(write_u32, u32);
    write_integer!(write_u64, u64);
    write_integer!(write_usize, usize);
    write_integer!(write_i8, i8);
    write_integer!(write_i16, i16);
    write_integer!(write_i32, i32);
    write_integer!(write_i64, i64);
    write_integer!(write_isize, isize);

    write_integer_unavailable!(write_u128, u128);
    write_integer_unavailable!(write_i128, i128);

    fn finish(&self) -> u64 {
        self.hash
    }
}

#[cfg(test)]
mod tests {
    use crate::IdentityHasher;
    use std::hash::Hasher;

    #[test]
    fn write() {
        let mut hasher = IdentityHasher::default();

        hasher.write(&42u64.to_ne_bytes());

        assert_eq!(hasher.finish(), 42);
    }

    #[test]
    fn write_less_than_8_bytes() {
        let mut hasher = IdentityHasher::default();

        hasher.write(&42u8.to_ne_bytes());

        assert_eq!(hasher.finish(), 42);
    }

    #[test]
    #[should_panic(expected = "IdentityHasher cannot write 16 bytes. Maximum is 8.")]
    fn write_more_than_8_bytes() {
        let mut hasher = IdentityHasher::default();

        hasher.write(&42u128.to_ne_bytes());
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = "IdentityHasher can only write a single time.")
    )]
    fn write_twice() {
        let mut hasher = IdentityHasher::default();

        hasher.write(&42u64.to_ne_bytes());
        hasher.write(&42u64.to_ne_bytes());
    }

    #[test]
    fn write_u8() {
        let mut hasher = IdentityHasher::default();

        hasher.write_u8(42);

        assert_eq!(hasher.finish(), 42);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = "IdentityHasher can only write a single time.")
    )]
    fn write_u8_twice() {
        let mut hasher = IdentityHasher::default();

        hasher.write_u8(42);
        hasher.write_u8(42);
    }

    #[test]
    fn write_u16() {
        let mut hasher = IdentityHasher::default();

        hasher.write_u16(42);

        assert_eq!(hasher.finish(), 42);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = "IdentityHasher can only write a single time.")
    )]
    fn write_u16_twice() {
        let mut hasher = IdentityHasher::default();

        hasher.write_u16(42);
        hasher.write_u16(42);
    }

    #[test]
    fn write_u32() {
        let mut hasher = IdentityHasher::default();

        hasher.write_u32(42);

        assert_eq!(hasher.finish(), 42);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = "IdentityHasher can only write a single time.")
    )]
    fn write_u32_twice() {
        let mut hasher = IdentityHasher::default();

        hasher.write_u32(42);
        hasher.write_u32(42);
    }

    #[test]
    fn write_u64() {
        let mut hasher = IdentityHasher::default();

        hasher.write_u64(42);

        assert_eq!(hasher.finish(), 42);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = "IdentityHasher can only write a single time.")
    )]
    fn write_u64_twice() {
        let mut hasher = IdentityHasher::default();

        hasher.write_u64(42);
        hasher.write_u64(42);
    }

    #[test]
    fn write_usize() {
        let mut hasher = IdentityHasher::default();

        hasher.write_usize(42);

        assert_eq!(hasher.finish(), 42);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = "IdentityHasher can only write a single time.")
    )]
    fn write_usize_twice() {
        let mut hasher = IdentityHasher::default();

        hasher.write_usize(42);
        hasher.write_usize(42);
    }

    #[test]
    fn write_i8() {
        let mut hasher = IdentityHasher::default();

        hasher.write_i8(42);

        assert_eq!(hasher.finish(), 42);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = "IdentityHasher can only write a single time.")
    )]
    fn write_i8_twice() {
        let mut hasher = IdentityHasher::default();

        hasher.write_i8(42);
        hasher.write_i8(42);
    }

    #[test]
    fn write_i16() {
        let mut hasher = IdentityHasher::default();

        hasher.write_i16(42);

        assert_eq!(hasher.finish(), 42);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = "IdentityHasher can only write a single time.")
    )]
    fn write_i16_twice() {
        let mut hasher = IdentityHasher::default();

        hasher.write_i16(42);
        hasher.write_i16(42);
    }

    #[test]
    fn write_i32() {
        let mut hasher = IdentityHasher::default();

        hasher.write_i32(42);

        assert_eq!(hasher.finish(), 42);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = "IdentityHasher can only write a single time.")
    )]
    fn write_i32_twice() {
        let mut hasher = IdentityHasher::default();

        hasher.write_i32(42);
        hasher.write_i32(42);
    }

    #[test]
    fn write_i64() {
        let mut hasher = IdentityHasher::default();

        hasher.write_i64(42);

        assert_eq!(hasher.finish(), 42);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = "IdentityHasher can only write a single time.")
    )]
    fn write_i64_twice() {
        let mut hasher = IdentityHasher::default();

        hasher.write_i64(42);
        hasher.write_i64(42);
    }

    #[test]
    fn write_isize() {
        let mut hasher = IdentityHasher::default();

        hasher.write_isize(42);

        assert_eq!(hasher.finish(), 42);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = "IdentityHasher can only write a single time.")
    )]
    fn write_isize_twice() {
        let mut hasher = IdentityHasher::default();

        hasher.write_isize(42);
        hasher.write_isize(42);
    }

    #[test]
    #[should_panic(expected = "IdentityHasher cannot hash an u128.")]
    fn write_u128() {
        let mut hasher = IdentityHasher::default();

        hasher.write_u128(42);
    }

    #[test]
    #[should_panic(expected = "IdentityHasher cannot hash an i128.")]
    fn write_i128() {
        let mut hasher = IdentityHasher::default();

        hasher.write_i128(42);
    }
}
