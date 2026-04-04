use std::{num::NonZeroU32, str::FromStr, time::Duration};

pub struct StreamPosition {
    pub seq_num: u64,
    pub timestamp: u64,
}

impl From<s2_sdk::types::StreamPosition> for StreamPosition {
    fn from(value: s2_sdk::types::StreamPosition) -> Self {
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

impl From<ClientConfig> for s2_sdk::types::S2Config {
    fn from(val: ClientConfig) -> Self {
        let mut config = s2_sdk::types::S2Config::new(val.access_token);
        if let Some(endpoint) = val.endpoint {
            config = config.with_endpoints(
                s2_sdk::types::S2Endpoints::new(
                    s2_sdk::types::AccountEndpoint::from_str(&endpoint).unwrap(),
                    s2_sdk::types::BasinEndpoint::from_str(&endpoint).unwrap(),
                )
                .unwrap(),
            );
        }
        if let Some(timeout) = val.connection_timeout_millis {
            config = config.with_connection_timeout(Duration::from_millis(timeout));
        }
        if let Some(timeout) = val.request_timeout_millis {
            config = config.with_request_timeout(Duration::from_millis(timeout));
        }
        if let Some(compression) = val.compression {
            config = config.with_compression(compression.into());
        }
        if let Some(retry_config) = val.retry_config {
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

impl From<Compression> for s2_sdk::types::Compression {
    fn from(val: Compression) -> Self {
        match val {
            Compression::None => s2_sdk::types::Compression::None,
            Compression::Gzip => s2_sdk::types::Compression::Gzip,
            Compression::Zstd => s2_sdk::types::Compression::Zstd,
        }
    }
}

pub struct RetryConfig {
    pub max_attempts: Option<u32>,
    pub min_base_delay_millis: Option<u64>,
    pub max_base_delay_millis: Option<u64>,
    pub append_retry_policy: Option<AppendRetryPolicy>,
}

impl From<RetryConfig> for s2_sdk::types::RetryConfig {
    fn from(val: RetryConfig) -> Self {
        let mut config = s2_sdk::types::RetryConfig::default();
        if let Some(attempts) = val.max_attempts {
            config = config.with_max_attempts(NonZeroU32::new(attempts).unwrap());
        }
        if let Some(min_delay) = val.min_base_delay_millis {
            config = config.with_min_base_delay(Duration::from_millis(min_delay));
        }
        if let Some(max_delay) = val.max_base_delay_millis {
            config = config.with_max_base_delay(Duration::from_millis(max_delay));
        }
        if let Some(retry_policy) = val.append_retry_policy {
            config = config.with_append_retry_policy(retry_policy.into());
        }
        config
    }
}

pub enum AppendRetryPolicy {
    All,
    NoSideEffects,
}

impl From<AppendRetryPolicy> for s2_sdk::types::AppendRetryPolicy {
    fn from(val: AppendRetryPolicy) -> Self {
        match val {
            AppendRetryPolicy::All => s2_sdk::types::AppendRetryPolicy::All,
            AppendRetryPolicy::NoSideEffects => s2_sdk::types::AppendRetryPolicy::NoSideEffects,
        }
    }
}

pub struct ReadInput {
    pub start: ReadStart,
    pub stop: ReadStop,
}

impl From<ReadInput> for s2_sdk::types::ReadInput {
    fn from(val: ReadInput) -> Self {
        s2_sdk::types::ReadInput::new()
            .with_start(val.start.into())
            .with_stop(val.stop.into())
    }
}

pub struct ReadStart {
    pub from: ReadFrom,
    pub clamp_to_tail: bool,
}

impl From<ReadStart> for s2_sdk::types::ReadStart {
    fn from(val: ReadStart) -> Self {
        let mut start = s2_sdk::types::ReadStart::new();
        start = start.with_from(val.from.into());
        start = start.with_clamp_to_tail(val.clamp_to_tail);
        start
    }
}

pub enum ReadFrom {
    SeqNum(u64),
    Timestamp(u64),
    TailOffset(u64),
}

impl From<ReadFrom> for s2_sdk::types::ReadFrom {
    fn from(val: ReadFrom) -> Self {
        match val {
            ReadFrom::SeqNum(seq_num) => s2_sdk::types::ReadFrom::SeqNum(seq_num),
            ReadFrom::Timestamp(timestamp) => s2_sdk::types::ReadFrom::Timestamp(timestamp),
            ReadFrom::TailOffset(offset) => s2_sdk::types::ReadFrom::TailOffset(offset),
        }
    }
}

pub struct ReadStop {
    pub limits: ReadLimits,
    pub until_timestamp: Option<u64>,
    pub wait_secs: Option<u32>,
}

impl From<ReadStop> for s2_sdk::types::ReadStop {
    fn from(val: ReadStop) -> Self {
        let mut stop = s2_sdk::types::ReadStop::new().with_limits(val.limits.into());
        if let Some(until) = val.until_timestamp {
            stop = stop.with_until(std::ops::RangeTo { end: until });
        }
        if let Some(wait) = val.wait_secs {
            stop = stop.with_wait(wait);
        }
        stop
    }
}

pub struct ReadLimits {
    pub count: Option<usize>,
    pub bytes: Option<usize>,
}

impl From<ReadLimits> for s2_sdk::types::ReadLimits {
    fn from(val: ReadLimits) -> Self {
        let mut limits = s2_sdk::types::ReadLimits::new();
        if let Some(count) = val.count {
            limits = limits.with_count(count);
        }
        if let Some(bytes) = val.bytes {
            limits = limits.with_bytes(bytes);
        }
        limits
    }
}

pub struct ReadBatch {
    pub records: Vec<SequencedRecord>,
    pub tail: Option<StreamPosition>,
}

impl From<s2_sdk::types::ReadBatch> for ReadBatch {
    fn from(value: s2_sdk::types::ReadBatch) -> Self {
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

impl From<s2_sdk::types::SequencedRecord> for SequencedRecord {
    fn from(value: s2_sdk::types::SequencedRecord) -> Self {
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

impl From<AppendInput> for s2_sdk::types::AppendInput {
    fn from(val: AppendInput) -> Self {
        let mut input = s2_sdk::types::AppendInput::new(val.records.into());
        if let Some(seq_num) = val.match_seq_num {
            input = input.with_match_seq_num(seq_num);
        }
        if let Some(token) = val.fencing_token {
            input =
                input.with_fencing_token(s2_sdk::types::FencingToken::from_str(&token).unwrap());
        }
        input
    }
}

pub struct AppendRecordBatch {
    pub records: Vec<AppendRecord>,
}

impl From<AppendRecordBatch> for s2_sdk::types::AppendRecordBatch {
    fn from(val: AppendRecordBatch) -> Self {
        s2_sdk::types::AppendRecordBatch::try_from_iter(val.records.into_iter().map(|r| r.into()))
            .unwrap()
    }
}

pub struct AppendRecord {
    pub body: Vec<u8>,
    pub headers: Vec<(Vec<u8>, Vec<u8>)>,
    pub timestamp: Option<u64>,
}

impl From<AppendRecord> for s2_sdk::types::AppendRecord {
    fn from(val: AppendRecord) -> Self {
        let mut record = s2_sdk::types::AppendRecord::new(val.body)
            .unwrap()
            .with_headers(
                val.headers
                    .into_iter()
                    .map(|(k, v)| s2_sdk::types::Header::new(k, v)),
            )
            .unwrap();
        if let Some(timestamp) = val.timestamp {
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

impl From<s2_sdk::types::AppendAck> for AppendAck {
    fn from(value: s2_sdk::types::AppendAck) -> Self {
        Self {
            start: value.start.into(),
            end: value.end.into(),
            tail: value.tail.into(),
        }
    }
}
