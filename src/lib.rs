#![cfg_attr(rustc_1_6, no_std)]

#[cfg(feature = "doc_item")]
extern crate doc_item;
#[cfg(feature = "serde")]
extern crate serde;
#[cfg(not(rustc_1_6))]
extern crate std as core;

use core::fmt;
use core::hash::Hasher;
use core::mem::transmute;
#[cfg(feature = "doc_item")]
use doc_item::since;
#[cfg(feature = "serde")]
use serde::de;
#[cfg(feature = "serde")]
use serde::de::MapAccess;
#[cfg(feature = "serde")]
use serde::de::SeqAccess;
#[cfg(feature = "serde")]
use serde::de::Visitor;
#[cfg(feature = "serde")]
use serde::ser::SerializeStruct;
#[cfg(feature = "serde")]
use serde::Deserialize;
#[cfg(feature = "serde")]
use serde::Deserializer;
#[cfg(feature = "serde")]
use serde::Serialize;
#[cfg(feature = "serde")]
use serde::Serializer;

#[cfg_attr(feature = "doc_item", since(content = "1.0.0"))]
#[derive(Clone, Debug, Default)]
pub struct IdentityHasher {
    hash: u64,
    #[cfg(debug_assertions)]
    used: bool,
}

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
        #[cfg(rustc_1_3)]
        #[cfg_attr(feature = "doc_item", since(content = "1.3.0"))]
        #[inline]
        fn $_fn(&mut self, i: $int_type) {
            debug_assert_unused!(self);
            self.hash = i as u64;
        }
    };
}

macro_rules! write_integer_unavailable {
    ($_fn:ident, $int_type:ty) => {
        #[cfg_attr(feature = "doc_item", since(content = "1.26.0"))]
        #[inline]
        fn $_fn(&mut self, _i: $int_type) {
            panic!("IdentityHasher cannot hash an {}.", stringify!($int_type));
        }
    };
}

impl Hasher for IdentityHasher {
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

    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "doc_item", since(content = "1.13.0"))]
impl Serialize for IdentityHasher {
    #[cfg(debug_assertions)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = try!(serializer.serialize_struct("IdentityHasher", 2));
        try!(state.serialize_field("hash", &self.hash));
        try!(state.serialize_field("used", &self.used));
        state.end()
    }

    #[cfg(not(debug_assertions))]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = try!(serializer.serialize_struct("IdentityHasher", 1));
        try!(state.serialize_field("hash", &self.hash));
        state.end()
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "doc_item", since(content = "1.13.0"))]
impl<'de> Deserialize<'de> for IdentityHasher {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[cfg(debug_assertions)]
        enum Field {
            Hash,
            Used,
        }

        #[cfg(not(debug_assertions))]
        enum Field {
            Hash,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    #[cfg(debug_assertions)]
                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`hash` or `used`")
                    }

                    #[cfg(not(debug_assertions))]
                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`hash`")
                    }

                    #[cfg(debug_assertions)]
                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "hash" => Ok(Field::Hash),
                            "used" => Ok(Field::Used),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }

                    #[cfg(not(debug_assertions))]
                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "hash" => Ok(Field::Hash),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct IdentityHasherVisitor;

        impl<'de> Visitor<'de> for IdentityHasherVisitor {
            type Value = IdentityHasher;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct IdentityHasher")
            }

            #[cfg(debug_assertions)]
            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let hash = match try!(seq.next_element()) {
                    Some(value) => value,
                    None => return Err(de::Error::invalid_length(0, &self)),
                };
                let used = match try!(seq.next_element()) {
                    Some(value) => value,
                    None => return Err(de::Error::invalid_length(1, &self)),
                };
                Ok(IdentityHasher {
                    hash: hash,
                    used: used,
                })
            }

            #[cfg(not(debug_assertions))]
            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let hash = match try!(seq.next_element()) {
                    Some(value) => value,
                    None => return Err(de::Error::invalid_length(0, &self)),
                };
                Ok(IdentityHasher { hash: hash })
            }

            #[cfg(debug_assertions)]
            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut hash = None;
                let mut used = None;
                while let Some(key) = try!(map.next_key()) {
                    match key {
                        Field::Hash => {
                            if hash.is_some() {
                                return Err(de::Error::duplicate_field("hash"));
                            }
                            hash = Some(try!(map.next_value()));
                        }
                        Field::Used => {
                            if used.is_some() {
                                return Err(de::Error::duplicate_field("used"));
                            }
                            used = Some(try!(map.next_value()));
                        }
                    }
                }
                let hash = match hash {
                    Some(value) => value,
                    None => return Err(de::Error::missing_field("hash")),
                };
                let used = match used {
                    Some(value) => value,
                    None => return Err(de::Error::missing_field("used")),
                };
                Ok(IdentityHasher {
                    hash: hash,
                    used: used,
                })
            }

            #[cfg(not(debug_assertions))]
            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut hash = None;
                while let Some(key) = try!(map.next_key()) {
                    match key {
                        Field::Hash => {
                            if hash.is_some() {
                                return Err(de::Error::duplicate_field("hash"));
                            }
                            hash = Some(try!(map.next_value()));
                        }
                    }
                }
                let hash = match hash {
                    Some(value) => value,
                    None => return Err(de::Error::missing_field("hash")),
                };
                Ok(IdentityHasher { hash: hash })
            }
        }

        #[cfg(debug_assertions)]
        const FIELDS: &'static [&'static str] = &["hash", "used"];
        #[cfg(not(debug_assertions))]
        const FIELDS: &'static [&'static str] = &["hash"];
        deserializer.deserialize_struct("IdentityHasher", FIELDS, IdentityHasherVisitor)
    }
}

#[cfg(test)]
mod tests {
    use core::hash::Hasher;
    use core::mem::transmute;
    use IdentityHasher;

    #[test]
    fn write() {
        let mut hasher = IdentityHasher::default();

        hasher.write(&unsafe {
            // SAFETY: [u8; 8] and u64 are the same size, and any representation of a u64 will
            // correspond to a valid value of [u8; 8].
            transmute::<u64, [u8; 8]>(42)
        });

        assert_eq!(hasher.finish(), 42);
    }

    #[test]
    fn write_less_than_8_bytes() {
        let mut hasher = IdentityHasher::default();

        hasher.write(&unsafe {
            // SAFETY: [u8; 1] and u8 are the same size, and any representation of a u8 will
            // correspond to a valid value of [u8; 1].
            transmute::<u8, [u8; 1]>(42)
        });

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
