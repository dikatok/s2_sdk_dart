use std::{str::FromStr, time::Duration};

use flutter_rust_bridge::{RustAutoOpaqueNom, frb};
use tokio_stream::StreamExt;

use crate::{error::S2Error, frb_generated::StreamSink, stream::S2Stream};

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

    #[flutter_rust_bridge::frb(sync)]
    pub fn stream(&self, name: String) -> S2Stream {
        self.basin
            .try_read()
            .unwrap()
            .stream(s2_sdk::types::StreamName::from_str(name.as_str()).unwrap())
            .into()
    }

    pub async fn list_streams(&self, input: ListStreamsInput) -> Result<PageOfStreamInfo, S2Error> {
        self.basin
            .try_read()
            .unwrap()
            .list_streams(input.into())
            .await
            .map(|page| PageOfStreamInfo {
                values: page.values.into_iter().map(|info| info.into()).collect(),
                has_more: page.has_more,
            })
            .map_err(|e| e.into())
    }

    #[frb(stream_dart_await)]
    pub async fn list_all_streams(
        &self,
        sink: StreamSink<StreamInfo>,
        input: ListAllStreamsInput,
    ) -> anyhow::Result<(), S2Error> {
        let mut stream = self
            .basin
            .try_read()
            .unwrap()
            .list_all_streams(input.into());
        while let Some(info) = stream.next().await {
            match info {
                Ok(info) => {
                    let _ = sink.add(info.into());
                }
                Err(err) => {
                    let _ = sink.add_error(anyhow::anyhow!(err.to_string()));
                }
            };
        }
        Ok(())
    }

    pub async fn create_stream(&self, input: CreateStreamInput) -> Result<StreamInfo, S2Error> {
        self.basin
            .try_read()
            .unwrap()
            .create_stream(input.into())
            .await
            .map(|info| info.into())
            .map_err(|e| e.into())
    }

    pub async fn get_stream_config(&self, name: String) -> Result<StreamConfig, S2Error> {
        self.basin
            .try_read()
            .unwrap()
            .get_stream_config(s2_sdk::types::StreamName::from_str(name.as_str()).unwrap())
            .await
            .map(|config| config.into())
            .map_err(|e| e.into())
    }

    pub async fn delete_stream(&self, input: DeleteStreamInput) -> Result<(), S2Error> {
        self.basin
            .try_read()
            .unwrap()
            .delete_stream(input.into())
            .await
            .map_err(|e| e.into())
    }

    pub async fn reconfigure_stream(
        &self,
        input: ReconfigureStreamInput,
    ) -> Result<StreamConfig, S2Error> {
        self.basin
            .try_read()
            .unwrap()
            .reconfigure_stream(input.into())
            .await
            .map(|config| config.into())
            .map_err(|e| e.into())
    }
}

impl From<s2_sdk::S2Basin> for S2Basin {
    fn from(value: s2_sdk::S2Basin) -> Self {
        S2Basin::new(value)
    }
}

pub struct ListStreamsInput {
    pub prefix: Option<String>,
    pub start_after: Option<String>,
    pub limit: Option<u64>,
}

impl From<ListStreamsInput> for s2_sdk::types::ListStreamsInput {
    fn from(value: ListStreamsInput) -> Self {
        let mut input = s2_sdk::types::ListStreamsInput::default();
        if let Some(prefix) = value.prefix {
            input = input.with_prefix(s2_sdk::types::StreamNamePrefix::from_str(&prefix).unwrap());
        }
        if let Some(start_after) = value.start_after {
            input = input.with_start_after(
                s2_sdk::types::StreamNameStartAfter::from_str(&start_after).unwrap(),
            );
        }
        if let Some(limit) = value.limit {
            input = input.with_limit(limit.try_into().expect("limit too large for usize"));
        }
        input
    }
}

pub struct StreamInfo {
    pub name: String,
    pub created_at: u64,
    pub deleted_at: Option<u64>,
}

impl From<s2_sdk::types::StreamInfo> for StreamInfo {
    fn from(value: s2_sdk::types::StreamInfo) -> Self {
        StreamInfo {
            name: value.name.to_string(),
            created_at: time::OffsetDateTime::from(value.created_at).unix_timestamp() as u64,
            deleted_at: value
                .deleted_at
                .map(|t| time::OffsetDateTime::from(t).unix_timestamp() as u64),
        }
    }
}

pub struct PageOfStreamInfo {
    pub values: Vec<StreamInfo>,
    pub has_more: bool,
}

