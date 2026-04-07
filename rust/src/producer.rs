use std::str::FromStr;

use flutter_rust_bridge::{RustAutoOpaqueNom, frb};

use crate::{
    error::S2Error,
    types::{AppendAck, AppendRecord},
};

#[frb(opaque)]
pub struct S2Producer {
    producer: RustAutoOpaqueNom<Option<s2_sdk::producer::Producer>>,
}

impl S2Producer {
    pub(crate) fn new(producer: s2_sdk::producer::Producer) -> S2Producer {
        S2Producer {
            producer: RustAutoOpaqueNom::new(Some(producer)),
        }
    }

    pub async fn submit(&self, record: AppendRecord) -> Result<RecordSubmitTicket, S2Error> {
        match self
            .producer
            .try_read()
            .unwrap()
            .as_ref()
            .unwrap()
            .submit(record.into())
            .await
        {
            Ok(ticket) => Ok(ticket.into()),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn close(self) -> Result<(), S2Error> {
        let guard = self.producer.try_write().unwrap().take();
        if let Some(producer) = guard {
            match producer.close().await {
                Ok(()) => return Ok(()),
                Err(e) => return Err(e.into()),
            }
        }
        Err(S2Error::from_str("Producer is already closed").unwrap())
    }
}

#[frb(opaque)]
pub struct RecordSubmitTicket {
    pub(crate) ticket: RustAutoOpaqueNom<Option<s2_sdk::producer::RecordSubmitTicket>>,
}

impl From<s2_sdk::producer::RecordSubmitTicket> for RecordSubmitTicket {
    fn from(value: s2_sdk::producer::RecordSubmitTicket) -> Self {
        RecordSubmitTicket {
            ticket: RustAutoOpaqueNom::new(Some(value)),
        }
    }
}

impl RecordSubmitTicket {
    pub async fn ack(self) -> Result<IndexedAppendAck, S2Error> {
        match self.ticket.try_write().unwrap().take().unwrap().await {
            Ok(ack) => Ok(ack.into()),
            Err(e) => Err(e.into()),
        }
    }
}

pub struct IndexedAppendAck {
    pub seq_num: u64,
    pub batch: AppendAck,
}

impl From<s2_sdk::producer::IndexedAppendAck> for IndexedAppendAck {
    fn from(value: s2_sdk::producer::IndexedAppendAck) -> Self {
        IndexedAppendAck {
            seq_num: value.seq_num,
            batch: value.batch.into(),
        }
    }
}
