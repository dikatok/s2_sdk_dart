use std::{str::FromStr, time::Duration};

use flutter_rust_bridge::{RustAutoOpaqueNom, frb};
use time::OffsetDateTime;
use tokio_stream::StreamExt;

use crate::{
    basin::{S2Basin, StreamConfig},
    error::S2Error,
    frb_generated::StreamSink,
    types::{ClientConfig, Operation, ResourceSet},
};

#[frb(opaque)]
pub struct S2Client {
    client: RustAutoOpaqueNom<s2_sdk::S2>,
}

impl S2Client {
    #[frb(sync)]
    pub fn new(config: ClientConfig) -> S2Client {
        S2Client {
            client: RustAutoOpaqueNom::new(s2_sdk::S2::new(config.into()).unwrap()),
        }
    }

    #[frb(sync)]
    pub fn basin(&self, name: String) -> S2Basin {
        self.client
            .try_read()
            .unwrap()
            .basin(s2_sdk::types::BasinName::from_str(name.as_str()).unwrap())
            .into()
    }

    pub async fn list_basins(&self, input: ListBasinsInput) -> Result<PageOfBasinInfo, S2Error> {
        self.client
            .try_read()
            .unwrap()
            .list_basins(input.into())
            .await
            .map(|page| PageOfBasinInfo {
                values: page.values.into_iter().map(BasinInfo::from).collect(),
                has_more: page.has_more,
            })
            .map_err(|e| e.into())
    }

    pub async fn list_all_basins(
        &self,
        sink: StreamSink<BasinInfo>,
        input: ListAllBasinsInput,
    ) -> anyhow::Result<(), S2Error> {
        let mut stream = self
            .client
            .try_read()
            .unwrap()
            .list_all_basins(input.into());
        while let Some(basin) = stream.next().await {
            match basin {
                Ok(basin) => {
                    let _ = sink.add(basin.into());
                }
                Err(err) => {
                    let _ = sink.add_error(anyhow::anyhow!(err.to_string()));
                }
            };
        }
        Ok(())
    }

    pub async fn create_basin(&self, input: CreateBasinInput) -> Result<BasinInfo, S2Error> {
        self.client
            .try_read()
            .unwrap()
            .create_basin(input.into())
            .await
            .map(|info| info.into())
            .map_err(|e| e.into())
    }

    pub async fn get_basin_config(&self, name: String) -> Result<BasinConfig, S2Error> {
        self.client
            .try_read()
            .unwrap()
            .get_basin_config(s2_sdk::types::BasinName::from_str(name.as_str()).unwrap())
            .await
            .map(|config| config.into())
            .map_err(|e| e.into())
    }

    pub async fn delete_basin(&self, input: DeleteBasinInput) -> Result<(), S2Error> {
        self.client
            .try_read()
            .unwrap()
            .delete_basin(input.into())
            .await
            .map_err(|e| e.into())
    }

    pub async fn reconfigure_basin(
        &self,
        input: ReconfigureBasinInput,
    ) -> Result<BasinConfig, S2Error> {
        self.client
            .try_read()
            .unwrap()
            .reconfigure_basin(input.into())
            .await
            .map(|config| config.into())
            .map_err(|e| e.into())
    }

    pub async fn list_access_tokens(
        &self,
        input: ListAccessTokensInput,
    ) -> Result<PageOfAccessTokenInfo, S2Error> {
        self.client
            .try_read()
            .unwrap()
            .list_access_tokens(input.into())
            .await
            .map(|page| PageOfAccessTokenInfo {
                values: page.values.into_iter().map(AccessTokenInfo::from).collect(),
                has_more: page.has_more,
            })
            .map_err(|e| e.into())
    }

    pub async fn list_all_access_tokens(
        &self,
        sink: StreamSink<AccessTokenInfo>,
        input: ListAllAccessTokensInput,
    ) -> anyhow::Result<(), S2Error> {
        let mut stream = self
            .client
            .try_read()
            .unwrap()
            .list_all_access_tokens(input.into());
        while let Some(token) = stream.next().await {
            match token {
                Ok(token) => {
                    let _ = sink.add(token.into());
                }
                Err(err) => {
                    let _ = sink.add_error(anyhow::anyhow!(err.to_string()));
                }
            };
        }
        Ok(())
    }

    pub async fn issue_access_token(
        &self,
        input: IssueAccessTokenInput,
    ) -> Result<String, S2Error> {
        self.client
            .try_read()
            .unwrap()
            .issue_access_token(input.into())
            .await
            .map(|s| s.to_string())
            .map_err(|e| e.into())
    }

    pub async fn revoke_access_token(&self, id: String) -> Result<(), S2Error> {
        self.client
            .try_read()
            .unwrap()
            .revoke_access_token(s2_sdk::types::AccessTokenId::from_str(&id).unwrap())
            .await
            .map_err(|e| e.into())
    }
}

