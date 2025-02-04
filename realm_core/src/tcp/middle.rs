use std::io::Result;

use tokio::net::TcpStream;

use super::socket;
use super::plain;

#[cfg(feature = "transport")]
use super::transport;

use crate::trick::Ref;
use crate::endpoint::{RemoteAddr, ConnectOpts};

pub async fn connect_and_relay(local: TcpStream, raddr: Ref<RemoteAddr>, conn_opts: Ref<ConnectOpts>) -> Result<()> {
    let ConnectOpts {
        #[cfg(feature = "transport")]
        transport,
        ..
    } = conn_opts.as_ref();

    // before connect
    // ..

    // connect!
    let remote = socket::connect(raddr.as_ref(), conn_opts.as_ref()).await?;
    log::info!(
        "[tcp]{} => {} as {}",
        local.peer_addr().unwrap(),
        raddr.as_ref(),
        remote.peer_addr().unwrap()
    );

    // after connected
    // ..

    // relay
    let res = {
        #[cfg(feature = "transport")]
        {
            if let Some((ac, cc)) = transport {
                transport::run_relay(local, remote, ac, cc).await
            } else {
                plain::run_relay(local, remote).await
            }
        }
        #[cfg(not(feature = "transport"))]
        {
            plain::run_relay(local, remote).await
        }
    };

    // ignore relay error
    if let Err(e) = res {
        log::debug!("[tcp]forward error: {}, ignored", e);
    }

    Ok(())
}
