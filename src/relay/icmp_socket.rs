/**
 * The ICMP socket
 *
 * only impl ICMP_ECHO and ICMP_ECHO_REPLY methods .
 *
 */
use std::io::{self, Read, Write};
use std::net::{IpAddr, SocketAddr};

use mio::{event, Interest, Registry, Token};
use socket2::Domain;
use socket2::Protocol;
use socket2::Socket;
use socket2::Type;

pub struct IcmpSocket(Socket);

impl IcmpSocket {
    pub fn bind(ip: IpAddr) -> io::Result<IcmpSocket> {
        let protocol = match ip {
            IpAddr::V4(_) => Some(Protocol::ICMPV4),
            IpAddr::V6(_) => Some(Protocol::ICMPV6),
        };
        Socket::new(
            Domain::for_address(SocketAddr::new(ip, 0)),
            Type::RAW,
            protocol,
        )
        .map(|socket| {
            socket
                .set_nonblocking(true)
                .expect("socket set non blocking failed");
            IcmpSocket(socket)
        })
    }

    pub fn connect(&self, addr: &SocketAddr) -> io::Result<()> {
        self.0.connect(&(*addr).into())
    }
}

impl Write for IcmpSocket {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

impl Read for IcmpSocket {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        let mut bytes = vec![0u8; 512];
        let mut size = self.0.read(&mut bytes)?;
        // Drop IPV4 Header
        size = buf.write(&bytes[20..size])?;
        Ok(size)
    }
}

#[cfg(unix)]
use mio::unix::SourceFd;
#[cfg(unix)]
use std::os::unix::io::AsRawFd;

#[cfg(unix)]
impl event::Source for IcmpSocket {
    fn register(
        &mut self,
        registry: &Registry,
        token: Token,
        interests: Interest,
    ) -> io::Result<()> {
        SourceFd(&self.0.as_raw_fd()).register(registry, token, interests)
    }

    fn reregister(
        &mut self,
        registry: &Registry,
        token: Token,
        interests: Interest,
    ) -> io::Result<()> {
        SourceFd(&self.0.as_raw_fd()).reregister(registry, token, interests)
    }

    fn deregister(&mut self, registry: &mio::Registry) -> io::Result<()> {
        SourceFd(&self.0.as_raw_fd()).deregister(registry)
    }
}

// #[cfg(any(
//     target_os = "bitrig",
//     target_os = "dragonfly",
//     target_os = "freebsd",
//     target_os = "ios",
//     target_os = "macos",
//     target_os = "netbsd",
//     target_os = "openbsd"
// ))]
// #[allow(dead_code)]
// struct Selector {
//     id: usize,
//     kq: RawFd,
// }

// #[cfg(any(
//     target_os = "android",
//     target_os = "illumos",
//     target_os = "linux",
//     target_os = "solaris"
// ))]
// #[allow(dead_code)]
// struct Selector {
//     id: usize,
//     epfd: RawFd,
// }

// #[cfg(unix)]
// impl Selector {
//     pub fn id(&self) -> usize {
//         self.id
//     }
// }

// #[cfg(windows)]
// struct BufferPool {
//     pool: Vec<Vec<u8>>,
// }

// #[cfg(windows)]
// struct SelectorInner {
//     id: usize,
//     port: CompletionPort,
//     buffers: Mutex<BufferPool>,
// }
// #[cfg(windows)]
// struct Selector {
//     inner: Arc<SelectorInner>,
// }

// #[cfg(windows)]
// impl Selector {
//     pub fn id(&self) -> usize {
//         self.inner.id
//     }
// }

// struct ExRegistry {
//     pub selector: Selector
// }

// #[derive(Debug)]
// struct SelectorId {
//     id: AtomicUsize,
// }

// impl SelectorId {
//     pub fn new() -> SelectorId {
//         SelectorId {
//             id: AtomicUsize::new(0),
//         }
//     }

//     pub fn associate_selector(&self, registry: &Registry) -> io::Result<()> {
//         let registry_exposed
//         let registry_id = registry.selector().id();
//         let poll_exposed: &ExPoll = unsafe { transmute(poll) };
//         let selector_id = self.id.load(Ordering::SeqCst);
//         if selector_id != 0 && selector_id != poll_exposed.selector.id() {
//             Err(io::Error::new(
//                 io::ErrorKind::Other,
//                 "socket already registered",
//             ))
//         } else {
//             self.id.store(poll_exposed.selector.id(), Ordering::SeqCst);
//             Ok(())
//         }
//     }
// }

// impl Clone for SelectorId {
//     fn clone(&self) -> SelectorId {
//         SelectorId {
//             id: AtomicUsize::new(self.id.load(Ordering::SeqCst)),
//         }
//     }
// }