pub struct BasinInfo {
    pub name: String,
    pub scope: Option<BasinScope>,
    pub created_at: u64,
    pub deleted_at: Option<u64>,
}

impl From<s2_sdk::types::BasinInfo> for BasinInfo {
    fn from(info: s2_sdk::types::BasinInfo) -> Self {
        BasinInfo {
            name: info.name.to_string(),
            scope: info.scope.map(|s| s.into()),
            created_at: OffsetDateTime::from(info.created_at).unix_timestamp() as u64,
            deleted_at: info
                .deleted_at
                .map(|dt| OffsetDateTime::from(dt).unix_timestamp() as u64),
        }
    }
}

pub enum BasinScope {
    AwsUsEast1,
}

impl From<s2_sdk::types::BasinScope> for BasinScope {
    fn from(scope: s2_sdk::types::BasinScope) -> Self {
        match scope {
            s2_sdk::types::BasinScope::AwsUsEast1 => BasinScope::AwsUsEast1,
        }
    }
}

impl From<BasinScope> for s2_sdk::types::BasinScope {
    fn from(scope: BasinScope) -> Self {
        match scope {
            BasinScope::AwsUsEast1 => s2_sdk::types::BasinScope::AwsUsEast1,
        }
    }
}

pub struct PageOfBasinInfo {
    pub values: Vec<BasinInfo>,
    pub has_more: bool,
}

pub struct ListBasinsInput {
    pub prefix: String,
    pub start_after: String,
    pub limit: Option<usize>,
}

impl From<ListBasinsInput> for s2_sdk::types::ListBasinsInput {
    fn from(value: ListBasinsInput) -> Self {
        let mut input = s2_sdk::types::ListBasinsInput::default();
        input = input.with_prefix(s2_sdk::types::BasinNamePrefix::from_str(&value.prefix).unwrap());
        input = input.with_start_after(
            s2_sdk::types::BasinNameStartAfter::from_str(&value.start_after).unwrap(),
        );
        if let Some(limit) = value.limit {
            input = input.with_limit(limit);
        }
        input
    }
}

pub struct ListAllBasinsInput {
    pub prefix: String,
    pub start_after: String,
    pub include_deleted: bool,
}

impl From<ListAllBasinsInput> for s2_sdk::types::ListAllBasinsInput {
    fn from(value: ListAllBasinsInput) -> Self {
        let mut input = s2_sdk::types::ListAllBasinsInput::default();
        input = input.with_prefix(s2_sdk::types::BasinNamePrefix::from_str(&value.prefix).unwrap());
        input = input.with_start_after(
            s2_sdk::types::BasinNameStartAfter::from_str(&value.start_after).unwrap(),
        );
        input = input.with_include_deleted(value.include_deleted);
        input
    }
}

pub struct CreateBasinInput {
    pub name: String,
    pub config: Option<BasinConfig>,
    pub scope: Option<BasinScope>,
}

impl From<CreateBasinInput> for s2_sdk::types::CreateBasinInput {
    fn from(value: CreateBasinInput) -> Self {
        let mut input = s2_sdk::types::CreateBasinInput::new(
            s2_sdk::types::BasinName::from_str(&value.name).unwrap(),
        );
        if let Some(config) = value.config {
            input = input.with_config(config.into());
        }
        if let Some(scope) = value.scope {
            input = input.with_scope(scope.into());
        }
        input
    }
}

pub struct BasinConfig {
    pub default_stream_config: Option<StreamConfig>,
    pub create_stream_on_append: bool,
    pub create_stream_on_read: bool,
}

impl From<BasinConfig> for s2_sdk::types::BasinConfig {
    fn from(config: BasinConfig) -> Self {
        let mut basin_config = s2_sdk::types::BasinConfig::default();
        if let Some(stream_config) = config.default_stream_config {
            basin_config = basin_config.with_default_stream_config(stream_config.into());
        }
        basin_config = basin_config
            .with_create_stream_on_append(config.create_stream_on_append)
            .with_create_stream_on_read(config.create_stream_on_read);
        basin_config
    }
}

impl From<s2_sdk::types::BasinConfig> for BasinConfig {
    fn from(config: s2_sdk::types::BasinConfig) -> Self {
        BasinConfig {
            default_stream_config: config.default_stream_config.map(|sc| sc.into()),
            create_stream_on_append: config.create_stream_on_append,
            create_stream_on_read: config.create_stream_on_read,
        }
    }
}

pub struct DeleteBasinInput {
    pub name: String,
    pub ignore_not_found: bool,
}

