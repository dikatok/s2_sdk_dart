use std::{num::TryFromIntError, str::FromStr};

#[flutter_rust_bridge::frb(dart_code = r#"
    String toString() {
		return 'S2Error: $message';
    }
"#)]
pub struct S2Error {
    pub message: String,
}

impl From<s2_sdk::types::S2Error> for S2Error {
    fn from(value: s2_sdk::types::S2Error) -> Self {
        S2Error {
            message: value.to_string(),
        }
    }
}

impl From<s2_sdk::types::ValidationError> for S2Error {
    fn from(value: s2_sdk::types::ValidationError) -> Self {
        S2Error { message: value.0 }
    }
}

impl FromStr for S2Error {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(S2Error {
            message: s.to_string(),
        })
    }
}

impl From<TryFromIntError> for S2Error {
    fn from(value: TryFromIntError) -> Self {
        S2Error {
            message: value.to_string(),
        }
    }
}
