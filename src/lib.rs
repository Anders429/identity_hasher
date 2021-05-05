#![no_std]

use core::hash::Hasher;
use core::mem::transmute;

#[derive(Clone, Debug, Default)]
pub struct IdentityHasher {
    hash: u64,
    #[cfg(debug_assertions)]
    used: bool,
}

macro_rules! debug_assert_unused {
    ($_self:ident) => {
        #[cfg(debug_assertions)]
        {
            assert!(!$_self.used, "IdentityHasher can only write a single time.");
            $_self.used = true;
        }
    };
}

macro_rules! write_integer {
    ($_fn:ident, $int_type:ty) => {
        fn $_fn(&mut self, i: $int_type) {
            debug_assert_unused!(self);
            self.hash = i as u64;
        }
    };
}

macro_rules! write_integer_unavailable {
    ($_fn:ident, $int_type:ty) => {
        fn $_fn(&mut self, _i: $int_type) {
            panic!("IdentityHasher cannot hash an {}.", stringify!($int_type));
        }
    };
}

impl Hasher for IdentityHasher {
    fn write(&mut self, bytes: &[u8]) {
        debug_assert_unused!(self);
        debug_assert!(
            bytes.len() <= 8,
            "IdentityHasher cannot write {} bytes. Maximum is 8.",
            bytes.len()
        );

        let mut u64_bytes = [0; 8];
        u64_bytes
            .iter_mut()
            .zip(bytes.iter())
            .fold((), |_, (u64_byte, byte)| *u64_byte = *byte);
        self.hash = unsafe {
            // SAFETY: [u8; 8] and u64 are the same size, and any representation of 8 bytes will
            // correspond to a valid u64 value.
            transmute::<[u8; 8], u64>(u64_bytes)
        };
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

    #[cfg(has_u128)]
    write_integer_unavailable!(write_u128, u128);
    #[cfg(has_i128)]
    write_integer_unavailable!(write_i128, i128);

    fn finish(&self) -> u64 {
        self.hash
    }
}

#[cfg(test)]
mod tests {
    use core::hash::Hasher;
    use IdentityHasher;

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
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = "IdentityHasher cannot write 16 bytes. Maximum is 8.")
    )]
    fn write_more_than_8_bytes() {
        let mut hasher = IdentityHasher::default();

        hasher.write(&[0; 16]);
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

    macro_rules! test_write_integer {
        ($_fn:ident) => {
            #[test]
            fn $_fn() {
                let mut hasher = IdentityHasher::default();

                hasher.$_fn(42);

                assert_eq!(hasher.finish(), 42);
            }
        };
    }

    macro_rules! test_write_integer_twice {
        ($_fn:ident, $method:ident) => {
            #[test]
            #[cfg_attr(
                debug_assertions,
                should_panic(expected = "IdentityHasher can only write a single time.")
            )]
            fn $_fn() {
                let mut hasher = IdentityHasher::default();

                hasher.$method(42);
                hasher.$method(42);
            }
        };
    }

    test_write_integer!(write_u8);
    test_write_integer!(write_u16);
    test_write_integer!(write_u32);
    test_write_integer!(write_u64);
    test_write_integer!(write_usize);
    test_write_integer!(write_i8);
    test_write_integer!(write_i16);
    test_write_integer!(write_i32);
    test_write_integer!(write_i64);
    test_write_integer!(write_isize);

    test_write_integer_twice!(write_u8_twice, write_u8);
    test_write_integer_twice!(write_u16_twice, write_u16);
    test_write_integer_twice!(write_u32_twice, write_u32);
    test_write_integer_twice!(write_u64_twice, write_u64);
    test_write_integer_twice!(write_usize_twice, write_usize);
    test_write_integer_twice!(write_i8_twice, write_i8);
    test_write_integer_twice!(write_i16_twice, write_i16);
    test_write_integer_twice!(write_i32_twice, write_i32);
    test_write_integer_twice!(write_i64_twice, write_i64);
    test_write_integer_twice!(write_isize_twice, write_isize);

    #[cfg(has_u128)]
    #[test]
    #[should_panic(expected = "IdentityHasher cannot hash an u128.")]
    fn write_u128() {
        let mut hasher = IdentityHasher::default();

        hasher.write_u128(42);
    }

    #[cfg(has_i128)]
    #[test]
    #[should_panic(expected = "IdentityHasher cannot hash an i128.")]
    fn write_i128() {
        let mut hasher = IdentityHasher::default();

        hasher.write_i128(42);
    }
}
