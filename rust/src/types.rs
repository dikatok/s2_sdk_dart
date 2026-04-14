use std::{num::NonZeroU32, str::FromStr, time::Duration};

use crate::error::S2Error;

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

impl TryFrom<ClientConfig> for s2_sdk::types::S2Config {
    type Error = S2Error;

    fn try_from(value: ClientConfig) -> Result<Self, Self::Error> {
        let mut config = s2_sdk::types::S2Config::new(value.access_token);
        if let Some(endpoint) = value.endpoint {
            config = config.with_endpoints(s2_sdk::types::S2Endpoints::new(
                s2_sdk::types::AccountEndpoint::from_str(&endpoint)?,
                s2_sdk::types::BasinEndpoint::from_str(&endpoint)?,
            )?);
        }
        if let Some(timeout) = value.connection_timeout_millis {
            config = config.with_connection_timeout(Duration::from_millis(timeout));
        }
        if let Some(timeout) = value.request_timeout_millis {
            config = config.with_request_timeout(Duration::from_millis(timeout));
        }
        if let Some(compression) = value.compression {
            config = config.with_compression(compression.into());
        }
        if let Some(retry_config) = value.retry_config {
            config = config.with_retry(retry_config.into());
        }
        Ok(config)
    }
}

pub enum Compression {
    None,
    Gzip,
    Zstd,
}

