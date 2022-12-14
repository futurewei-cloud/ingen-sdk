#[allow(clippy::all)]
pub mod common {}
#[allow(clippy::all)]
pub mod icmp {
    #[derive(Clone)]
    pub enum Error {
        ErrorWithDescription(String),
    }
    impl core::fmt::Debug for Error {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Error::ErrorWithDescription(e) => f.debug_tuple("Error::ErrorWithDescription").field(e).finish(),
            }
        }
    }
    #[derive(Clone)]
    pub struct IcmpPingResult {
        pub src_ip: String,
        pub dst_ip: String,
        pub icmp_code: u8,
        pub identifier: u16,
        pub seq: u16,
        pub ttl: u8,
        pub packet_size: u64,
        pub duration_in_microseconds: u64,
    }
    impl core::fmt::Debug for IcmpPingResult {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("IcmpPingResult")
                .field("src-ip", &self.src_ip)
                .field("dst-ip", &self.dst_ip)
                .field("icmp-code", &self.icmp_code)
                .field("identifier", &self.identifier)
                .field("seq", &self.seq)
                .field("ttl", &self.ttl)
                .field("packet-size", &self.packet_size)
                .field("duration-in-microseconds", &self.duration_in_microseconds)
                .finish()
        }
    }
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Icmp(i32);
    impl Icmp {
        pub unsafe fn from_raw(raw: i32) -> Self {
            Self(raw)
        }

        pub fn into_raw(self) -> i32 {
            let ret = self.0;
            core::mem::forget(self);
            return ret;
        }

        pub fn as_raw(&self) -> i32 {
            self.0
        }
    }
    impl Drop for Icmp {
        fn drop(&mut self) {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_drop_icmp"]
                fn close(fd: i32);
            }
            unsafe {
                close(self.0);
            }
        }
    }
    impl Clone for Icmp {
        fn clone(&self) -> Self {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_clone_icmp"]
                fn clone(val: i32) -> i32;
            }
            unsafe { Self(clone(self.0)) }
        }
    }
    impl Icmp {
        pub fn ping(target_ip: &str) -> Result<IcmpPingResult, Error> {
            unsafe {
                let vec0 = target_ip;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                let ptr1 = __ICMP_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "icmp")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "icmp::ping")]
                    #[cfg_attr(not(target_arch = "wasm32"), link_name = "icmp_icmp::ping")]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(ptr0, len0, ptr1);
                match i32::from(*((ptr1 + 0) as *const u8)) {
                    0 => Ok({
                        let len2 = *((ptr1 + 12) as *const i32) as usize;
                        let len3 = *((ptr1 + 20) as *const i32) as usize;

                        IcmpPingResult {
                            src_ip: String::from_utf8(Vec::from_raw_parts(
                                *((ptr1 + 8) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap(),
                            dst_ip: String::from_utf8(Vec::from_raw_parts(
                                *((ptr1 + 16) as *const i32) as *mut _,
                                len3,
                                len3,
                            ))
                            .unwrap(),
                            icmp_code: i32::from(*((ptr1 + 24) as *const u8)) as u8,
                            identifier: i32::from(*((ptr1 + 26) as *const u16)) as u16,
                            seq: i32::from(*((ptr1 + 28) as *const u16)) as u16,
                            ttl: i32::from(*((ptr1 + 30) as *const u8)) as u8,
                            packet_size: *((ptr1 + 32) as *const i64) as u64,
                            duration_in_microseconds: *((ptr1 + 40) as *const i64) as u64,
                        }
                    }),
                    1 => Err(match i32::from(*((ptr1 + 8) as *const u8)) {
                        0 => Error::ErrorWithDescription({
                            let len4 = *((ptr1 + 16) as *const i32) as usize;

                            String::from_utf8(Vec::from_raw_parts(*((ptr1 + 12) as *const i32) as *mut _, len4, len4))
                                .unwrap()
                        }),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Icmp {
        pub fn ping_with_options(
            target_ip: &str,
            source_ip: &str,
            identifier: u16,
            ttl: u8,
            seq: u16,
        ) -> Result<IcmpPingResult, Error> {
            unsafe {
                let vec0 = target_ip;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                let vec1 = source_ip;
                let ptr1 = vec1.as_ptr() as i32;
                let len1 = vec1.len() as i32;
                let ptr2 = __ICMP_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "icmp")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "icmp::ping-with-options")]
                    #[cfg_attr(not(target_arch = "wasm32"), link_name = "icmp_icmp::ping-with-options")]
                    fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32);
                }
                wit_import(
                    ptr0,
                    len0,
                    ptr1,
                    len1,
                    wit_bindgen_rust::rt::as_i32(identifier),
                    wit_bindgen_rust::rt::as_i32(ttl),
                    wit_bindgen_rust::rt::as_i32(seq),
                    ptr2,
                );
                match i32::from(*((ptr2 + 0) as *const u8)) {
                    0 => Ok({
                        let len3 = *((ptr2 + 12) as *const i32) as usize;
                        let len4 = *((ptr2 + 20) as *const i32) as usize;

                        IcmpPingResult {
                            src_ip: String::from_utf8(Vec::from_raw_parts(
                                *((ptr2 + 8) as *const i32) as *mut _,
                                len3,
                                len3,
                            ))
                            .unwrap(),
                            dst_ip: String::from_utf8(Vec::from_raw_parts(
                                *((ptr2 + 16) as *const i32) as *mut _,
                                len4,
                                len4,
                            ))
                            .unwrap(),
                            icmp_code: i32::from(*((ptr2 + 24) as *const u8)) as u8,
                            identifier: i32::from(*((ptr2 + 26) as *const u16)) as u16,
                            seq: i32::from(*((ptr2 + 28) as *const u16)) as u16,
                            ttl: i32::from(*((ptr2 + 30) as *const u8)) as u8,
                            packet_size: *((ptr2 + 32) as *const i64) as u64,
                            duration_in_microseconds: *((ptr2 + 40) as *const i64) as u64,
                        }
                    }),
                    1 => Err(match i32::from(*((ptr2 + 8) as *const u8)) {
                        0 => Error::ErrorWithDescription({
                            let len5 = *((ptr2 + 16) as *const i32) as usize;

                            String::from_utf8(Vec::from_raw_parts(*((ptr2 + 12) as *const i32) as *mut _, len5, len5))
                                .unwrap()
                        }),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }

    #[repr(align(8))]
    struct __IcmpRetArea([u8; 48]);
    static mut __ICMP_RET_AREA: __IcmpRetArea = __IcmpRetArea([0; 48]);
}
#[allow(clippy::all)]
pub mod socket {
    #[derive(Clone)]
    pub enum Error {
        ErrorWithDescription(String),
    }
    impl core::fmt::Debug for Error {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Error::ErrorWithDescription(e) => f.debug_tuple("Error::ErrorWithDescription").field(e).finish(),
            }
        }
    }
    pub type RawFd = i32;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct TcpBindOptions {
        pub backlog: u32,
        pub nonblocking: bool,
        pub reuse_address: bool,
    }
    impl core::fmt::Debug for TcpBindOptions {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("TcpBindOptions")
                .field("backlog", &self.backlog)
                .field("nonblocking", &self.nonblocking)
                .field("reuse-address", &self.reuse_address)
                .finish()
        }
    }
    #[derive(Clone)]
    pub struct TcpConnectOptions<'a> {
        pub local_endpoint: &'a str,
        pub nonblocking: bool,
        pub connect_timeout_in_ms: u32,
        pub keep_alive_timeout_in_ms: u32,
        pub reuse_address: bool,
        pub send_buffer_size: u32,
        pub recv_buffer_size: u32,
    }
    impl<'a> core::fmt::Debug for TcpConnectOptions<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("TcpConnectOptions")
                .field("local-endpoint", &self.local_endpoint)
                .field("nonblocking", &self.nonblocking)
                .field("connect-timeout-in-ms", &self.connect_timeout_in_ms)
                .field("keep-alive-timeout-in-ms", &self.keep_alive_timeout_in_ms)
                .field("reuse-address", &self.reuse_address)
                .field("send-buffer-size", &self.send_buffer_size)
                .field("recv-buffer-size", &self.recv_buffer_size)
                .finish()
        }
    }
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum ShutdownOption {
        Read,
        Write,
        Both,
    }
    impl core::fmt::Debug for ShutdownOption {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                ShutdownOption::Read => f.debug_tuple("ShutdownOption::Read").finish(),
                ShutdownOption::Write => f.debug_tuple("ShutdownOption::Write").finish(),
                ShutdownOption::Both => f.debug_tuple("ShutdownOption::Both").finish(),
            }
        }
    }
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Socket(i32);
    impl Socket {
        pub unsafe fn from_raw(raw: i32) -> Self {
            Self(raw)
        }

        pub fn into_raw(self) -> i32 {
            let ret = self.0;
            core::mem::forget(self);
            return ret;
        }

        pub fn as_raw(&self) -> i32 {
            self.0
        }
    }
    impl Drop for Socket {
        fn drop(&mut self) {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_drop_socket"]
                fn close(fd: i32);
            }
            unsafe {
                close(self.0);
            }
        }
    }
    impl Clone for Socket {
        fn clone(&self) -> Self {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_clone_socket"]
                fn clone(val: i32) -> i32;
            }
            unsafe { Self(clone(self.0)) }
        }
    }
    impl Socket {
        pub fn tcp_bind(endpoint: &str, options: TcpBindOptions) -> Result<RawFd, Error> {
            unsafe {
                let vec0 = endpoint;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                let TcpBindOptions { backlog: backlog1, nonblocking: nonblocking1, reuse_address: reuse_address1 } =
                    options;
                let ptr2 = __SOCKET_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "socket")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "socket::tcp-bind")]
                    #[cfg_attr(not(target_arch = "wasm32"), link_name = "socket_socket::tcp-bind")]
                    fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32);
                }
                wit_import(
                    ptr0,
                    len0,
                    wit_bindgen_rust::rt::as_i32(backlog1),
                    match nonblocking1 {
                        true => 1,
                        false => 0,
                    },
                    match reuse_address1 {
                        true => 1,
                        false => 0,
                    },
                    ptr2,
                );
                match i32::from(*((ptr2 + 0) as *const u8)) {
                    0 => Ok(*((ptr2 + 4) as *const i32)),
                    1 => Err(match i32::from(*((ptr2 + 4) as *const u8)) {
                        0 => Error::ErrorWithDescription({
                            let len3 = *((ptr2 + 12) as *const i32) as usize;

                            String::from_utf8(Vec::from_raw_parts(*((ptr2 + 8) as *const i32) as *mut _, len3, len3))
                                .unwrap()
                        }),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Socket {
        pub fn tcp_connect(remote_endpoint: &str, options: TcpConnectOptions<'_>) -> Result<RawFd, Error> {
            unsafe {
                let vec0 = remote_endpoint;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                let TcpConnectOptions {
                    local_endpoint: local_endpoint1,
                    nonblocking: nonblocking1,
                    connect_timeout_in_ms: connect_timeout_in_ms1,
                    keep_alive_timeout_in_ms: keep_alive_timeout_in_ms1,
                    reuse_address: reuse_address1,
                    send_buffer_size: send_buffer_size1,
                    recv_buffer_size: recv_buffer_size1,
                } = options;
                let vec2 = local_endpoint1;
                let ptr2 = vec2.as_ptr() as i32;
                let len2 = vec2.len() as i32;
                let ptr3 = __SOCKET_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "socket")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "socket::tcp-connect")]
                    #[cfg_attr(not(target_arch = "wasm32"), link_name = "socket_socket::tcp-connect")]
                    fn wit_import(
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                    );
                }
                wit_import(
                    ptr0,
                    len0,
                    ptr2,
                    len2,
                    match nonblocking1 {
                        true => 1,
                        false => 0,
                    },
                    wit_bindgen_rust::rt::as_i32(connect_timeout_in_ms1),
                    wit_bindgen_rust::rt::as_i32(keep_alive_timeout_in_ms1),
                    match reuse_address1 {
                        true => 1,
                        false => 0,
                    },
                    wit_bindgen_rust::rt::as_i32(send_buffer_size1),
                    wit_bindgen_rust::rt::as_i32(recv_buffer_size1),
                    ptr3,
                );
                match i32::from(*((ptr3 + 0) as *const u8)) {
                    0 => Ok(*((ptr3 + 4) as *const i32)),
                    1 => Err(match i32::from(*((ptr3 + 4) as *const u8)) {
                        0 => Error::ErrorWithDescription({
                            let len4 = *((ptr3 + 12) as *const i32) as usize;

                            String::from_utf8(Vec::from_raw_parts(*((ptr3 + 8) as *const i32) as *mut _, len4, len4))
                                .unwrap()
                        }),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Socket {
        pub fn shutdown(fd: RawFd, opt: ShutdownOption) -> Result<(), Error> {
            unsafe {
                let ptr0 = __SOCKET_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "socket")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "socket::shutdown")]
                    #[cfg_attr(not(target_arch = "wasm32"), link_name = "socket_socket::shutdown")]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(
                    wit_bindgen_rust::rt::as_i32(fd),
                    match opt {
                        ShutdownOption::Read => 0,
                        ShutdownOption::Write => 1,
                        ShutdownOption::Both => 2,
                    },
                    ptr0,
                );
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(()),
                    1 => Err(match i32::from(*((ptr0 + 4) as *const u8)) {
                        0 => Error::ErrorWithDescription({
                            let len1 = *((ptr0 + 12) as *const i32) as usize;

                            String::from_utf8(Vec::from_raw_parts(*((ptr0 + 8) as *const i32) as *mut _, len1, len1))
                                .unwrap()
                        }),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Socket {
        pub fn get_local_addr(fd: RawFd) -> Result<String, Error> {
            unsafe {
                let ptr0 = __SOCKET_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "socket")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "socket::get-local-addr")]
                    #[cfg_attr(not(target_arch = "wasm32"), link_name = "socket_socket::get-local-addr")]
                    fn wit_import(_: i32, _: i32);
                }
                wit_import(wit_bindgen_rust::rt::as_i32(fd), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok({
                        let len1 = *((ptr0 + 8) as *const i32) as usize;

                        String::from_utf8(Vec::from_raw_parts(*((ptr0 + 4) as *const i32) as *mut _, len1, len1))
                            .unwrap()
                    }),
                    1 => Err(match i32::from(*((ptr0 + 4) as *const u8)) {
                        0 => Error::ErrorWithDescription({
                            let len2 = *((ptr0 + 12) as *const i32) as usize;

                            String::from_utf8(Vec::from_raw_parts(*((ptr0 + 8) as *const i32) as *mut _, len2, len2))
                                .unwrap()
                        }),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Socket {
        pub fn get_peer_addr(fd: RawFd) -> Result<String, Error> {
            unsafe {
                let ptr0 = __SOCKET_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "socket")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "socket::get-peer-addr")]
                    #[cfg_attr(not(target_arch = "wasm32"), link_name = "socket_socket::get-peer-addr")]
                    fn wit_import(_: i32, _: i32);
                }
                wit_import(wit_bindgen_rust::rt::as_i32(fd), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok({
                        let len1 = *((ptr0 + 8) as *const i32) as usize;

                        String::from_utf8(Vec::from_raw_parts(*((ptr0 + 4) as *const i32) as *mut _, len1, len1))
                            .unwrap()
                    }),
                    1 => Err(match i32::from(*((ptr0 + 4) as *const u8)) {
                        0 => Error::ErrorWithDescription({
                            let len2 = *((ptr0 + 12) as *const i32) as usize;

                            String::from_utf8(Vec::from_raw_parts(*((ptr0 + 8) as *const i32) as *mut _, len2, len2))
                                .unwrap()
                        }),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Socket {
        pub fn get_ttl(fd: RawFd) -> Result<u32, Error> {
            unsafe {
                let ptr0 = __SOCKET_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "socket")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "socket::get-ttl")]
                    #[cfg_attr(not(target_arch = "wasm32"), link_name = "socket_socket::get-ttl")]
                    fn wit_import(_: i32, _: i32);
                }
                wit_import(wit_bindgen_rust::rt::as_i32(fd), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(*((ptr0 + 4) as *const i32) as u32),
                    1 => Err(match i32::from(*((ptr0 + 4) as *const u8)) {
                        0 => Error::ErrorWithDescription({
                            let len1 = *((ptr0 + 12) as *const i32) as usize;

                            String::from_utf8(Vec::from_raw_parts(*((ptr0 + 8) as *const i32) as *mut _, len1, len1))
                                .unwrap()
                        }),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Socket {
        pub fn set_ttl(fd: RawFd, ttl: u32) -> Result<(), Error> {
            unsafe {
                let ptr0 = __SOCKET_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "socket")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "socket::set-ttl")]
                    #[cfg_attr(not(target_arch = "wasm32"), link_name = "socket_socket::set-ttl")]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(wit_bindgen_rust::rt::as_i32(fd), wit_bindgen_rust::rt::as_i32(ttl), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(()),
                    1 => Err(match i32::from(*((ptr0 + 4) as *const u8)) {
                        0 => Error::ErrorWithDescription({
                            let len1 = *((ptr0 + 12) as *const i32) as usize;

                            String::from_utf8(Vec::from_raw_parts(*((ptr0 + 8) as *const i32) as *mut _, len1, len1))
                                .unwrap()
                        }),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Socket {
        pub fn get_nodelay(fd: RawFd) -> Result<bool, Error> {
            unsafe {
                let ptr0 = __SOCKET_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "socket")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "socket::get-nodelay")]
                    #[cfg_attr(not(target_arch = "wasm32"), link_name = "socket_socket::get-nodelay")]
                    fn wit_import(_: i32, _: i32);
                }
                wit_import(wit_bindgen_rust::rt::as_i32(fd), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(match i32::from(*((ptr0 + 4) as *const u8)) {
                        0 => false,
                        1 => true,
                        _ => panic!("invalid bool discriminant"),
                    }),
                    1 => Err(match i32::from(*((ptr0 + 4) as *const u8)) {
                        0 => Error::ErrorWithDescription({
                            let len1 = *((ptr0 + 12) as *const i32) as usize;

                            String::from_utf8(Vec::from_raw_parts(*((ptr0 + 8) as *const i32) as *mut _, len1, len1))
                                .unwrap()
                        }),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Socket {
        pub fn set_nodelay(fd: RawFd, nodelay: bool) -> Result<(), Error> {
            unsafe {
                let ptr0 = __SOCKET_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "socket")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "socket::set-nodelay")]
                    #[cfg_attr(not(target_arch = "wasm32"), link_name = "socket_socket::set-nodelay")]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(
                    wit_bindgen_rust::rt::as_i32(fd),
                    match nodelay {
                        true => 1,
                        false => 0,
                    },
                    ptr0,
                );
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(()),
                    1 => Err(match i32::from(*((ptr0 + 4) as *const u8)) {
                        0 => Error::ErrorWithDescription({
                            let len1 = *((ptr0 + 12) as *const i32) as usize;

                            String::from_utf8(Vec::from_raw_parts(*((ptr0 + 8) as *const i32) as *mut _, len1, len1))
                                .unwrap()
                        }),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Socket {
        pub fn set_keepalive(fd: RawFd, keepalive_in_ms: u32) -> Result<(), Error> {
            unsafe {
                let ptr0 = __SOCKET_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "socket")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "socket::set-keepalive")]
                    #[cfg_attr(not(target_arch = "wasm32"), link_name = "socket_socket::set-keepalive")]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(wit_bindgen_rust::rt::as_i32(fd), wit_bindgen_rust::rt::as_i32(keepalive_in_ms), ptr0);
                match i32::from(*((ptr0 + 0) as *const u8)) {
                    0 => Ok(()),
                    1 => Err(match i32::from(*((ptr0 + 4) as *const u8)) {
                        0 => Error::ErrorWithDescription({
                            let len1 = *((ptr0 + 12) as *const i32) as usize;

                            String::from_utf8(Vec::from_raw_parts(*((ptr0 + 8) as *const i32) as *mut _, len1, len1))
                                .unwrap()
                        }),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl Socket {
        pub fn nslookup(domain_name: &str) -> Result<Vec<String>, Error> {
            unsafe {
                let vec0 = domain_name;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                let ptr1 = __SOCKET_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "socket")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "socket::nslookup")]
                    #[cfg_attr(not(target_arch = "wasm32"), link_name = "socket_socket::nslookup")]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(ptr0, len0, ptr1);
                match i32::from(*((ptr1 + 0) as *const u8)) {
                    0 => Ok({
                        let base3 = *((ptr1 + 4) as *const i32);
                        let len3 = *((ptr1 + 8) as *const i32);
                        let mut result3 = Vec::with_capacity(len3 as usize);
                        for i in 0..len3 {
                            let base = base3 + i * 8;
                            result3.push({
                                let len2 = *((base + 4) as *const i32) as usize;

                                String::from_utf8(Vec::from_raw_parts(
                                    *((base + 0) as *const i32) as *mut _,
                                    len2,
                                    len2,
                                ))
                                .unwrap()
                            });
                        }
                        if len3 != 0 {
                            std::alloc::dealloc(
                                base3 as *mut _,
                                std::alloc::Layout::from_size_align_unchecked((len3 as usize) * 8, 4),
                            );
                        }

                        result3
                    }),
                    1 => Err(match i32::from(*((ptr1 + 4) as *const u8)) {
                        0 => Error::ErrorWithDescription({
                            let len4 = *((ptr1 + 12) as *const i32) as usize;

                            String::from_utf8(Vec::from_raw_parts(*((ptr1 + 8) as *const i32) as *mut _, len4, len4))
                                .unwrap()
                        }),
                        _ => panic!("invalid enum discriminant"),
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }

    #[repr(align(8))]
    struct __SocketRetArea([u8; 48]);
    static mut __SOCKET_RET_AREA: __SocketRetArea = __SocketRetArea([0; 48]);
}
