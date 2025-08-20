use std::{
    mem::transmute,
    net::{Ipv4Addr, UdpSocket},
    os::fd::AsRawFd,
};

#[derive(Debug, Clone)]
pub struct MagicPacket {
    magic_bytes: [u8; 102],
    interface: Option<String>,
}

impl MagicPacket {
    pub const fn new(mac_address: &[u8; 6], interface: Option<String>) -> MagicPacket {
        let mut magic_bytes: [[u8; 6]; 17] = [*mac_address; 17];
        magic_bytes[0] = [0xFF; 6];
        let magic_bytes = unsafe { transmute::<[[u8; 6]; 17], [u8; 102]>(magic_bytes) };
        MagicPacket {
            magic_bytes,
            interface,
        }
    }

    #[inline]
    pub fn send(&self) -> std::io::Result<()> {
        self.send_from(self.interface.as_deref())
    }

    pub fn send_from(&self, interface: Option<&str>) -> std::io::Result<()> {
        macro_rules! syscall {
            ($fn: ident ( $($arg: expr),* $(,)* ) ) => {{
                #[allow(unused_unsafe)]
                let res = unsafe { libc::$fn($($arg, )*) };
                if res == -1 {
                    Err(std::io::Error::last_os_error())
                } else {
                    Ok(res)
                }
            }};
        }

        let socket = UdpSocket::bind((Ipv4Addr::new(0, 0, 0, 0), 0))?;
        socket.set_broadcast(true)?;
        if let Some(interface) = interface {
            syscall!(setsockopt(
                socket.as_raw_fd(),
                libc::SOL_SOCKET,
                libc::SO_BINDTODEVICE,
                interface.as_ptr().cast(),
                interface.len() as _,
            ))?;
        }
        socket.send_to(&self.magic_bytes, (Ipv4Addr::new(255, 255, 255, 255), 9))?;
        Ok(())
    }

    pub const fn magic_bytes(&self) -> &[u8; 102] {
        &self.magic_bytes
    }
}