impl From<Compression> for s2_sdk::types::Compression {
    fn from(value: Compression) -> Self {
        match value {
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
    fn from(value: RetryConfig) -> Self {
        let mut config = s2_sdk::types::RetryConfig::default();
        if let Some(attempts) = value.max_attempts {
            config = match NonZeroU32::new(attempts) {
                Some(attempts) => config.with_max_attempts(attempts),
                _ => config,
            };
        }
        if let Some(min_delay) = value.min_base_delay_millis {
            config = config.with_min_base_delay(Duration::from_millis(min_delay));
        }
        if let Some(max_delay) = value.max_base_delay_millis {
            config = config.with_max_base_delay(Duration::from_millis(max_delay));
        }
        if let Some(retry_policy) = value.append_retry_policy {
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
    fn from(value: AppendRetryPolicy) -> Self {
        match value {
            AppendRetryPolicy::All => s2_sdk::types::AppendRetryPolicy::All,
            AppendRetryPolicy::NoSideEffects => s2_sdk::types::AppendRetryPolicy::NoSideEffects,
        }
    }
}

pub struct ReadInput {
    pub start: Option<ReadStart>,
    pub stop: Option<ReadStop>,
}

impl TryFrom<ReadInput> for s2_sdk::types::ReadInput {
    type Error = S2Error;

    fn try_from(value: ReadInput) -> Result<Self, Self::Error> {
        let mut input = s2_sdk::types::ReadInput::new();
        if let Some(start) = value.start {
            input = input.with_start(start.into());
        }
        if let Some(stop) = value.stop {
            input = input.with_stop(stop.try_into()?);
        }
        Ok(input)
    }
}

pub struct ReadStart {
    pub from: Option<ReadFrom>,
    pub clamp_to_tail: Option<bool>,
}

impl From<ReadStart> for s2_sdk::types::ReadStart {
    fn from(value: ReadStart) -> Self {
        let mut start = s2_sdk::types::ReadStart::new();
        if let Some(from) = value.from {
            start = start.with_from(from.into());
        }
        if let Some(clamp) = value.clamp_to_tail {
            start = start.with_clamp_to_tail(clamp);
        }
        start
    }
}

pub enum ReadFrom {
    SeqNum(u64),
    Timestamp(u64),
    TailOffset(u64),
}

impl From<ReadFrom> for s2_sdk::types::ReadFrom {
    fn from(value: ReadFrom) -> Self {
        match value {
            ReadFrom::SeqNum(seq_num) => s2_sdk::types::ReadFrom::SeqNum(seq_num),
            ReadFrom::Timestamp(timestamp) => s2_sdk::types::ReadFrom::Timestamp(timestamp),
            ReadFrom::TailOffset(offset) => s2_sdk::types::ReadFrom::TailOffset(offset),
        }
    }
}

pub struct ReadStop {
    pub limits: Option<ReadLimits>,
    pub until_timestamp: Option<u64>,
    pub wait_secs: Option<u32>,
}

impl TryFrom<ReadStop> for s2_sdk::types::ReadStop {
    type Error = S2Error;

    fn try_from(value: ReadStop) -> Result<Self, Self::Error> {
        let mut stop = s2_sdk::types::ReadStop::new();
        if let Some(limits) = value.limits {
            stop = stop.with_limits(limits.try_into()?);
        }
        if let Some(until) = value.until_timestamp {
            stop = stop.with_until(std::ops::RangeTo { end: until });
        }
        if let Some(wait) = value.wait_secs {
            stop = stop.with_wait(wait);
        }
        Ok(stop)
    }
}

pub struct ReadLimits {
    pub count: Option<u64>,
    pub bytes: Option<u64>,
}

impl TryFrom<ReadLimits> for s2_sdk::types::ReadLimits {
    type Error = S2Error;

    fn try_from(value: ReadLimits) -> Result<Self, Self::Error> {
        let mut limits = s2_sdk::types::ReadLimits::new();
        if let Some(count) = value.count {
            limits = limits.with_count(count.try_into()?);
        }
        if let Some(bytes) = value.bytes {
            limits = limits.with_bytes(bytes.try_into()?);
        }
        Ok(limits)
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

impl TryFrom<AppendInput> for s2_sdk::types::AppendInput {
    type Error = S2Error;

    fn try_from(value: AppendInput) -> Result<Self, Self::Error> {
        let mut input = s2_sdk::types::AppendInput::new(value.records.try_into()?);
        if let Some(seq_num) = value.match_seq_num {
            input = input.with_match_seq_num(seq_num);
        }
        if let Some(token) = value.fencing_token {
            input = input.with_fencing_token(s2_sdk::types::FencingToken::from_str(&token)?);
        }
        Ok(input)
    }
}

pub struct AppendRecordBatch {
    pub records: Vec<AppendRecord>,
}

impl TryFrom<AppendRecordBatch> for s2_sdk::types::AppendRecordBatch {
    type Error = S2Error;

    fn try_from(value: AppendRecordBatch) -> Result<Self, Self::Error> {
        let records: Vec<s2_sdk::types::AppendRecord> = value
            .records
            .into_iter()
            .map(s2_sdk::types::AppendRecord::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(s2_sdk::types::AppendRecordBatch::try_from_iter(records)?)
    }
}

pub struct AppendRecord {
    pub body: Vec<u8>,
    pub headers: Vec<(Vec<u8>, Vec<u8>)>,
    pub timestamp: Option<u64>,
}

impl TryFrom<AppendRecord> for s2_sdk::types::AppendRecord {
    type Error = S2Error;

    fn try_from(value: AppendRecord) -> Result<Self, Self::Error> {
        let mut record = s2_sdk::types::AppendRecord::new(value.body)?.with_headers(
            value
                .headers
                .into_iter()
                .map(|(k, v)| s2_sdk::types::Header::new(k, v)),
        )?;
        if let Some(timestamp) = value.timestamp {
            record = record.with_timestamp(timestamp)
        }
        Ok(record)
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

pub enum ResourceSet {
    None,
    Exact(String),
    Prefix(String),
}

impl From<s2_sdk::types::BasinMatcher> for ResourceSet {
    fn from(value: s2_sdk::types::BasinMatcher) -> Self {
        match value {
            s2_sdk::types::BasinMatcher::Exact(name) => ResourceSet::Exact(name.to_string()),
            s2_sdk::types::BasinMatcher::Prefix(prefix) => ResourceSet::Prefix(prefix.to_string()),
            s2_sdk::types::BasinMatcher::None => ResourceSet::None,
        }
    }
}

impl TryFrom<ResourceSet> for s2_sdk::types::BasinMatcher {
    type Error = S2Error;

    fn try_from(value: ResourceSet) -> Result<Self, Self::Error> {
        Ok(match value {
            ResourceSet::Exact(name) => {
                s2_sdk::types::BasinMatcher::Exact(s2_sdk::types::BasinName::from_str(&name)?)
            }
            ResourceSet::Prefix(prefix) => s2_sdk::types::BasinMatcher::Prefix(
                s2_sdk::types::BasinNamePrefix::from_str(&prefix)?,
            ),
            ResourceSet::None => s2_sdk::types::BasinMatcher::None,
        })
    }
}

impl From<s2_sdk::types::StreamMatcher> for ResourceSet {
    fn from(value: s2_sdk::types::StreamMatcher) -> Self {
        match value {
            s2_sdk::types::StreamMatcher::Exact(name) => ResourceSet::Exact(name.to_string()),
            s2_sdk::types::StreamMatcher::Prefix(prefix) => ResourceSet::Prefix(prefix.to_string()),
            s2_sdk::types::StreamMatcher::None => ResourceSet::None,
        }
    }
}

impl TryFrom<ResourceSet> for s2_sdk::types::StreamMatcher {
    type Error = S2Error;

    fn try_from(value: ResourceSet) -> Result<Self, Self::Error> {
        Ok(match value {
            ResourceSet::Exact(name) => {
                s2_sdk::types::StreamMatcher::Exact(s2_sdk::types::StreamName::from_str(&name)?)
            }
            ResourceSet::Prefix(prefix) => s2_sdk::types::StreamMatcher::Prefix(
                s2_sdk::types::StreamNamePrefix::from_str(&prefix)?,
            ),
            ResourceSet::None => s2_sdk::types::StreamMatcher::None,
        })
    }
}

impl From<s2_sdk::types::AccessTokenMatcher> for ResourceSet {
    fn from(value: s2_sdk::types::AccessTokenMatcher) -> Self {
        match value {
            s2_sdk::types::AccessTokenMatcher::Exact(id) => ResourceSet::Exact(id.to_string()),
            s2_sdk::types::AccessTokenMatcher::Prefix(prefix) => {
                ResourceSet::Prefix(prefix.to_string())
            }
            s2_sdk::types::AccessTokenMatcher::None => ResourceSet::None,
        }
    }
}

impl TryFrom<ResourceSet> for s2_sdk::types::AccessTokenMatcher {
    type Error = S2Error;

    fn try_from(value: ResourceSet) -> Result<Self, Self::Error> {
        Ok(match value {
            ResourceSet::Exact(id) => s2_sdk::types::AccessTokenMatcher::Exact(
                s2_sdk::types::AccessTokenId::from_str(&id)?,
            ),
            ResourceSet::Prefix(prefix) => s2_sdk::types::AccessTokenMatcher::Prefix(
                s2_sdk::types::AccessTokenIdPrefix::from_str(&prefix)?,
            ),
            ResourceSet::None => s2_sdk::types::AccessTokenMatcher::None,
        })
    }
}

pub enum Operation {
    ListBasins,
    CreateBasin,
    GetBasinConfig,
    DeleteBasin,
    ReconfigureBasin,
    ListAccessTokens,
    IssueAccessToken,
    RevokeAccessToken,
    GetAccountMetrics,
    GetBasinMetrics,
    GetStreamMetrics,
    ListStreams,
    CreateStream,
    GetStreamConfig,
    DeleteStream,
    ReconfigureStream,
    CheckTail,
    Append,
    Read,
    Trim,
    Fence,
}

impl From<s2_sdk::types::Operation> for Operation {
    fn from(value: s2_sdk::types::Operation) -> Self {
        match value {
            s2_sdk::types::Operation::ListBasins => Operation::ListBasins,
            s2_sdk::types::Operation::CreateBasin => Operation::CreateBasin,
            s2_sdk::types::Operation::GetBasinConfig => Operation::GetBasinConfig,
            s2_sdk::types::Operation::DeleteBasin => Operation::DeleteBasin,
            s2_sdk::types::Operation::ReconfigureBasin => Operation::ReconfigureBasin,
            s2_sdk::types::Operation::ListAccessTokens => Operation::ListAccessTokens,
            s2_sdk::types::Operation::IssueAccessToken => Operation::IssueAccessToken,
            s2_sdk::types::Operation::RevokeAccessToken => Operation::RevokeAccessToken,
            s2_sdk::types::Operation::GetAccountMetrics => Operation::GetAccountMetrics,
            s2_sdk::types::Operation::GetBasinMetrics => Operation::GetBasinMetrics,
            s2_sdk::types::Operation::GetStreamMetrics => Operation::GetStreamMetrics,
            s2_sdk::types::Operation::ListStreams => Operation::ListStreams,
            s2_sdk::types::Operation::CreateStream => Operation::CreateStream,
            s2_sdk::types::Operation::GetStreamConfig => Operation::GetStreamConfig,
            s2_sdk::types::Operation::DeleteStream => Operation::DeleteStream,
            s2_sdk::types::Operation::ReconfigureStream => Operation::ReconfigureStream,
            s2_sdk::types::Operation::CheckTail => Operation::CheckTail,
            s2_sdk::types::Operation::Append => Operation::Append,
            s2_sdk::types::Operation::Read => Operation::Read,
            s2_sdk::types::Operation::Trim => Operation::Trim,
            s2_sdk::types::Operation::Fence => Operation::Fence,
        }
    }
}

impl From<Operation> for s2_sdk::types::Operation {
    fn from(value: Operation) -> Self {
        match value {
            Operation::ListBasins => s2_sdk::types::Operation::ListBasins,
            Operation::CreateBasin => s2_sdk::types::Operation::CreateBasin,
            Operation::GetBasinConfig => s2_sdk::types::Operation::GetBasinConfig,
            Operation::DeleteBasin => s2_sdk::types::Operation::DeleteBasin,
            Operation::ReconfigureBasin => s2_sdk::types::Operation::ReconfigureBasin,
            Operation::ListAccessTokens => s2_sdk::types::Operation::ListAccessTokens,
            Operation::IssueAccessToken => s2_sdk::types::Operation::IssueAccessToken,
            Operation::RevokeAccessToken => s2_sdk::types::Operation::RevokeAccessToken,
            Operation::GetAccountMetrics => s2_sdk::types::Operation::GetAccountMetrics,
            Operation::GetBasinMetrics => s2_sdk::types::Operation::GetBasinMetrics,
            Operation::GetStreamMetrics => s2_sdk::types::Operation::GetStreamMetrics,
            Operation::ListStreams => s2_sdk::types::Operation::ListStreams,
            Operation::CreateStream => s2_sdk::types::Operation::CreateStream,
            Operation::GetStreamConfig => s2_sdk::types::Operation::GetStreamConfig,
            Operation::DeleteStream => s2_sdk::types::Operation::DeleteStream,
            Operation::ReconfigureStream => s2_sdk::types::Operation::ReconfigureStream,
            Operation::CheckTail => s2_sdk::types::Operation::CheckTail,
            Operation::Append => s2_sdk::types::Operation::Append,
            Operation::Read => s2_sdk::types::Operation::Read,
            Operation::Trim => s2_sdk::types::Operation::Trim,
            Operation::Fence => s2_sdk::types::Operation::Fence,
        }
    }
}
