use std::os::fd::{FromRawFd, IntoRawFd};
use std::os::unix::net::UnixStream;
use std::sync::Arc;

use smithay::reexports::wayland_server::DisplayHandle;
use zbus::dbus_interface;

use crate::niri::ClientState;

pub struct ServiceChannel {
    display: DisplayHandle,
}

#[dbus_interface(name = "org.gnome.Mutter.ServiceChannel")]
impl ServiceChannel {
    async fn open_wayland_service_connection(
        &mut self,
        service_client_type: u32,
    ) -> zbus::fdo::Result<zbus::zvariant::OwnedFd> {
        if service_client_type != 1 {
            return Err(zbus::fdo::Error::InvalidArgs(
                "Invalid service client type".to_owned(),
            ));
        }

        let (sock1, sock2) = UnixStream::pair().unwrap();
        self.display
            .insert_client(sock2, Arc::new(ClientState::default()))
            .unwrap();
        Ok(unsafe { zbus::zvariant::OwnedFd::from_raw_fd(sock1.into_raw_fd()) })
    }
}

impl ServiceChannel {
    pub fn new(display: DisplayHandle) -> Self {
        Self { display }
    }
}
