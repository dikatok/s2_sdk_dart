use std::str::FromStr;

pub(crate) use s2_sdk::types::S2Error as IError;

pub struct S2Error {
    pub message: String,
}

impl From<IError> for S2Error {
    fn from(error: IError) -> Self {
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
