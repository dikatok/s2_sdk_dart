use std::str::FromStr;

use flutter_rust_bridge::{RustAutoOpaqueNom, frb};

use crate::{
    error::S2Error,
    types::{AppendAck, AppendInput},
};

#[frb(opaque)]
pub struct S2AppendSession {
    session: RustAutoOpaqueNom<Option<s2_sdk::append_session::AppendSession>>,
}

impl S2AppendSession {
    pub async fn submit(&self, record: AppendInput) -> Result<BatchSubmitTicket, S2Error> {
        match self
            .session
            .try_read()
            .unwrap()
            .as_ref()
            .unwrap()
            .submit(record.into())
            .await
        {
            Ok(ticket) => Ok(ticket.into()),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn close(self) -> Result<(), S2Error> {
        let guard = self.session.try_write().unwrap().take();
        if let Some(session) = guard {
            match session.close().await {
                Ok(()) => return Ok(()),
                Err(e) => return Err(e.into()),
            }
        }
        Err(S2Error::from_str("Session is already closed").unwrap())
    }
}

impl From<s2_sdk::append_session::AppendSession> for S2AppendSession {
    fn from(session: s2_sdk::append_session::AppendSession) -> Self {
        S2AppendSession {
            session: RustAutoOpaqueNom::new(Some(session)),
        }
    }
}

#[frb(opaque)]
pub struct BatchSubmitTicket {
    pub(crate) ticket: RustAutoOpaqueNom<Option<s2_sdk::append_session::BatchSubmitTicket>>,
}

impl From<s2_sdk::append_session::BatchSubmitTicket> for BatchSubmitTicket {
    fn from(ticket: s2_sdk::append_session::BatchSubmitTicket) -> Self {
        BatchSubmitTicket {
            ticket: RustAutoOpaqueNom::new(Some(ticket)),
        }
    }
}

impl BatchSubmitTicket {
    pub async fn ack(self) -> Result<AppendAck, S2Error> {
        match self.ticket.try_write().unwrap().take().unwrap().await {
            Ok(ack) => Ok(ack.into()),
            Err(e) => Err(e.into()),
        }
    }
}