pub struct ListAllStreamsInput {
    pub prefix: Option<String>,
    pub start_after: Option<String>,
    pub include_deleted: Option<bool>,
}

impl From<ListAllStreamsInput> for s2_sdk::types::ListAllStreamsInput {
    fn from(value: ListAllStreamsInput) -> Self {
        let mut input = s2_sdk::types::ListAllStreamsInput::default();
        if let Some(prefix) = value.prefix {
            input = input.with_prefix(s2_sdk::types::StreamNamePrefix::from_str(&prefix).unwrap());
        }
        if let Some(start_after) = value.start_after {
            input = input.with_start_after(
                s2_sdk::types::StreamNameStartAfter::from_str(&start_after).unwrap(),
            );
        }
        if let Some(include_deleted) = value.include_deleted {
            input = input.with_include_deleted(include_deleted);
        }
        input
    }
}

pub struct CreateStreamInput {
    pub name: String,
    pub config: Option<StreamConfig>,
}

impl From<CreateStreamInput> for s2_sdk::types::CreateStreamInput {
    fn from(value: CreateStreamInput) -> Self {
        let mut input = s2_sdk::types::CreateStreamInput::new(
            s2_sdk::types::StreamName::from_str(value.name.as_str()).unwrap(),
        );
        if let Some(config) = value.config {
            input = input.with_config(config.into());
        }
        input
    }
}

pub struct DeleteStreamInput {
    pub name: String,
    pub ignore_not_found: Option<bool>,
}

impl From<DeleteStreamInput> for s2_sdk::types::DeleteStreamInput {
    fn from(value: DeleteStreamInput) -> Self {
        let mut input = s2_sdk::types::DeleteStreamInput::new(
            s2_sdk::types::StreamName::from_str(value.name.as_str()).unwrap(),
        );
        if let Some(ignore_not_found) = value.ignore_not_found {
            input = input.with_ignore_not_found(ignore_not_found);
        }
        input
    }
}

pub struct StreamConfig {
    pub storage_class: Option<StorageClass>,
    pub retention_policy: Option<RetentionPolicy>,
    pub timestamping: Option<TimestampingConfig>,
    pub delete_on_empty: Option<DeleteOnEmptyConfig>,
}

impl From<s2_sdk::types::StreamConfig> for StreamConfig {
    fn from(value: s2_sdk::types::StreamConfig) -> Self {
        StreamConfig {
            storage_class: value.storage_class.map(|value| value.into()),
            retention_policy: value.retention_policy.map(|value| value.into()),
            timestamping: value.timestamping.map(|value| TimestampingConfig {
                mode: value.mode.map(|mode| mode.into()),
                uncapped: if value.uncapped { Some(true) } else { None },
            }),
            delete_on_empty: value.delete_on_empty.map(|value| DeleteOnEmptyConfig {
                min_age_secs: value.min_age_secs,
            }),
        }
    }
}

impl From<StreamConfig> for s2_sdk::types::StreamConfig {
    fn from(value: StreamConfig) -> Self {
        let mut config = s2_sdk::types::StreamConfig::default();
        if let Some(storage_class) = value.storage_class {
            config = config.with_storage_class(storage_class.into());
        }
        if let Some(retention_policy) = value.retention_policy {
            config = config.with_retention_policy(retention_policy.into());
        }
        if let Some(timestamping) = value.timestamping {
            let mut timestamping_config = s2_sdk::types::TimestampingConfig::new();
            if let Some(mode) = timestamping.mode {
                timestamping_config = timestamping_config.with_mode(mode.into());
            }
            if let Some(uncapped) = timestamping.uncapped {
                timestamping_config = timestamping_config.with_uncapped(uncapped);
            }
            config = config.with_timestamping(timestamping_config);
        }
        if let Some(delete_on_empty) = value.delete_on_empty {
            config = config.with_delete_on_empty(delete_on_empty.into());
        }
        config
    }
}

pub enum StorageClass {
    Standard,
    Express,
}

impl From<StorageClass> for s2_sdk::types::StorageClass {
    fn from(value: StorageClass) -> Self {
        match value {
            StorageClass::Standard => s2_sdk::types::StorageClass::Standard,
            StorageClass::Express => s2_sdk::types::StorageClass::Express,
        }
    }
}

impl From<s2_sdk::types::StorageClass> for StorageClass {
    fn from(value: s2_sdk::types::StorageClass) -> Self {
        match value {
            s2_sdk::types::StorageClass::Standard => StorageClass::Standard,
            s2_sdk::types::StorageClass::Express => StorageClass::Express,
        }
    }
}

