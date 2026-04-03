use std::{num::NonZeroU32, str::FromStr, time::Duration};

pub use s2_sdk::types::{
    AccountEndpoint, AppendAck as _AppendAck, AppendInput as _AppendInput,
    AppendRecord as _AppendRecord, AppendRecordBatch as _AppendRecordBatch,
    AppendRetryPolicy as _AppendRetryPolicy, BasinEndpoint, Compression as _Compression,
    FencingToken, Header, ReadBatch as _ReadBatch, ReadFrom as _ReadFrom, ReadInput as _ReadInput,
    ReadLimits as _ReadLimits, ReadStart as _ReadStart, ReadStop as _ReadStop,
    RetryConfig as _RetryConfig, S2Config as _S2Config, S2Endpoints,
    SequencedRecord as _SequencedRecord, StreamPosition as _StreamPosition,
};

pub struct StreamPosition {
    pub seq_num: u64,
    pub timestamp: u64,
}

impl From<_StreamPosition> for StreamPosition {
    fn from(value: _StreamPosition) -> Self {
        Self {
            seq_num: value.seq_num,
            timestamp: value.timestamp,
        }
    }
}

pub struct ClientConfig {
    pub access_token: String,
    pub endpoint: Option<String>,
    pub connection_timeout_millis: Option<u64>,
    pub request_timeout_millis: Option<u64>,
    pub compression: Option<Compression>,
    pub retry_config: Option<RetryConfig>,
}

impl Into<_S2Config> for ClientConfig {
    fn into(self) -> _S2Config {
        let mut config = _S2Config::new(self.access_token);
        if let Some(endpoint) = self.endpoint {
            config = config.with_endpoints(
                S2Endpoints::new(
                    AccountEndpoint::from_str(&endpoint).unwrap(),
                    BasinEndpoint::from_str(&endpoint).unwrap(),
                )
                .unwrap(),
            );
        }
        if let Some(timeout) = self.connection_timeout_millis {
            config = config.with_connection_timeout(Duration::from_millis(timeout));
        }
        if let Some(timeout) = self.request_timeout_millis {
            config = config.with_request_timeout(Duration::from_millis(timeout));
        }
        if let Some(compression) = self.compression {
            config = config.with_compression(compression.into());
        }
        if let Some(retry_config) = self.retry_config {
            config = config.with_retry(retry_config.into());
        }
        config
    }
}

pub enum Compression {
    None,
    Gzip,
    Zstd,
}

impl Into<_Compression> for Compression {
    fn into(self) -> _Compression {
        match self {
            Compression::None => _Compression::None,
            Compression::Gzip => _Compression::Gzip,
            Compression::Zstd => _Compression::Zstd,
        }
    }
}

pub struct RetryConfig {
    pub max_attempts: Option<u32>,
    pub min_base_delay_millis: Option<u64>,
    pub max_base_delay_millis: Option<u64>,
    pub append_retry_policy: Option<AppendRetryPolicy>,
}

impl Into<_RetryConfig> for RetryConfig {
    fn into(self) -> _RetryConfig {
        let mut config = _RetryConfig::default();
        if let Some(attempts) = self.max_attempts {
            config = config.with_max_attempts(NonZeroU32::new(attempts).unwrap());
        }
        if let Some(min_delay) = self.min_base_delay_millis {
            config = config.with_min_base_delay(Duration::from_millis(min_delay));
        }
        if let Some(max_delay) = self.max_base_delay_millis {
            config = config.with_max_base_delay(Duration::from_millis(max_delay));
        }
        if let Some(retry_policy) = self.append_retry_policy {
            config = config.with_append_retry_policy(retry_policy.into());
        }
        config
    }
}

pub enum AppendRetryPolicy {
    All,
    NoSideEffects,
}

impl Into<_AppendRetryPolicy> for AppendRetryPolicy {
    fn into(self) -> _AppendRetryPolicy {
        match self {
            AppendRetryPolicy::All => _AppendRetryPolicy::All,
            AppendRetryPolicy::NoSideEffects => _AppendRetryPolicy::NoSideEffects,
        }
    }
}

pub struct ReadInput {
    pub start: ReadStart,
    pub stop: ReadStop,
}

impl Into<_ReadInput> for ReadInput {
    fn into(self) -> _ReadInput {
        _ReadInput::new()
            .with_start(self.start.into())
            .with_stop(self.stop.into())
    }
}

pub struct ReadStart {
    pub from: ReadFrom,
    pub clamp_to_tail: bool,
}

