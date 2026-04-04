use flutter_rust_bridge::{RustAutoOpaqueNom, frb};

use crate::types::ClientConfig;

#[frb(opaque)]
pub struct S2Client {
    client: RustAutoOpaqueNom<s2_sdk::S2>,
}

impl S2Client {
    pub fn new(config: ClientConfig) -> S2Client {
        S2Client {
            client: RustAutoOpaqueNom::new(s2_sdk::S2::new(config.into()).unwrap()),
        }
    }

    pub async fn basin(&self) {}

    pub async fn list_basins(&self) {}

    pub async fn list_all_basins(&self) {}

    pub async fn create_basin(&self) {}

    pub async fn get_basin_config(&self) {}

    pub async fn delete_basin(&self) {}

    pub async fn reconfigure_basin(&self) {}

    pub async fn list_access_tokens(&self) {}

    pub async fn list_all_access_tokens(&self) {}

    pub async fn issue_access_token(&self) {}

    pub async fn revoke_access_token(&self) {}
}