pub enum RetentionPolicy {
    Infinite,
    Age(u64),
}

impl From<RetentionPolicy> for s2_sdk::types::RetentionPolicy {
    fn from(value: RetentionPolicy) -> Self {
        match value {
            RetentionPolicy::Infinite => s2_sdk::types::RetentionPolicy::Infinite,
            RetentionPolicy::Age(seconds) => s2_sdk::types::RetentionPolicy::Age(seconds),
        }
    }
}

impl From<s2_sdk::types::RetentionPolicy> for RetentionPolicy {
    fn from(value: s2_sdk::types::RetentionPolicy) -> Self {
        match value {
            s2_sdk::types::RetentionPolicy::Infinite => RetentionPolicy::Infinite,
            s2_sdk::types::RetentionPolicy::Age(duration) => RetentionPolicy::Age(duration),
        }
    }
}

pub struct TimestampingConfig {
    pub mode: Option<TimestampingMode>,
    pub uncapped: Option<bool>,
}

pub enum TimestampingMode {
    ClientPrefer,
    ClientRequire,
    Arrival,
}

impl From<TimestampingMode> for s2_sdk::types::TimestampingMode {
    fn from(value: TimestampingMode) -> Self {
        match value {
            TimestampingMode::ClientPrefer => s2_sdk::types::TimestampingMode::ClientPrefer,
            TimestampingMode::ClientRequire => s2_sdk::types::TimestampingMode::ClientRequire,
            TimestampingMode::Arrival => s2_sdk::types::TimestampingMode::Arrival,
        }
    }
}

impl From<s2_sdk::types::TimestampingMode> for TimestampingMode {
    fn from(value: s2_sdk::types::TimestampingMode) -> Self {
        match value {
            s2_sdk::types::TimestampingMode::ClientPrefer => TimestampingMode::ClientPrefer,
            s2_sdk::types::TimestampingMode::ClientRequire => TimestampingMode::ClientRequire,
            s2_sdk::types::TimestampingMode::Arrival => TimestampingMode::Arrival,
        }
    }
}

pub struct DeleteOnEmptyConfig {
    pub min_age_secs: u64,
}

impl From<DeleteOnEmptyConfig> for s2_sdk::types::DeleteOnEmptyConfig {
    fn from(value: DeleteOnEmptyConfig) -> Self {
        s2_sdk::types::DeleteOnEmptyConfig::new()
            .with_min_age(std::time::Duration::from_secs(value.min_age_secs))
    }
}

impl From<s2_sdk::types::DeleteOnEmptyConfig> for DeleteOnEmptyConfig {
    fn from(value: s2_sdk::types::DeleteOnEmptyConfig) -> Self {
        DeleteOnEmptyConfig {
            min_age_secs: value.min_age_secs,
        }
    }
}

pub struct ReconfigureStreamInput {
    pub name: String,
    pub config: StreamConfig,
}

impl From<ReconfigureStreamInput> for s2_sdk::types::ReconfigureStreamInput {
    fn from(value: ReconfigureStreamInput) -> Self {
        s2_sdk::types::ReconfigureStreamInput::new(
            s2_sdk::types::StreamName::from_str(value.name.as_str()).unwrap(),
            value.config.into(),
        )
    }
}

impl From<StreamConfig> for s2_sdk::types::StreamReconfiguration {
    fn from(value: StreamConfig) -> Self {
        let mut config = s2_sdk::types::StreamReconfiguration::new();
        if let Some(storage_class) = value.storage_class {
            config = config.with_storage_class(storage_class.into());
        }
        if let Some(retention_policy) = value.retention_policy {
            config = config.with_retention_policy(retention_policy.into());
        }
        if let Some(timestamping) = value.timestamping {
            let mut timestamping_reconfig = s2_sdk::types::TimestampingReconfiguration::new();
            if let Some(mode) = timestamping.mode {
                timestamping_reconfig = timestamping_reconfig.with_mode(mode.into());
            }
            if let Some(uncapped) = timestamping.uncapped {
                timestamping_reconfig = timestamping_reconfig.with_uncapped(uncapped);
            }
            config = config.with_timestamping(timestamping_reconfig);
        }
        if let Some(delete_on_empty) = value.delete_on_empty {
            let delete_on_empty_reconfig = s2_sdk::types::DeleteOnEmptyReconfiguration::new()
                .with_min_age(Duration::from_secs(delete_on_empty.min_age_secs));
            config = config.with_delete_on_empty(delete_on_empty_reconfig);
        }
        config
    }
}