impl From<DeleteBasinInput> for s2_sdk::types::DeleteBasinInput {
    fn from(input: DeleteBasinInput) -> Self {
        s2_sdk::types::DeleteBasinInput::new(
            s2_sdk::types::BasinName::from_str(&input.name).unwrap(),
        )
        .with_ignore_not_found(input.ignore_not_found)
    }
}

pub struct ReconfigureBasinInput {
    pub name: String,
    pub config: BasinConfig,
}

impl From<ReconfigureBasinInput> for s2_sdk::types::ReconfigureBasinInput {
    fn from(input: ReconfigureBasinInput) -> Self {
        s2_sdk::types::ReconfigureBasinInput::new(
            s2_sdk::types::BasinName::from_str(&input.name).unwrap(),
            input.config.into(),
        )
    }
}

impl From<BasinConfig> for s2_sdk::types::BasinReconfiguration {
    fn from(config: BasinConfig) -> Self {
        let mut reconfig = s2_sdk::types::BasinReconfiguration::default();
        if let Some(stream_config) = config.default_stream_config {
            let mut stream_reconfig = s2_sdk::types::StreamReconfiguration::default();
            if let Some(storage_class) = stream_config.storage_class {
                stream_reconfig = stream_reconfig.with_storage_class(storage_class.into());
            }
            if let Some(retention_policy) = stream_config.retention_policy {
                stream_reconfig = stream_reconfig.with_retention_policy(retention_policy.into());
            }
            if let Some(timestamping) = stream_config.timestamping {
                let mut timestamping_config = s2_sdk::types::TimestampingReconfiguration::default();
                if let Some(mode) = timestamping.mode {
                    timestamping_config = timestamping_config.with_mode(mode.into());
                }
                timestamping_config = timestamping_config.with_uncapped(timestamping.uncapped);
                stream_reconfig = stream_reconfig.with_timestamping(timestamping_config);
            }
            if let Some(delete_on_empty) = stream_config.delete_on_empty {
                let mut delete_on_empty_config = s2_sdk::types::DeleteOnEmptyReconfiguration::new();
                delete_on_empty_config = delete_on_empty_config
                    .with_min_age(Duration::from_secs(delete_on_empty.min_age_secs));
                stream_reconfig = stream_reconfig.with_delete_on_empty(delete_on_empty_config);
            }
            reconfig = reconfig.with_default_stream_config(stream_reconfig);
        }
        reconfig = reconfig
            .with_create_stream_on_append(config.create_stream_on_append)
            .with_create_stream_on_read(config.create_stream_on_read);
        reconfig
    }
}

pub struct ListAccessTokensInput {
    pub prefix: String,
    pub start_after: String,
    pub limit: Option<usize>,
}

impl From<ListAccessTokensInput> for s2_sdk::types::ListAccessTokensInput {
    fn from(input: ListAccessTokensInput) -> Self {
        let mut input = s2_sdk::types::ListAccessTokensInput::new()
            .with_prefix(s2_sdk::types::AccessTokenIdPrefix::from_str(&input.prefix).unwrap())
            .with_start_after(
                s2_sdk::types::AccessTokenIdStartAfter::from_str(&input.start_after).unwrap(),
            );
        if let Some(limit) = input.limit {
            input = input.with_limit(limit);
        }
        input
    }
}

pub struct AccessTokenInfo {
    pub id: String,
    pub expires_at: Option<u64>,
    pub auto_prefix_streams: bool,
    pub scope: AccessTokenScope,
}

impl From<s2_sdk::types::AccessTokenInfo> for AccessTokenInfo {
    fn from(value: s2_sdk::types::AccessTokenInfo) -> Self {
        Self {
            id: value.id.to_string(),
            expires_at: OffsetDateTime::from(value.expires_at)
                .unix_timestamp()
                .try_into()
                .ok(),
            auto_prefix_streams: value.auto_prefix_streams,
            scope: value.scope.into(),
        }
    }
}

pub struct AccessTokenScope {
    pub basins: ResourceSet,
    pub streams: ResourceSet,
    pub access_tokens: ResourceSet,
    pub op_group_permissions: Option<OperationGroupPermissions>,
    pub ops: Vec<Operation>,
}

impl From<s2_sdk::types::AccessTokenScope> for AccessTokenScope {
    fn from(value: s2_sdk::types::AccessTokenScope) -> Self {
        Self {
            basins: value.basins.map(|b| b.into()).unwrap_or(ResourceSet::None),
            streams: value.streams.map(|s| s.into()).unwrap_or(ResourceSet::None),
            access_tokens: value
                .access_tokens
                .map(|a| a.into())
                .unwrap_or(ResourceSet::None),
            op_group_permissions: value.op_group_perms.map(|o| o.into()),
            ops: value.ops.into_iter().map(|op| op.into()).collect(),
        }
    }
}

