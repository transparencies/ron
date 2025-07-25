use alloc::{borrow::ToOwned, boxed::Box, string::String, vec::Vec};
use core::fmt;

use serde::{
    de::{Error, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};

use crate::{
    error::SpannedResult,
    value::{Map, Number, Value},
};

impl core::str::FromStr for Value {
    type Err = crate::error::SpannedError;

    /// Creates a value from a string reference.
    fn from_str(s: &str) -> SpannedResult<Self> {
        let mut de = super::Deserializer::from_str(s)?;

        let val = Value::deserialize(&mut de).map_err(|e| de.span_error(e))?;
        de.end().map_err(|e| de.span_error(e))?;

        Ok(val)
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a RON value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Bool(v))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Number(Number::new(v)))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Number(Number::new(v)))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Number(Number::new(v)))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Number(Number::new(v)))
    }

    #[cfg(feature = "integer128")]
    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Number(Number::new(v)))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Number(Number::new(v)))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Number(Number::new(v)))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Number(Number::new(v)))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Number(Number::new(v)))
    }

    #[cfg(feature = "integer128")]
    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Number(Number::new(v)))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Number(Number::new(v)))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Number(Number::new(v)))
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Char(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_string(v.to_owned())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::String(v))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_byte_buf(v.to_vec())
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Bytes(v))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Option(None))
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Value::Option(Some(Box::new(
            deserializer.deserialize_any(ValueVisitor)?,
        ))))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Unit)
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut vec = Vec::new();
        if let Some(cap) = seq.size_hint() {
            vec.reserve_exact(cap);
        }

        while let Some(x) = seq.next_element()? {
            vec.push(x);
        }

        Ok(Value::Seq(vec))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut res: Map = Map::new();

        #[cfg(feature = "indexmap")]
        if let Some(cap) = map.size_hint() {
            res.0.reserve_exact(cap);
        }

        while let Some(entry) = map.next_entry::<Value, Value>()? {
            res.insert(entry.0, entry.1);
        }

        Ok(Value::Map(res))
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use core::str::FromStr;

    use super::*;

    fn eval(s: &str) -> Value {
        s.parse().expect("Failed to parse")
    }

    #[test]
    fn test_none() {
        assert_eq!(eval("None"), Value::Option(None));
    }

    #[test]
    fn test_some() {
        assert_eq!(eval("Some(())"), Value::Option(Some(Box::new(Value::Unit))));
        assert_eq!(
            eval("Some  (  () )"),
            Value::Option(Some(Box::new(Value::Unit)))
        );
    }

    #[test]
    fn test_tuples_basic() {
        assert_eq!(
            eval("(3, 4.0, 5.0)"),
            Value::Seq(vec![
                Value::Number(Number::U8(3)),
                Value::Number(Number::F32(4.0.into())),
                Value::Number(Number::F32(5.0.into())),
            ],),
        );
    }

    #[test]
    fn test_tuples_ident() {
        assert_eq!(
            eval("(true, 3, 4, 5.0)"),
            Value::Seq(vec![
                Value::Bool(true),
                Value::Number(Number::U8(3)),
                Value::Number(Number::U8(4)),
                Value::Number(Number::F32(5.0.into())),
            ]),
        );
    }

    #[test]
    fn test_tuples_error() {
        use crate::de::{Error, Position, Span, SpannedError};

        assert_eq!(
            Value::from_str("Foo:").unwrap_err(),
            SpannedError {
                code: Error::TrailingCharacters,
                span: Span {
                    start: Position { line: 1, col: 4 },
                    end: Position { line: 1, col: 4 }
                }
            },
        );
    }

    #[test]
    fn test_floats() {
        assert_eq!(
            eval("(inf, -inf, NaN)"),
            Value::Seq(vec![
                Value::Number(Number::new(core::f32::INFINITY)),
                Value::Number(Number::new(core::f32::NEG_INFINITY)),
                Value::Number(Number::new(core::f32::NAN)),
            ]),
        );
    }

    #[test]
    fn test_complex() {
        assert_eq!(
            eval(
                "Some([
    Room ( width: 20, height: 5, name: \"The Room\" ),

    (
        width: 10.0,
        height: 10.0,
        name: \"Another room\",
        enemy_levels: {
            \"Enemy1\": 3,
            \"Enemy2\": 5,
            \"Enemy3\": 7,
        },
    ),
])"
            ),
            Value::Option(Some(Box::new(Value::Seq(vec![
                Value::Map(
                    vec![
                        (
                            Value::String("width".to_owned()),
                            Value::Number(Number::U8(20)),
                        ),
                        (
                            Value::String("height".to_owned()),
                            Value::Number(Number::U8(5)),
                        ),
                        (
                            Value::String("name".to_owned()),
                            Value::String("The Room".to_owned()),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                ),
                Value::Map(
                    vec![
                        (
                            Value::String("width".to_owned()),
                            Value::Number(Number::F32(10.0.into())),
                        ),
                        (
                            Value::String("height".to_owned()),
                            Value::Number(Number::F32(10.0.into())),
                        ),
                        (
                            Value::String("name".to_owned()),
                            Value::String("Another room".to_owned()),
                        ),
                        (
                            Value::String("enemy_levels".to_owned()),
                            Value::Map(
                                vec![
                                    (
                                        Value::String("Enemy1".to_owned()),
                                        Value::Number(Number::U8(3)),
                                    ),
                                    (
                                        Value::String("Enemy2".to_owned()),
                                        Value::Number(Number::U8(5)),
                                    ),
                                    (
                                        Value::String("Enemy3".to_owned()),
                                        Value::Number(Number::U8(7)),
                                    ),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                ),
            ]))))
        );
    }

    #[test]
    fn test_struct() {
        assert_eq!(
            eval("(a:42)"),
            Value::Map(
                [(
                    Value::String(String::from("a")),
                    Value::Number(Number::U8(42))
                )]
                .into_iter()
                .collect()
            ),
        );
        assert_eq!(
            eval("(r#a:42)"),
            Value::Map(
                [(
                    Value::String(String::from("a")),
                    Value::Number(Number::U8(42))
                )]
                .into_iter()
                .collect()
            ),
        );
        assert_eq!(
            "(r#:42)".parse::<Value>().unwrap_err(),
            crate::error::SpannedError {
                code: crate::Error::ExpectedString,
                span: crate::error::Span {
                    start: crate::error::Position { line: 1, col: 3 },
                    end: crate::error::Position { line: 1, col: 4 },
                }
            },
        );

        // Check for a failure in Deserializer::check_struct_type
        // - opening brace triggers the struct type check
        // - unclosed block comment fails the whitespace skip
        assert_eq!(
            "( /*".parse::<Value>().unwrap_err(),
            crate::error::SpannedError {
                code: crate::Error::UnclosedBlockComment,
                span: crate::error::Span {
                    start: crate::error::Position { line: 1, col: 3 },
                    end: crate::error::Position { line: 1, col: 5 },
                }
            },
        );
    }
}
