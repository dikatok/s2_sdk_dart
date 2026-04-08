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
    pub async fn new(config: ClientConfig) -> Result<S2Client, S2Error> {
        let connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_or_http()
            .enable_http1()
            .build();
        let s2 = s2_sdk::S2::new_with_connector(config.try_into()?, connector)?;
        Ok(S2Client {
            client: RustAutoOpaqueNom::new(s2),
        })
    }

    #[frb(sync)]
    pub fn basin(&self, name: String) -> Result<S2Basin, S2Error> {
        let basin = self
            .client
            .try_read()
            .unwrap()
            .basin(s2_sdk::types::BasinName::from_str(name.as_str())?)
            .into();
        Ok(basin)
    }

    pub async fn list_basins(&self, input: ListBasinsInput) -> Result<PageOfBasinInfo, S2Error> {
        self.client
            .try_read()
            .unwrap()
            .list_basins(input.try_into()?)
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
            .list_all_basins(input.try_into()?);
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
            .create_basin(input.try_into()?)
            .await
            .map(|info| info.into())
            .map_err(|e| e.into())
    }

    pub async fn get_basin_config(&self, name: String) -> Result<BasinConfig, S2Error> {
        self.client
            .try_read()
            .unwrap()
            .get_basin_config(s2_sdk::types::BasinName::from_str(name.as_str())?)
            .await
            .map(|config| config.into())
            .map_err(|e| e.into())
    }

    pub async fn delete_basin(&self, input: DeleteBasinInput) -> Result<(), S2Error> {
        self.client
            .try_read()
            .unwrap()
            .delete_basin(input.try_into()?)
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
            .reconfigure_basin(input.try_into()?)
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
            .list_access_tokens(input.try_into()?)
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
            .list_all_access_tokens(input.try_into()?);
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
            .issue_access_token(input.try_into()?)
            .await
            .map(|s| s.to_string())
            .map_err(|e| e.into())
    }

    pub async fn revoke_access_token(&self, id: String) -> Result<(), S2Error> {
        self.client
            .try_read()
            .unwrap()
            .revoke_access_token(s2_sdk::types::AccessTokenId::from_str(&id)?)
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
    fn from(value: s2_sdk::types::BasinInfo) -> Self {
        BasinInfo {
            name: value.name.to_string(),
            scope: value.scope.map(|s| s.into()),
            created_at: OffsetDateTime::from(value.created_at).unix_timestamp() as u64,
            deleted_at: value
                .deleted_at
                .map(|dt| OffsetDateTime::from(dt).unix_timestamp() as u64),
        }
    }
}

pub enum BasinScope {
    AwsUsEast1,
}

impl From<s2_sdk::types::BasinScope> for BasinScope {
    fn from(value: s2_sdk::types::BasinScope) -> Self {
        match value {
            s2_sdk::types::BasinScope::AwsUsEast1 => BasinScope::AwsUsEast1,
        }
    }
}

impl From<BasinScope> for s2_sdk::types::BasinScope {
    fn from(value: BasinScope) -> Self {
        match value {
            BasinScope::AwsUsEast1 => s2_sdk::types::BasinScope::AwsUsEast1,
        }
    }
}

pub struct PageOfBasinInfo {
    pub values: Vec<BasinInfo>,
    pub has_more: bool,
}

pub struct ListBasinsInput {
    pub prefix: Option<String>,
    pub start_after: Option<String>,
    pub limit: Option<u64>,
}

impl TryFrom<ListBasinsInput> for s2_sdk::types::ListBasinsInput {
    type Error = S2Error;

    fn try_from(value: ListBasinsInput) -> Result<Self, Self::Error> {
        let mut input = s2_sdk::types::ListBasinsInput::new();
        if let Some(prefix) = value.prefix {
            input = input.with_prefix(s2_sdk::types::BasinNamePrefix::from_str(&prefix)?);
        }
        if let Some(start_after) = value.start_after {
            input =
                input.with_start_after(s2_sdk::types::BasinNameStartAfter::from_str(&start_after)?);
        }
        if let Some(limit) = value.limit {
            input = input.with_limit(limit.try_into()?);
        }
        Ok(input)
    }
}

