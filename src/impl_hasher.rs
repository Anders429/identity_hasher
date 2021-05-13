use core::hash::Hasher;
use core::mem::transmute;
#[cfg(feature = "doc_item")]
use doc_item::since;
use IdentityHasher;

#[cfg(debug_assertions)]
macro_rules! debug_assert_unused {
    ($_self:ident) => {
        assert!(!$_self.used, "IdentityHasher can only write a single time.");
        $_self.used = true;
    };
}

#[cfg(not(debug_assertions))]
macro_rules! debug_assert_unused {
    ($_self:ident) => {};
}

macro_rules! write_integer {
    ($_fn:ident, $int_type:ty) => {
        #[doc = concat!("Writes a single `", stringify!($int_type), "` into this hasher.")]
        ///
        /// If the hasher has been written to previously, this method will panic on debug builds.
        #[inline]
        fn $_fn(&mut self, i: $int_type) {
            debug_assert_unused!(self);
            self.hash = i as u64;
        }
    };
}

macro_rules! write_integer_unavailable {
    ($_fn:ident, $int_type:ty) => {
        /// This method is unavailable for this hasher, as it requires writing more than 8 bytes.
        /// Using it will result in a panic.
        #[cfg_attr(feature = "doc_item", since(content = "1.26.0"))]
        #[inline]
        fn $_fn(&mut self, _i: $int_type) {
            panic!("IdentityHasher cannot hash an {}.", stringify!($int_type));
        }
    };
}

/// [`Hasher`] implementation for `IdentityHasher`.
///
/// Only one of the write methods provided here may ever be called. Multiple writes will panic on
/// debug builds, but the checks are optimized away on release builds. Improper use on release will
/// not trigger any undefined behavior, but it is not considered correct and will likely not
/// function as expected.
///
/// As the `finish()` method must return a [`u64`], only a maximum of 8 bytes may be written.
/// Attempting to write either a [`u128`] or [`i128`] will panic on both debug and release builds.
/// Attempting to write more than 8 bytes using the `write()` method will panic on debug builds and
/// will only write the first 8 bytes on release builds.
///
/// If a type cannot be hashed according to the above criteria, a different hasher should be used.
///
/// # Example
/// ```
/// use identity_hasher::IdentityHasher;
/// use std::hash::Hasher;
///
/// let mut hasher = IdentityHasher::default();
///
/// hasher.write_u8(42);
///
/// assert_eq!(hasher.finish(), 42);
/// ```
///
/// [`Hasher`]: core::hash::Hasher
impl Hasher for IdentityHasher {
    /// Write some data into this hasher.
    ///
    /// This method should only write up to 8 bytes. It will panic on debug builds if more than 8
    /// bytes are provided, and on release builds it will only write the first 8 bytes.
    ///
    /// If the hasher has been written to previously, this method will panic on debug builds.
    #[inline]
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

    /// Returns the written value.
    #[inline]
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

        hasher.write(&[42, 0, 0, 0, 0, 0, 0, 0]);

        assert_eq!(hasher.finish(), 42);
    }

    #[test]
    fn write_less_than_8_bytes() {
        let mut hasher = IdentityHasher::default();

        hasher.write(&[42]);

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

        hasher.write(&[42, 0, 0, 0, 0, 0, 0, 0]);
        hasher.write(&[42, 0, 0, 0, 0, 0, 0, 0]);
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
