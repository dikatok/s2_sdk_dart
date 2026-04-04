use std::str::FromStr;

use flutter_rust_bridge::{RustAutoOpaqueNom, frb};

use crate::stream::S2Stream;

#[frb(opaque)]
pub struct S2Basin {
    basin: RustAutoOpaqueNom<s2_sdk::S2Basin>,
}

impl S2Basin {
    pub(crate) fn new(basin: s2_sdk::S2Basin) -> S2Basin {
        S2Basin {
            basin: RustAutoOpaqueNom::new(basin),
        }
    }

    pub async fn stream(&self, name: String) -> S2Stream {
        self.basin
            .try_read()
            .unwrap()
            .stream(s2_sdk::types::StreamName::from_str(name.as_str()).unwrap())
            .into()
    }

    // #[frb(stream_dart_await)]
    // pub async fn list_streams(
    //     &self,
    //     sink: StreamSink<SequencedRecord>,
    // ) -> anyhow::Result<(), S2Error> {
    //     self.basin.try_read().unwrap().list_streams(input);
    //     Ok(())
    // }

    pub async fn list_all_streams(&self) {}

    pub async fn create_stream(&self) {}

    pub async fn create_or_reconfigure_stream(&self) {}

    pub async fn get_stream_config(&self) {}

    pub async fn delete_stream(&self) {}

    pub async fn reconfigure_stream(&self) {}
}

// pub struct ListStreamsInput {
//     pub prefix: Option<String>,
//     pub start_after: Option<String>,
//     pub limit: Option<usize>,
// }

// impl From<ListStreamsInput> for s2_sdk::types::ListStreamsInput {
//     fn from(value: ListStreamsInput) -> Self {
//         let mut input = s2_sdk::types::ListStreamsInput::default();
//         if let Some(prefix) = value.prefix {
//             input = input
//                 .with_prefix(s2_sdk::types::StreamNamePrefix::from_str(prefix.as_str()).unwrap());
//         }
//         if let Some(start_after) = value.start_after {
//             input = input
//                 .with_start_after(StreamNameStartAfter::from_str(start_after.as_str()).unwrap());
//         }
//         if let Some(limit) = value.limit {
//             input = input.with_limit(limit);
//         }
//         input
//     }
// }