pub struct ListAllBasinsInput {
    pub prefix: Option<String>,
    pub start_after: Option<String>,
    pub include_deleted: Option<bool>,
}

impl TryFrom<ListAllBasinsInput> for s2_sdk::types::ListAllBasinsInput {
    type Error = S2Error;

    fn try_from(value: ListAllBasinsInput) -> Result<Self, Self::Error> {
        let mut input = s2_sdk::types::ListAllBasinsInput::default();
        if let Some(prefix) = value.prefix {
            input = input.with_prefix(s2_sdk::types::BasinNamePrefix::from_str(&prefix)?);
        }
        if let Some(start_after) = value.start_after {
            input =
                input.with_start_after(s2_sdk::types::BasinNameStartAfter::from_str(&start_after)?);
        }
        if let Some(include_deleted) = value.include_deleted {
            input = input.with_include_deleted(include_deleted);
        }
        Ok(input)
    }
}

pub struct CreateBasinInput {
    pub name: String,
    pub config: Option<BasinConfig>,
    pub scope: Option<BasinScope>,
}

impl TryFrom<CreateBasinInput> for s2_sdk::types::CreateBasinInput {
    type Error = S2Error;

    fn try_from(value: CreateBasinInput) -> Result<Self, Self::Error> {
        let mut input =
            s2_sdk::types::CreateBasinInput::new(s2_sdk::types::BasinName::from_str(&value.name)?);
        if let Some(config) = value.config {
            input = input.with_config(config.into());
        }
        if let Some(scope) = value.scope {
            input = input.with_scope(scope.into());
        }
        Ok(input)
    }
}

pub struct BasinConfig {
    pub default_stream_config: Option<StreamConfig>,
    pub create_stream_on_append: bool,
    pub create_stream_on_read: bool,
}

impl From<BasinConfig> for s2_sdk::types::BasinConfig {
    fn from(value: BasinConfig) -> Self {
        let mut basin_config = s2_sdk::types::BasinConfig::default();
        if let Some(stream_config) = value.default_stream_config {
            basin_config = basin_config.with_default_stream_config(stream_config.into());
        }
        basin_config = basin_config
            .with_create_stream_on_append(value.create_stream_on_append)
            .with_create_stream_on_read(value.create_stream_on_read);
        basin_config
    }
}

impl From<s2_sdk::types::BasinConfig> for BasinConfig {
    fn from(value: s2_sdk::types::BasinConfig) -> Self {
        BasinConfig {
            default_stream_config: value.default_stream_config.map(|sc| sc.into()),
            create_stream_on_append: value.create_stream_on_append,
            create_stream_on_read: value.create_stream_on_read,
        }
    }
}

pub struct DeleteBasinInput {
    pub name: String,
    pub ignore_not_found: bool,
}

impl TryFrom<DeleteBasinInput> for s2_sdk::types::DeleteBasinInput {
    type Error = S2Error;

    fn try_from(value: DeleteBasinInput) -> Result<Self, Self::Error> {
        Ok(
            s2_sdk::types::DeleteBasinInput::new(s2_sdk::types::BasinName::from_str(&value.name)?)
                .with_ignore_not_found(value.ignore_not_found),
        )
    }
}

pub struct ReconfigureBasinInput {
    pub name: String,
    pub config: BasinConfig,
}

impl TryFrom<ReconfigureBasinInput> for s2_sdk::types::ReconfigureBasinInput {
    type Error = S2Error;

    fn try_from(value: ReconfigureBasinInput) -> Result<Self, Self::Error> {
        Ok(s2_sdk::types::ReconfigureBasinInput::new(
            s2_sdk::types::BasinName::from_str(&value.name)?,
            value.config.into(),
        ))
    }
}

