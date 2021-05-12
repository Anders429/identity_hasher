use core::fmt;
#[cfg(feature = "doc_item")]
use doc_item::docbox;
#[cfg(feature = "doc_item")]
use doc_item::since;
use serde::de;
use serde::de::MapAccess;
use serde::de::SeqAccess;
use serde::de::Visitor;
use serde::ser::SerializeStruct;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use IdentityHasher;

#[cfg_attr(feature = "doc_item", since(content = "1.13.0"))]
#[cfg_attr(feature = "doc_item", docbox(content = "This is supported on <strong>crate feature <code>serde</code></strong> only.", class="portability"))]
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

#[cfg_attr(feature = "doc_item", since(content = "1.13.0"))]
#[cfg_attr(feature = "doc_item", docbox(content = "This is supported on <strong>crate feature <code>serde</code></strong> only.", class="portability"))]
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
    use serde_test::assert_tokens;
    use serde_test::Token;
    use IdentityHasher;

    #[test]
    #[cfg_attr(not(debug_assertions), ignore)]
    fn serialize_and_deserialize_debug() {
        assert_tokens(
            &IdentityHasher::default(),
            &[
                Token::Struct {
                    name: "IdentityHasher",
                    len: 2,
                },
                Token::Str("hash"),
                Token::U64(0),
                Token::Str("used"),
                Token::Bool(false),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore)]
    fn serialize_and_deserialize_release() {
        assert_tokens(
            &IdentityHasher::default(),
            &[
                Token::Struct {
                    name: "IdentityHasher",
                    len: 1,
                },
                Token::Str("hash"),
                Token::U64(0),
                Token::StructEnd,
            ],
        );
    }
}