impl Into<_ReadStart> for ReadStart {
    fn into(self) -> _ReadStart {
        let mut start = _ReadStart::new();
        start = start.with_from(self.from.into());
        start = start.with_clamp_to_tail(self.clamp_to_tail);
        start
    }
}

pub enum ReadFrom {
    SeqNum(u64),
    Timestamp(u64),
    TailOffset(u64),
}

impl Into<_ReadFrom> for ReadFrom {
    fn into(self) -> _ReadFrom {
        match self {
            ReadFrom::SeqNum(seq_num) => _ReadFrom::SeqNum(seq_num),
            ReadFrom::Timestamp(timestamp) => _ReadFrom::Timestamp(timestamp),
            ReadFrom::TailOffset(offset) => _ReadFrom::TailOffset(offset),
        }
    }
}

pub struct ReadStop {
    pub limits: ReadLimits,
    pub until_timestamp: Option<u64>,
    pub wait_secs: Option<u32>,
}

impl Into<_ReadStop> for ReadStop {
    fn into(self) -> _ReadStop {
        let mut stop = _ReadStop::new().with_limits(self.limits.into());
        if let Some(until) = self.until_timestamp {
            stop = stop.with_until(std::ops::RangeTo { end: until });
        }
        if let Some(wait) = self.wait_secs {
            stop = stop.with_wait(wait);
        }
        stop
    }
}

pub struct ReadLimits {
    pub count: Option<usize>,
    pub bytes: Option<usize>,
}

impl Into<_ReadLimits> for ReadLimits {
    fn into(self) -> _ReadLimits {
        let mut limits = _ReadLimits::new();
        if let Some(count) = self.count {
            limits = limits.with_count(count);
        }
        if let Some(bytes) = self.bytes {
            limits = limits.with_bytes(bytes);
        }
        limits
    }
}

pub struct ReadBatch {
    pub records: Vec<SequencedRecord>,
    pub tail: Option<StreamPosition>,
}

impl From<_ReadBatch> for ReadBatch {
    fn from(value: _ReadBatch) -> Self {
        Self {
            records: value.records.into_iter().map(Into::into).collect(),
            tail: value.tail.map(Into::into),
        }
    }
}

pub struct SequencedRecord {
    pub seq_num: u64,
    pub body: Vec<u8>,
    pub headers: Vec<(Vec<u8>, Vec<u8>)>,
    pub timestamp: u64,
}

impl From<_SequencedRecord> for SequencedRecord {
    fn from(value: _SequencedRecord) -> Self {
        Self {
            seq_num: value.seq_num,
            body: value.body.into(),
            headers: value
                .headers
                .into_iter()
                .map(|h| (h.name.into(), h.value.into()))
                .collect(),
            timestamp: value.timestamp,
        }
    }
}

pub struct AppendInput {
    pub records: AppendRecordBatch,
    pub match_seq_num: Option<u64>,
    pub fencing_token: Option<String>,
}

impl Into<_AppendInput> for AppendInput {
    fn into(self) -> _AppendInput {
        let mut input = _AppendInput::new(self.records.into());
        if let Some(seq_num) = self.match_seq_num {
            input = input.with_match_seq_num(seq_num);
        }
        if let Some(token) = self.fencing_token {
            input = input.with_fencing_token(FencingToken::from_str(&token).unwrap());
        }
        input
    }
}

pub struct AppendRecordBatch {
    pub records: Vec<AppendRecord>,
}

impl Into<_AppendRecordBatch> for AppendRecordBatch {
    fn into(self) -> _AppendRecordBatch {
        _AppendRecordBatch::try_from_iter(self.records.into_iter().map(|r| r.into())).unwrap()
    }
}

pub struct AppendRecord {
    pub body: Vec<u8>,
    pub headers: Vec<(Vec<u8>, Vec<u8>)>,
    pub timestamp: Option<u64>,
}

impl Into<_AppendRecord> for AppendRecord {
    fn into(self) -> _AppendRecord {
        let mut record = _AppendRecord::new(self.body)
            .unwrap()
            .with_headers(self.headers.into_iter().map(|(k, v)| Header::new(k, v)))
            .unwrap();
        if let Some(timestamp) = self.timestamp {
            record = record.with_timestamp(timestamp)
        }
        record
    }
}

pub struct AppendAck {
    pub start: StreamPosition,
    pub end: StreamPosition,
    pub tail: StreamPosition,
}

impl From<_AppendAck> for AppendAck {
    fn from(value: _AppendAck) -> Self {
        Self {
            start: value.start.into(),
            end: value.end.into(),
            tail: value.tail.into(),
        }
    }
}