pub struct PageOfAccessTokenInfo {
    pub values: Vec<AccessTokenInfo>,
    pub has_more: bool,
}

pub enum ReadWritePermissions {
    Read,
    Write,
    ReadWrite,
}

impl From<s2_sdk::types::ReadWritePermissions> for ReadWritePermissions {
    fn from(value: s2_sdk::types::ReadWritePermissions) -> Self {
        if value.read && value.write {
            ReadWritePermissions::ReadWrite
        } else if value.read {
            ReadWritePermissions::Read
        } else if value.write {
            ReadWritePermissions::Write
        } else {
            ReadWritePermissions::Read
        }
    }
}

impl From<ReadWritePermissions> for s2_sdk::types::ReadWritePermissions {
    fn from(value: ReadWritePermissions) -> Self {
        match value {
            ReadWritePermissions::Read => s2_sdk::types::ReadWritePermissions::read_only(),
            ReadWritePermissions::Write => s2_sdk::types::ReadWritePermissions::write_only(),
            ReadWritePermissions::ReadWrite => s2_sdk::types::ReadWritePermissions::read_write(),
        }
    }
}

pub struct OperationGroupPermissions {
    pub basin: Option<ReadWritePermissions>,
    pub stream: Option<ReadWritePermissions>,
    pub account: Option<ReadWritePermissions>,
}

impl From<s2_sdk::types::OperationGroupPermissions> for OperationGroupPermissions {
    fn from(value: s2_sdk::types::OperationGroupPermissions) -> Self {
        Self {
            basin: value.basin.map(|b| b.into()),
            stream: value.stream.map(|s| s.into()),
            account: value.account.map(|a| a.into()),
        }
    }
}

impl From<OperationGroupPermissions> for s2_sdk::types::OperationGroupPermissions {
    fn from(value: OperationGroupPermissions) -> Self {
        let mut perms = s2_sdk::types::OperationGroupPermissions::default();
        if let Some(basin_perms) = value.basin {
            perms = perms.with_basin(basin_perms.into());
        }
        if let Some(stream_perms) = value.stream {
            perms = perms.with_stream(stream_perms.into());
        }
        if let Some(account_perms) = value.account {
            perms = perms.with_account(account_perms.into());
        }
        perms
    }
}

pub struct ListAllAccessTokensInput {
    pub prefix: String,
    pub start_after: String,
}

impl From<ListAllAccessTokensInput> for s2_sdk::types::ListAllAccessTokensInput {
    fn from(input: ListAllAccessTokensInput) -> Self {
        s2_sdk::types::ListAllAccessTokensInput::new()
            .with_prefix(s2_sdk::types::AccessTokenIdPrefix::from_str(&input.prefix).unwrap())
            .with_start_after(
                s2_sdk::types::AccessTokenIdStartAfter::from_str(&input.start_after).unwrap(),
            )
    }
}

pub struct IssueAccessTokenInput {
    pub id: String,
    pub expires_at: Option<u64>,
    pub auto_prefix_streams: bool,
    pub scope: AccessTokenScopeInput,
}

impl From<IssueAccessTokenInput> for s2_sdk::types::IssueAccessTokenInput {
    fn from(value: IssueAccessTokenInput) -> Self {
        let mut input = s2_sdk::types::IssueAccessTokenInput::new(
            s2_sdk::types::AccessTokenId::from_str(&value.id).unwrap(),
            value.scope.into(),
        )
        .with_auto_prefix_streams(value.auto_prefix_streams);
        if let Some(expires_at) = value.expires_at {
            input = input.with_expires_at(
                s2_sdk::types::S2DateTime::try_from(
                    OffsetDateTime::from_unix_timestamp(expires_at as i64).unwrap(),
                )
                .unwrap(),
            );
        }
        input
    }
}

pub struct AccessTokenScopeInput {
    pub basins: Option<ResourceSet>,
    pub streams: Option<ResourceSet>,
    pub access_tokens: Option<ResourceSet>,
    pub op_group_permissions: Option<OperationGroupPermissions>,
    pub ops: Vec<Operation>,
}

impl From<AccessTokenScopeInput> for s2_sdk::types::AccessTokenScopeInput {
    fn from(value: AccessTokenScopeInput) -> Self {
        let mut input = s2_sdk::types::AccessTokenScopeInput::from_ops(
            value.ops.into_iter().map(|op| op.into()),
        );
        if let Some(basins) = value.basins {
            input = input.with_basins(basins.into());
        }
        if let Some(streams) = value.streams {
            input = input.with_streams(streams.into());
        }
        if let Some(access_tokens) = value.access_tokens {
            input = input.with_access_tokens(access_tokens.into());
        }
        if let Some(op_group_permissions) = value.op_group_permissions {
            input = input.with_op_group_perms(op_group_permissions.into());
        }
        input
    }
}
