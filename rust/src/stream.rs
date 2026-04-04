use flutter_rust_bridge::{RustAutoOpaqueNom, frb};
use tokio_stream::StreamExt;

use crate::{
    append_session::S2AppendSession,
    error::S2Error,
    frb_generated::StreamSink,
    producer::S2Producer,
    types::{AppendAck, AppendInput, ReadBatch, ReadInput, SequencedRecord, StreamPosition},
};

#[frb(opaque)]
pub struct S2Stream {
    stream: RustAutoOpaqueNom<s2_sdk::S2Stream>,
}

impl S2Stream {
    pub(crate) fn new(stream: s2_sdk::S2Stream) -> S2Stream {
        S2Stream {
            stream: RustAutoOpaqueNom::new(stream),
        }
    }

    pub async fn check_tail(&self) -> Result<StreamPosition, S2Error> {
        match self.stream.try_read().unwrap().check_tail().await {
            Ok(position) => Ok(position.into()),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn append(&self, input: AppendInput) -> Result<AppendAck, S2Error> {
        match self.stream.try_read().unwrap().append(input.into()).await {
            Ok(ack) => Ok(ack.into()),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn read(&self, input: ReadInput) -> Result<ReadBatch, S2Error> {
        match self.stream.try_read().unwrap().read(input.into()).await {
            Ok(batch) => Ok(batch.into()),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn append_session(&self) -> Result<S2AppendSession, S2Error> {
        // let config = match config.try_into_config() {
        //     Ok(config) => config,
        //     Err(e) => return Err(e.into()),
        // };
        let session = self
            .stream
            .try_read()
            .unwrap()
            .append_session(s2_sdk::append_session::AppendSessionConfig::default());
        Ok(session.into())
    }

    pub async fn producer(&self) -> Result<S2Producer, S2Error> {
        let producer = self
            .stream
            .try_read()
            .unwrap()
            .producer(s2_sdk::producer::ProducerConfig::default());
        Ok(S2Producer::new(producer))
    }

    #[frb(stream_dart_await)]
    pub async fn read_session(
        &self,
        sink: StreamSink<SequencedRecord>,
        input: ReadInput,
    ) -> anyhow::Result<(), S2Error> {
        let mut session = match self
            .stream
            .try_read()
            .unwrap()
            .read_session(input.into())
            .await
        {
            Ok(session) => session,
            Err(e) => return Err(e.into()),
        };
        while let Some(batch) = session.next().await {
            match batch {
                Ok(batch) => {
                    for record in batch.records {
                        let _ = sink.add(record.into());
                    }
                }
                Err(err) => {
                    let _ = sink.add_error(anyhow::anyhow!(err.to_string()));
                }
            };
        }
        Ok(())
    }
}

impl From<s2_sdk::S2Stream> for S2Stream {
    fn from(stream: s2_sdk::S2Stream) -> Self {
        S2Stream::new(stream)
    }
}
