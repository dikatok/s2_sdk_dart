use std::{num::NonZeroU32, str::FromStr, time::Duration};

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

    #[flutter_rust_bridge::frb(sync)]
    pub fn append_session(&self, config: AppendSessionConfig) -> Result<S2AppendSession, S2Error> {
        let session =
            self.stream
                .try_read()
                .unwrap()
                .append_session(match config.try_into_config() {
                    Ok(config) => config,
                    Err(e) => return Err(e.into()),
                });
        Ok(session.into())
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn producer(&self, config: ProducerConfig) -> Result<S2Producer, S2Error> {
        let producer = self.stream.try_read().unwrap().producer(config.try_into()?);
        Ok(producer.into())
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
    fn from(value: s2_sdk::S2Stream) -> Self {
        S2Stream::new(value)
    }
}

pub struct AppendSessionConfig {
    pub max_unacked_bytes: Option<u32>,
    pub max_unacked_batches: Option<u32>,
}

impl AppendSessionConfig {
    pub(crate) fn try_into_config(
        self,
    ) -> Result<s2_sdk::append_session::AppendSessionConfig, S2Error> {
        let mut config = s2_sdk::append_session::AppendSessionConfig::default();
        if let Some(bytes) = self.max_unacked_bytes {
            config = match config.with_max_unacked_bytes(bytes) {
                Ok(config) => config,
                Err(e) => {
                    return Err(S2Error::from_str(e.to_string().as_str()).unwrap());
                }
            }
        }
        if let Some(batches) = self.max_unacked_batches
            && batches > 0
        {
            config = config.with_max_unacked_batches(NonZeroU32::new(batches).unwrap());
        }
        Ok(config)
    }
}

pub struct ProducerConfig {
    pub max_unacked_bytes: Option<u32>,
    pub batching: Option<BatchingConfig>,
    pub fencing_token: Option<String>,
    pub match_seq_num: Option<u64>,
}

impl TryFrom<ProducerConfig> for s2_sdk::producer::ProducerConfig {
    type Error = S2Error;

    fn try_from(value: ProducerConfig) -> Result<Self, S2Error> {
        let mut config = s2_sdk::producer::ProducerConfig::default();
        if let Some(bytes) = value.max_unacked_bytes {
            config = match config.with_max_unacked_bytes(bytes) {
                Ok(config) => config,
                Err(e) => return Err(e.into()),
            };
        }
        if let Some(batching) = value.batching {
            config = config.with_batching(match batching.try_into() {
                Ok(batching) => batching,
                Err(e) => return Err(e.into()),
            });
        }
        if let Some(fencing_token) = value.fencing_token {
            config = config.with_fencing_token(
                match s2_sdk::types::FencingToken::from_str(&fencing_token) {
                    Ok(token) => token,
                    Err(e) => return Err(e.into()),
                },
            );
        }
        if let Some(match_seq_num) = value.match_seq_num {
            config = config.with_match_seq_num(match_seq_num);
        }
        Ok(config)
    }
}

pub struct BatchingConfig {
    pub linger_millis: Option<u64>,
    pub max_batch_bytes: Option<u64>,
    pub max_batch_records: Option<u64>,
}

impl TryFrom<BatchingConfig> for s2_sdk::batching::BatchingConfig {
    type Error = S2Error;

    fn try_from(value: BatchingConfig) -> Result<Self, S2Error> {
        let mut config = s2_sdk::batching::BatchingConfig::new();
        if let Some(linger_millis) = value.linger_millis {
            config = config.with_linger(Duration::from_millis(linger_millis));
        }
        if let Some(max_batch_bytes) = value.max_batch_bytes {
            config = match config.with_max_batch_bytes(match max_batch_bytes.try_into() {
                Ok(max_batch_bytes) => max_batch_bytes,
                Err(_) => {
                    return Err(S2Error::from_str("max_batch_bytes too large for usize").unwrap());
                }
            }) {
                Ok(config) => config,
                Err(e) => return Err(e.into()),
            };
        }
        if let Some(max_batch_records) = value.max_batch_records {
            config = match config.with_max_batch_records(match max_batch_records.try_into() {
                Ok(max_batch_records) => max_batch_records,
                Err(_) => {
                    return Err(S2Error::from_str("max_batch_records too large for usize").unwrap());
                }
            }) {
                Ok(config) => config,
                Err(e) => return Err(e.into()),
            };
        }
        Ok(config)
    }
}
