#[allow(clippy::all)]
pub mod common {
    #[allow(unused_imports)]
    use wit_bindgen_wasmtime::{anyhow, wasmtime};
}
#[allow(clippy::all)]
pub mod socket {
    #[allow(unused_imports)]
    use wit_bindgen_wasmtime::{anyhow, wasmtime};
    #[derive(Clone)]
    pub enum Error {
        ErrorWithDescription(String),
    }
    impl core::fmt::Debug for Error {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Error::ErrorWithDescription(e) => f
                    .debug_tuple("Error::ErrorWithDescription")
                    .field(e)
                    .finish(),
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
    }
    impl<'a> core::fmt::Debug for TcpConnectOptions<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("TcpConnectOptions")
                .field("local-endpoint", &self.local_endpoint)
                .field("nonblocking", &self.nonblocking)
                .field("connect-timeout-in-ms", &self.connect_timeout_in_ms)
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
    #[wit_bindgen_wasmtime::async_trait]
    pub trait Socket: Sized + Send {
        type Socket: std::fmt::Debug + Send + Sync;
        async fn socket_tcp_bind(
            &mut self,
            wasi_ctx: &mut wasmtime_wasi::WasiCtx,
            endpoint: &str,
            options: TcpBindOptions,
        ) -> Result<RawFd, Error>;

        async fn socket_tcp_connect(
            &mut self,
            wasi_ctx: &mut wasmtime_wasi::WasiCtx,
            remote_endpoint: &str,
            options: TcpConnectOptions<'_>,
        ) -> Result<RawFd, Error>;

        async fn socket_shutdown(
            &mut self,
            wasi_ctx: &mut wasmtime_wasi::WasiCtx,
            fd: RawFd,
            opt: ShutdownOption,
        ) -> Result<(), Error>;

        async fn socket_get_local_addr(
            &mut self,
            wasi_ctx: &mut wasmtime_wasi::WasiCtx,
            fd: RawFd,
        ) -> Result<String, Error>;

        async fn socket_get_peer_addr(
            &mut self,
            wasi_ctx: &mut wasmtime_wasi::WasiCtx,
            fd: RawFd,
        ) -> Result<String, Error>;

        async fn socket_get_ttl(
            &mut self,
            wasi_ctx: &mut wasmtime_wasi::WasiCtx,
            fd: RawFd,
        ) -> Result<u32, Error>;

        async fn socket_set_ttl(
            &mut self,
            wasi_ctx: &mut wasmtime_wasi::WasiCtx,
            fd: RawFd,
            ttl: u32,
        ) -> Result<(), Error>;

        async fn socket_get_nodelay(
            &mut self,
            wasi_ctx: &mut wasmtime_wasi::WasiCtx,
            fd: RawFd,
        ) -> Result<bool, Error>;

        async fn socket_set_nodelay(
            &mut self,
            wasi_ctx: &mut wasmtime_wasi::WasiCtx,
            fd: RawFd,
            nodelay: bool,
        ) -> Result<(), Error>;

        fn drop_socket(&mut self, state: Self::Socket) {
            drop(state);
        }
    }

