use std::str::FromStr;

pub struct S2Error {
    pub message: String,
}

impl From<s2_sdk::types::S2Error> for S2Error {
    fn from(error: s2_sdk::types::S2Error) -> Self {
        S2Error {
            message: error.to_string(),
        }
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
