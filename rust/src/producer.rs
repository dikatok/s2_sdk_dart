use std::str::FromStr;

use flutter_rust_bridge::{RustAutoOpaqueNom, frb};

pub use s2_sdk::producer::{
    IndexedAppendAck as _IndexedAppendAck, Producer as IProducer,
    RecordSubmitTicket as _RecordSubmitTicket,
};

use crate::{
    error::S2Error,
    types::{AppendAck, AppendRecord},
};

#[frb(opaque)]
pub struct S2Producer {
    producer: RustAutoOpaqueNom<Option<IProducer>>,
}

impl S2Producer {
    pub fn new(producer: IProducer) -> S2Producer {
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

pub struct RecordSubmitTicket {
    pub ticket: RustAutoOpaqueNom<Option<_RecordSubmitTicket>>,
}

impl From<_RecordSubmitTicket> for RecordSubmitTicket {
    fn from(ticket: _RecordSubmitTicket) -> Self {
        RecordSubmitTicket {
            ticket: RustAutoOpaqueNom::new(Some(ticket)),
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

impl From<_IndexedAppendAck> for IndexedAppendAck {
    fn from(ack: _IndexedAppendAck) -> Self {
        IndexedAppendAck {
            seq_num: ack.seq_num,
            batch: ack.batch.into(),
        }
    }
}