    pub struct SocketTables<T: Socket> {
        pub(crate) socket_table: wit_bindgen_wasmtime::Table<T::Socket>,
    }
    impl<T: Socket> Default for SocketTables<T> {
        fn default() -> Self {
            Self {
                socket_table: Default::default(),
            }
        }
    }
    pub fn add_to_linker<T, U>(
        linker: &mut wasmtime::Linker<T>,
        get: impl Fn(&mut T) -> (&mut U, &mut wasmtime_wasi::WasiCtx, &mut SocketTables<U>)
            + Send
            + Sync
            + Copy
            + 'static,
    ) -> anyhow::Result<()>
    where
        U: Socket,
        T: Send,
    {
        use wit_bindgen_wasmtime::rt::get_func;
        use wit_bindgen_wasmtime::rt::get_memory;
        linker.func_wrap6_async(
            "socket",
            "socket::tcp-bind",
            move |mut caller: wasmtime::Caller<'_, T>,
                  arg0: i32,
                  arg1: i32,
                  arg2: i32,
                  arg3: i32,
                  arg4: i32,
                  arg5: i32| {
                Box::new(async move {
                    let func = get_func(&mut caller, "canonical_abi_realloc")?;
                    let func_canonical_abi_realloc =
                        func.typed::<(i32, i32, i32, i32), i32, _>(&caller)?;
                    let memory = &get_memory(&mut caller, "memory")?;
                    let (mem, data) = memory.data_and_store_mut(&mut caller);
                    let mut _bc = wit_bindgen_wasmtime::BorrowChecker::new(mem);
                    let host = get(data);
                    let (host, wasi_ctx, _tables) = host;
                    let ptr0 = arg0;
                    let len0 = arg1;
                    let param0 = _bc.slice_str(ptr0, len0)?;
                    let param1 = TcpBindOptions {
                        backlog: arg2 as u32,
                        nonblocking: match arg3 {
                            0 => false,
                            1 => true,
                            _ => return Err(invalid_variant("bool")),
                        },
                        reuse_address: match arg4 {
                            0 => false,
                            1 => true,
                            _ => return Err(invalid_variant("bool")),
                        },
                    };
                    let result = host.socket_tcp_bind(wasi_ctx, param0, param1).await;
                    match result {
                        Ok(e) => {
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory
                                .store(arg5 + 0, wit_bindgen_wasmtime::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(
                                arg5 + 4,
                                wit_bindgen_wasmtime::rt::as_i32(wit_bindgen_wasmtime::rt::as_i32(
                                    e,
                                )),
                            )?;
                        }
                        Err(e) => {
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory
                                .store(arg5 + 0, wit_bindgen_wasmtime::rt::as_i32(1i32) as u8)?;
                            match e {
                                Error::ErrorWithDescription(e) => {
                                    caller_memory.store(
                                        arg5 + 4,
                                        wit_bindgen_wasmtime::rt::as_i32(0i32) as u8,
                                    )?;
                                    let vec1 = e;
                                    let ptr1 = func_canonical_abi_realloc
                                        .call_async(&mut caller, (0, 0, 1, vec1.len() as i32))
                                        .await?;
                                    let (caller_memory, data) =
                                        memory.data_and_store_mut(&mut caller);
                                    let (_, _wasi_ctx, _tables) = get(data);
                                    caller_memory.store_many(ptr1, vec1.as_bytes())?;
                                    caller_memory.store(
                                        arg5 + 12,
                                        wit_bindgen_wasmtime::rt::as_i32(vec1.len() as i32),
                                    )?;
                                    caller_memory
                                        .store(arg5 + 8, wit_bindgen_wasmtime::rt::as_i32(ptr1))?;
                                }
                            };
                        }
                    };
                    Ok(())
                })
            },
        )?;
        linker.func_wrap7_async(
            "socket",
            "socket::tcp-connect",
            move |mut caller: wasmtime::Caller<'_, T>,
                  arg0: i32,
                  arg1: i32,
                  arg2: i32,
                  arg3: i32,
                  arg4: i32,
                  arg5: i32,
                  arg6: i32| {
                Box::new(async move {
                    let func = get_func(&mut caller, "canonical_abi_realloc")?;
                    let func_canonical_abi_realloc =
                        func.typed::<(i32, i32, i32, i32), i32, _>(&caller)?;
                    let memory = &get_memory(&mut caller, "memory")?;
                    let (mem, data) = memory.data_and_store_mut(&mut caller);
                    let mut _bc = wit_bindgen_wasmtime::BorrowChecker::new(mem);
                    let host = get(data);
                    let (host, wasi_ctx, _tables) = host;
                    let ptr0 = arg0;
                    let len0 = arg1;
                    let ptr1 = arg2;
                    let len1 = arg3;
                    let param0 = _bc.slice_str(ptr0, len0)?;
                    let param1 = TcpConnectOptions {
                        local_endpoint: _bc.slice_str(ptr1, len1)?,
                        nonblocking: match arg4 {
                            0 => false,
                            1 => true,
                            _ => return Err(invalid_variant("bool")),
                        },
                        connect_timeout_in_ms: arg5 as u32,
                    };
                    let result = host.socket_tcp_connect(wasi_ctx, param0, param1).await;
                    match result {
                        Ok(e) => {
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory
                                .store(arg6 + 0, wit_bindgen_wasmtime::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(
                                arg6 + 4,
                                wit_bindgen_wasmtime::rt::as_i32(wit_bindgen_wasmtime::rt::as_i32(
                                    e,
                                )),
                            )?;
                        }
                        Err(e) => {
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory
                                .store(arg6 + 0, wit_bindgen_wasmtime::rt::as_i32(1i32) as u8)?;
                            match e {
                                Error::ErrorWithDescription(e) => {
                                    caller_memory.store(
                                        arg6 + 4,
                                        wit_bindgen_wasmtime::rt::as_i32(0i32) as u8,
                                    )?;
                                    let vec2 = e;
                                    let ptr2 = func_canonical_abi_realloc
                                        .call_async(&mut caller, (0, 0, 1, vec2.len() as i32))
                                        .await?;
                                    let (caller_memory, data) =
                                        memory.data_and_store_mut(&mut caller);
                                    let (_, _wasi_ctx, _tables) = get(data);
                                    caller_memory.store_many(ptr2, vec2.as_bytes())?;
                                    caller_memory.store(
                                        arg6 + 12,
                                        wit_bindgen_wasmtime::rt::as_i32(vec2.len() as i32),
                                    )?;
                                    caller_memory
                                        .store(arg6 + 8, wit_bindgen_wasmtime::rt::as_i32(ptr2))?;
                                }
                            };
                        }
                    };
                    Ok(())
                })
            },
        )?;
        linker.func_wrap3_async(
            "socket",
            "socket::shutdown",
            move |mut caller: wasmtime::Caller<'_, T>, arg0: i32, arg1: i32, arg2: i32| {
                Box::new(async move {
                    let func = get_func(&mut caller, "canonical_abi_realloc")?;
                    let func_canonical_abi_realloc =
                        func.typed::<(i32, i32, i32, i32), i32, _>(&caller)?;
                    let memory = &get_memory(&mut caller, "memory")?;
                    let host = get(caller.data_mut());
                    let (host, wasi_ctx, _tables) = host;
                    let param0 = arg0;
                    let param1 = match arg1 {
                        0 => ShutdownOption::Read,
                        1 => ShutdownOption::Write,
                        2 => ShutdownOption::Both,
                        _ => return Err(invalid_variant("ShutdownOption")),
                    };
                    let result = host.socket_shutdown(wasi_ctx, param0, param1).await;
                    match result {
                        Ok(e) => {
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmtime::rt::as_i32(0i32) as u8)?;
                            let () = e;
                        }
                        Err(e) => {
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmtime::rt::as_i32(1i32) as u8)?;
                            match e {
                                Error::ErrorWithDescription(e) => {
                                    caller_memory.store(
                                        arg2 + 4,
                                        wit_bindgen_wasmtime::rt::as_i32(0i32) as u8,
                                    )?;
                                    let vec0 = e;
                                    let ptr0 = func_canonical_abi_realloc
                                        .call_async(&mut caller, (0, 0, 1, vec0.len() as i32))
                                        .await?;
                                    let (caller_memory, data) =
                                        memory.data_and_store_mut(&mut caller);
                                    let (_, _wasi_ctx, _tables) = get(data);
                                    caller_memory.store_many(ptr0, vec0.as_bytes())?;
                                    caller_memory.store(
                                        arg2 + 12,
                                        wit_bindgen_wasmtime::rt::as_i32(vec0.len() as i32),
                                    )?;
                                    caller_memory
                                        .store(arg2 + 8, wit_bindgen_wasmtime::rt::as_i32(ptr0))?;
                                }
                            };
                        }
                    };
                    Ok(())
                })
            },
        )?;
        linker.func_wrap2_async(
            "socket",
            "socket::get-local-addr",
            move |mut caller: wasmtime::Caller<'_, T>, arg0: i32, arg1: i32| {
                Box::new(async move {
                    let func = get_func(&mut caller, "canonical_abi_realloc")?;
                    let func_canonical_abi_realloc =
                        func.typed::<(i32, i32, i32, i32), i32, _>(&caller)?;
                    let memory = &get_memory(&mut caller, "memory")?;
                    let host = get(caller.data_mut());
                    let (host, wasi_ctx, _tables) = host;
                    let param0 = arg0;
                    let result = host.socket_get_local_addr(wasi_ctx, param0).await;
                    match result {
                        Ok(e) => {
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory
                                .store(arg1 + 0, wit_bindgen_wasmtime::rt::as_i32(0i32) as u8)?;
                            let vec0 = e;
                            let ptr0 = func_canonical_abi_realloc
                                .call_async(&mut caller, (0, 0, 1, vec0.len() as i32))
                                .await?;
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory.store_many(ptr0, vec0.as_bytes())?;
                            caller_memory.store(
                                arg1 + 8,
                                wit_bindgen_wasmtime::rt::as_i32(vec0.len() as i32),
                            )?;
                            caller_memory
                                .store(arg1 + 4, wit_bindgen_wasmtime::rt::as_i32(ptr0))?;
                        }
                        Err(e) => {
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory
                                .store(arg1 + 0, wit_bindgen_wasmtime::rt::as_i32(1i32) as u8)?;
                            match e {
                                Error::ErrorWithDescription(e) => {
                                    caller_memory.store(
                                        arg1 + 4,
                                        wit_bindgen_wasmtime::rt::as_i32(0i32) as u8,
                                    )?;
                                    let vec1 = e;
                                    let ptr1 = func_canonical_abi_realloc
                                        .call_async(&mut caller, (0, 0, 1, vec1.len() as i32))
                                        .await?;
                                    let (caller_memory, data) =
                                        memory.data_and_store_mut(&mut caller);
                                    let (_, _wasi_ctx, _tables) = get(data);
                                    caller_memory.store_many(ptr1, vec1.as_bytes())?;
                                    caller_memory.store(
                                        arg1 + 12,
                                        wit_bindgen_wasmtime::rt::as_i32(vec1.len() as i32),
                                    )?;
                                    caller_memory
                                        .store(arg1 + 8, wit_bindgen_wasmtime::rt::as_i32(ptr1))?;
                                }
                            };
                        }
                    };
                    Ok(())
                })
            },
        )?;
        linker.func_wrap2_async(
            "socket",
            "socket::get-peer-addr",
            move |mut caller: wasmtime::Caller<'_, T>, arg0: i32, arg1: i32| {
                Box::new(async move {
                    let func = get_func(&mut caller, "canonical_abi_realloc")?;
                    let func_canonical_abi_realloc =
                        func.typed::<(i32, i32, i32, i32), i32, _>(&caller)?;
                    let memory = &get_memory(&mut caller, "memory")?;
                    let host = get(caller.data_mut());
                    let (host, wasi_ctx, _tables) = host;
                    let param0 = arg0;
                    let result = host.socket_get_peer_addr(wasi_ctx, param0).await;
                    match result {
                        Ok(e) => {
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory
                                .store(arg1 + 0, wit_bindgen_wasmtime::rt::as_i32(0i32) as u8)?;
                            let vec0 = e;
                            let ptr0 = func_canonical_abi_realloc
                                .call_async(&mut caller, (0, 0, 1, vec0.len() as i32))
                                .await?;
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory.store_many(ptr0, vec0.as_bytes())?;
                            caller_memory.store(
                                arg1 + 8,
                                wit_bindgen_wasmtime::rt::as_i32(vec0.len() as i32),
                            )?;
                            caller_memory
                                .store(arg1 + 4, wit_bindgen_wasmtime::rt::as_i32(ptr0))?;
                        }
                        Err(e) => {
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory
                                .store(arg1 + 0, wit_bindgen_wasmtime::rt::as_i32(1i32) as u8)?;
                            match e {
                                Error::ErrorWithDescription(e) => {
                                    caller_memory.store(
                                        arg1 + 4,
                                        wit_bindgen_wasmtime::rt::as_i32(0i32) as u8,
                                    )?;
                                    let vec1 = e;
                                    let ptr1 = func_canonical_abi_realloc
                                        .call_async(&mut caller, (0, 0, 1, vec1.len() as i32))
                                        .await?;
                                    let (caller_memory, data) =
                                        memory.data_and_store_mut(&mut caller);
                                    let (_, _wasi_ctx, _tables) = get(data);
                                    caller_memory.store_many(ptr1, vec1.as_bytes())?;
                                    caller_memory.store(
                                        arg1 + 12,
                                        wit_bindgen_wasmtime::rt::as_i32(vec1.len() as i32),
                                    )?;
                                    caller_memory
                                        .store(arg1 + 8, wit_bindgen_wasmtime::rt::as_i32(ptr1))?;
                                }
                            };
                        }
                    };
                    Ok(())
                })
            },
        )?;
        linker.func_wrap2_async(
            "socket",
            "socket::get-ttl",
            move |mut caller: wasmtime::Caller<'_, T>, arg0: i32, arg1: i32| {
                Box::new(async move {
                    let func = get_func(&mut caller, "canonical_abi_realloc")?;
                    let func_canonical_abi_realloc =
                        func.typed::<(i32, i32, i32, i32), i32, _>(&caller)?;
                    let memory = &get_memory(&mut caller, "memory")?;
                    let host = get(caller.data_mut());
                    let (host, wasi_ctx, _tables) = host;
                    let param0 = arg0;
                    let result = host.socket_get_ttl(wasi_ctx, param0).await;
                    match result {
                        Ok(e) => {
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory
                                .store(arg1 + 0, wit_bindgen_wasmtime::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(
                                arg1 + 4,
                                wit_bindgen_wasmtime::rt::as_i32(wit_bindgen_wasmtime::rt::as_i32(
                                    e,
                                )),
                            )?;
                        }
                        Err(e) => {
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory
                                .store(arg1 + 0, wit_bindgen_wasmtime::rt::as_i32(1i32) as u8)?;
                            match e {
                                Error::ErrorWithDescription(e) => {
                                    caller_memory.store(
                                        arg1 + 4,
                                        wit_bindgen_wasmtime::rt::as_i32(0i32) as u8,
                                    )?;
                                    let vec0 = e;
                                    let ptr0 = func_canonical_abi_realloc
                                        .call_async(&mut caller, (0, 0, 1, vec0.len() as i32))
                                        .await?;
                                    let (caller_memory, data) =
                                        memory.data_and_store_mut(&mut caller);
                                    let (_, _wasi_ctx, _tables) = get(data);
                                    caller_memory.store_many(ptr0, vec0.as_bytes())?;
                                    caller_memory.store(
                                        arg1 + 12,
                                        wit_bindgen_wasmtime::rt::as_i32(vec0.len() as i32),
                                    )?;
                                    caller_memory
                                        .store(arg1 + 8, wit_bindgen_wasmtime::rt::as_i32(ptr0))?;
                                }
                            };
                        }
                    };
                    Ok(())
                })
            },
        )?;
        linker.func_wrap3_async(
            "socket",
            "socket::set-ttl",
            move |mut caller: wasmtime::Caller<'_, T>, arg0: i32, arg1: i32, arg2: i32| {
                Box::new(async move {
                    let func = get_func(&mut caller, "canonical_abi_realloc")?;
                    let func_canonical_abi_realloc =
                        func.typed::<(i32, i32, i32, i32), i32, _>(&caller)?;
                    let memory = &get_memory(&mut caller, "memory")?;
                    let host = get(caller.data_mut());
                    let (host, wasi_ctx, _tables) = host;
                    let param0 = arg0;
                    let param1 = arg1 as u32;
                    let result = host.socket_set_ttl(wasi_ctx, param0, param1).await;
                    match result {
                        Ok(e) => {
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmtime::rt::as_i32(0i32) as u8)?;
                            let () = e;
                        }
                        Err(e) => {
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmtime::rt::as_i32(1i32) as u8)?;
                            match e {
                                Error::ErrorWithDescription(e) => {
                                    caller_memory.store(
                                        arg2 + 4,
                                        wit_bindgen_wasmtime::rt::as_i32(0i32) as u8,
                                    )?;
                                    let vec0 = e;
                                    let ptr0 = func_canonical_abi_realloc
                                        .call_async(&mut caller, (0, 0, 1, vec0.len() as i32))
                                        .await?;
                                    let (caller_memory, data) =
                                        memory.data_and_store_mut(&mut caller);
                                    let (_, _wasi_ctx, _tables) = get(data);
                                    caller_memory.store_many(ptr0, vec0.as_bytes())?;
                                    caller_memory.store(
                                        arg2 + 12,
                                        wit_bindgen_wasmtime::rt::as_i32(vec0.len() as i32),
                                    )?;
                                    caller_memory
                                        .store(arg2 + 8, wit_bindgen_wasmtime::rt::as_i32(ptr0))?;
                                }
                            };
                        }
                    };
                    Ok(())
                })
            },
        )?;
        linker.func_wrap2_async(
            "socket",
            "socket::get-nodelay",
            move |mut caller: wasmtime::Caller<'_, T>, arg0: i32, arg1: i32| {
                Box::new(async move {
                    let func = get_func(&mut caller, "canonical_abi_realloc")?;
                    let func_canonical_abi_realloc =
                        func.typed::<(i32, i32, i32, i32), i32, _>(&caller)?;
                    let memory = &get_memory(&mut caller, "memory")?;
                    let host = get(caller.data_mut());
                    let (host, wasi_ctx, _tables) = host;
                    let param0 = arg0;
                    let result = host.socket_get_nodelay(wasi_ctx, param0).await;
                    match result {
                        Ok(e) => {
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory
                                .store(arg1 + 0, wit_bindgen_wasmtime::rt::as_i32(0i32) as u8)?;
                            caller_memory.store(
                                arg1 + 4,
                                wit_bindgen_wasmtime::rt::as_i32(match e {
                                    true => 1,
                                    false => 0,
                                }) as u8,
                            )?;
                        }
                        Err(e) => {
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory
                                .store(arg1 + 0, wit_bindgen_wasmtime::rt::as_i32(1i32) as u8)?;
                            match e {
                                Error::ErrorWithDescription(e) => {
                                    caller_memory.store(
                                        arg1 + 4,
                                        wit_bindgen_wasmtime::rt::as_i32(0i32) as u8,
                                    )?;
                                    let vec0 = e;
                                    let ptr0 = func_canonical_abi_realloc
                                        .call_async(&mut caller, (0, 0, 1, vec0.len() as i32))
                                        .await?;
                                    let (caller_memory, data) =
                                        memory.data_and_store_mut(&mut caller);
                                    let (_, _wasi_ctx, _tables) = get(data);
                                    caller_memory.store_many(ptr0, vec0.as_bytes())?;
                                    caller_memory.store(
                                        arg1 + 12,
                                        wit_bindgen_wasmtime::rt::as_i32(vec0.len() as i32),
                                    )?;
                                    caller_memory
                                        .store(arg1 + 8, wit_bindgen_wasmtime::rt::as_i32(ptr0))?;
                                }
                            };
                        }
                    };
                    Ok(())
                })
            },
        )?;
        linker.func_wrap3_async(
            "socket",
            "socket::set-nodelay",
            move |mut caller: wasmtime::Caller<'_, T>, arg0: i32, arg1: i32, arg2: i32| {
                Box::new(async move {
                    let func = get_func(&mut caller, "canonical_abi_realloc")?;
                    let func_canonical_abi_realloc =
                        func.typed::<(i32, i32, i32, i32), i32, _>(&caller)?;
                    let memory = &get_memory(&mut caller, "memory")?;
                    let host = get(caller.data_mut());
                    let (host, wasi_ctx, _tables) = host;
                    let param0 = arg0;
                    let param1 = match arg1 {
                        0 => false,
                        1 => true,
                        _ => return Err(invalid_variant("bool")),
                    };
                    let result = host.socket_set_nodelay(wasi_ctx, param0, param1).await;
                    match result {
                        Ok(e) => {
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmtime::rt::as_i32(0i32) as u8)?;
                            let () = e;
                        }
                        Err(e) => {
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmtime::rt::as_i32(1i32) as u8)?;
                            match e {
                                Error::ErrorWithDescription(e) => {
                                    caller_memory.store(
                                        arg2 + 4,
                                        wit_bindgen_wasmtime::rt::as_i32(0i32) as u8,
                                    )?;
                                    let vec0 = e;
                                    let ptr0 = func_canonical_abi_realloc
                                        .call_async(&mut caller, (0, 0, 1, vec0.len() as i32))
                                        .await?;
                                    let (caller_memory, data) =
                                        memory.data_and_store_mut(&mut caller);
                                    let (_, _wasi_ctx, _tables) = get(data);
                                    caller_memory.store_many(ptr0, vec0.as_bytes())?;
                                    caller_memory.store(
                                        arg2 + 12,
                                        wit_bindgen_wasmtime::rt::as_i32(vec0.len() as i32),
                                    )?;
                                    caller_memory
                                        .store(arg2 + 8, wit_bindgen_wasmtime::rt::as_i32(ptr0))?;
                                }
                            };
                        }
                    };
                    Ok(())
                })
            },
        )?;
        linker.func_wrap(
            "canonical_abi",
            "resource_drop_socket",
            move |mut caller: wasmtime::Caller<'_, T>, handle: u32| {
                let (host, _wasi_ctx, tables) = get(caller.data_mut());
                let handle = tables
                    .socket_table
                    .remove(handle)
                    .map_err(|e| wasmtime::Trap::new(format!("failed to remove handle: {}", e)))?;
                host.drop_socket(handle);
                Ok(())
            },
        )?;
        Ok(())
    }
    use wit_bindgen_wasmtime::rt::invalid_variant;
    use wit_bindgen_wasmtime::rt::RawMem;
}
