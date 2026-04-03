use std::str::FromStr;

use flutter_rust_bridge::{RustAutoOpaqueNom, frb};

pub(crate) use s2_sdk::{S2Basin as _Basin, types::StreamName};

use crate::stream::S2Stream;

#[frb(opaque)]
pub struct S2Basin {
    basin: RustAutoOpaqueNom<_Basin>,
}

impl S2Basin {
    pub(crate) fn new(basin: _Basin) -> S2Basin {
        S2Basin {
            basin: RustAutoOpaqueNom::new(basin),
        }
    }

    pub async fn stream(&self, name: String) -> S2Stream {
        self.basin
            .try_read()
            .unwrap()
            .stream(StreamName::from_str(name.as_str()).unwrap())
            .into()
    }

    pub async fn list_streams(&self) {}

    pub async fn list_all_streams(&self) {}

    pub async fn create_stream(&self) {}

    pub async fn create_or_reconfigure_stream(&self) {}

    pub async fn get_stream_config(&self) {}

    pub async fn delete_stream(&self) {}

    pub async fn reconfigure_stream(&self) {}
}