impl From<BasinConfig> for s2_sdk::types::BasinReconfiguration {
    fn from(value: BasinConfig) -> Self {
        let mut reconfig = s2_sdk::types::BasinReconfiguration::default();
        if let Some(stream_config) = value.default_stream_config {
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
                if let Some(uncapped) = timestamping.uncapped {
                    timestamping_config = timestamping_config.with_uncapped(uncapped);
                }
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
            .with_create_stream_on_append(value.create_stream_on_append)
            .with_create_stream_on_read(value.create_stream_on_read);
        reconfig
    }
}

pub struct ListAccessTokensInput {
    pub prefix: Option<String>,
    pub start_after: Option<String>,
    pub limit: Option<u64>,
}

impl TryFrom<ListAccessTokensInput> for s2_sdk::types::ListAccessTokensInput {
    type Error = S2Error;

    fn try_from(value: ListAccessTokensInput) -> Result<Self, Self::Error> {
        let mut input = s2_sdk::types::ListAccessTokensInput::new();
        if let Some(prefix) = value.prefix {
            input = input.with_prefix(s2_sdk::types::AccessTokenIdPrefix::from_str(&prefix)?);
        }
        if let Some(start_after) = value.start_after {
            input = input.with_start_after(s2_sdk::types::AccessTokenIdStartAfter::from_str(
                &start_after,
            )?);
        }
        if let Some(limit) = input.limit {
            input = input.with_limit(limit);
        }
        Ok(input)
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
    pub prefix: Option<String>,
    pub start_after: Option<String>,
}

impl TryFrom<ListAllAccessTokensInput> for s2_sdk::types::ListAllAccessTokensInput {
    type Error = S2Error;

    fn try_from(value: ListAllAccessTokensInput) -> Result<Self, Self::Error> {
        let mut input = s2_sdk::types::ListAllAccessTokensInput::new();
        if let Some(prefix) = value.prefix {
            input = input.with_prefix(s2_sdk::types::AccessTokenIdPrefix::from_str(&prefix)?);
        }
        if let Some(start_after) = value.start_after {
            input = input.with_start_after(s2_sdk::types::AccessTokenIdStartAfter::from_str(
                &start_after,
            )?);
        }
        Ok(input)
    }
}

pub struct IssueAccessTokenInput {
    pub id: String,
    pub expires_at: Option<u64>,
    pub auto_prefix_streams: bool,
    pub scope: AccessTokenScopeInput,
}

impl TryFrom<IssueAccessTokenInput> for s2_sdk::types::IssueAccessTokenInput {
    type Error = S2Error;

    fn try_from(value: IssueAccessTokenInput) -> Result<Self, Self::Error> {
        let mut input = s2_sdk::types::IssueAccessTokenInput::new(
            s2_sdk::types::AccessTokenId::from_str(&value.id)?,
            value.scope.try_into()?,
        )
        .with_auto_prefix_streams(value.auto_prefix_streams);
        if let Some(expires_at) = value.expires_at {
            input = input.with_expires_at(s2_sdk::types::S2DateTime::try_from(
                match OffsetDateTime::from_unix_timestamp(expires_at as i64) {
                    Ok(o) => o,
                    Err(e) => return Err(S2Error::from_str(e.to_string().as_str()).unwrap()),
                },
            )?);
        }
        Ok(input)
    }
}

pub struct AccessTokenScopeInput {
    pub basins: Option<ResourceSet>,
    pub streams: Option<ResourceSet>,
    pub access_tokens: Option<ResourceSet>,
    pub op_group_permissions: Option<OperationGroupPermissions>,
    pub ops: Vec<Operation>,
}

impl TryFrom<AccessTokenScopeInput> for s2_sdk::types::AccessTokenScopeInput {
    type Error = S2Error;

    fn try_from(value: AccessTokenScopeInput) -> Result<Self, Self::Error> {
        let mut input = s2_sdk::types::AccessTokenScopeInput::from_ops(
            value.ops.into_iter().map(|op| op.into()),
        );
        if let Some(basins) = value.basins {
            input = input.with_basins(basins.try_into()?);
        }
        if let Some(streams) = value.streams {
            input = input.with_streams(streams.try_into()?);
        }
        if let Some(access_tokens) = value.access_tokens {
            input = input.with_access_tokens(access_tokens.try_into()?);
        }
        if let Some(op_group_permissions) = value.op_group_permissions {
            input = input.with_op_group_perms(op_group_permissions.into());
        }
        Ok(input)
    }
}
