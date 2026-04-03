pub mod append_session;
pub mod basin;
pub mod client;
pub mod error;
mod frb_generated;
pub mod producer;
pub mod stream;
pub mod types;

#[flutter_rust_bridge::frb(init)]
pub async fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}
