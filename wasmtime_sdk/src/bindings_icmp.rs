#[allow(clippy::all)]
pub mod common {
    #[allow(unused_imports)]
    use wit_bindgen_wasmtime::{anyhow, wasmtime};
}
#[allow(clippy::all)]
pub mod icmp {
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
    #[wit_bindgen_wasmtime::async_trait]
    pub trait Icmp: Sized + Send {
        type Icmp: std::fmt::Debug + Send + Sync;
        async fn icmp_ping(
            &mut self,
            wasi_ctx: &mut wasmtime_wasi::WasiCtx,
            target_ip: &str,
        ) -> Result<IcmpPingResult, Error>;

        async fn icmp_ping_with_options(
            &mut self,
            wasi_ctx: &mut wasmtime_wasi::WasiCtx,
            target_ip: &str,
            source_ip: &str,
            identifier: u16,
            ttl: u8,
            seq: u16,
        ) -> Result<IcmpPingResult, Error>;

        fn drop_icmp(&mut self, state: Self::Icmp) {
            drop(state);
        }
    }

    pub struct IcmpTables<T: Icmp> {
        pub(crate) icmp_table: wit_bindgen_wasmtime::Table<T::Icmp>,
    }
    impl<T: Icmp> Default for IcmpTables<T> {
        fn default() -> Self {
            Self {
                icmp_table: Default::default(),
            }
        }
    }
    pub fn add_to_linker<T, U>(
        linker: &mut wasmtime::Linker<T>,
        get: impl Fn(&mut T) -> (&mut U, &mut wasmtime_wasi::WasiCtx, &mut IcmpTables<U>)
            + Send
            + Sync
            + Copy
            + 'static,
    ) -> anyhow::Result<()>
    where
        U: Icmp,
        T: Send,
    {
        use wit_bindgen_wasmtime::rt::get_func;
        use wit_bindgen_wasmtime::rt::get_memory;
        linker.func_wrap3_async(
            "icmp",
            "icmp::ping",
            move |mut caller: wasmtime::Caller<'_, T>, arg0: i32, arg1: i32, arg2: i32| {
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
                    let result = host.icmp_ping(wasi_ctx, param0).await;
                    match result {
                        Ok(e) => {
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmtime::rt::as_i32(0i32) as u8)?;
                            let IcmpPingResult {
                                src_ip: src_ip1,
                                dst_ip: dst_ip1,
                                icmp_code: icmp_code1,
                                identifier: identifier1,
                                seq: seq1,
                                ttl: ttl1,
                                packet_size: packet_size1,
                                duration_in_microseconds: duration_in_microseconds1,
                            } = e;
                            let vec2 = src_ip1;
                            let ptr2 = func_canonical_abi_realloc
                                .call_async(&mut caller, (0, 0, 1, vec2.len() as i32))
                                .await?;
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory.store_many(ptr2, vec2.as_bytes())?;
                            caller_memory.store(
                                arg2 + 12,
                                wit_bindgen_wasmtime::rt::as_i32(vec2.len() as i32),
                            )?;
                            caller_memory
                                .store(arg2 + 8, wit_bindgen_wasmtime::rt::as_i32(ptr2))?;
                            let vec3 = dst_ip1;
                            let ptr3 = func_canonical_abi_realloc
                                .call_async(&mut caller, (0, 0, 1, vec3.len() as i32))
                                .await?;
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory.store_many(ptr3, vec3.as_bytes())?;
                            caller_memory.store(
                                arg2 + 20,
                                wit_bindgen_wasmtime::rt::as_i32(vec3.len() as i32),
                            )?;
                            caller_memory
                                .store(arg2 + 16, wit_bindgen_wasmtime::rt::as_i32(ptr3))?;
                            caller_memory.store(
                                arg2 + 24,
                                wit_bindgen_wasmtime::rt::as_i32(wit_bindgen_wasmtime::rt::as_i32(
                                    icmp_code1,
                                )) as u8,
                            )?;
                            caller_memory.store(
                                arg2 + 26,
                                wit_bindgen_wasmtime::rt::as_i32(wit_bindgen_wasmtime::rt::as_i32(
                                    identifier1,
                                )) as u16,
                            )?;
                            caller_memory.store(
                                arg2 + 28,
                                wit_bindgen_wasmtime::rt::as_i32(wit_bindgen_wasmtime::rt::as_i32(
                                    seq1,
                                )) as u16,
                            )?;
                            caller_memory.store(
                                arg2 + 30,
                                wit_bindgen_wasmtime::rt::as_i32(wit_bindgen_wasmtime::rt::as_i32(
                                    ttl1,
                                )) as u8,
                            )?;
                            caller_memory.store(
                                arg2 + 32,
                                wit_bindgen_wasmtime::rt::as_i64(wit_bindgen_wasmtime::rt::as_i64(
                                    packet_size1,
                                )),
                            )?;
                            caller_memory.store(
                                arg2 + 40,
                                wit_bindgen_wasmtime::rt::as_i64(wit_bindgen_wasmtime::rt::as_i64(
                                    duration_in_microseconds1,
                                )),
                            )?;
                        }
                        Err(e) => {
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory
                                .store(arg2 + 0, wit_bindgen_wasmtime::rt::as_i32(1i32) as u8)?;
                            match e {
                                Error::ErrorWithDescription(e) => {
                                    caller_memory.store(
                                        arg2 + 8,
                                        wit_bindgen_wasmtime::rt::as_i32(0i32) as u8,
                                    )?;
                                    let vec4 = e;
                                    let ptr4 = func_canonical_abi_realloc
                                        .call_async(&mut caller, (0, 0, 1, vec4.len() as i32))
                                        .await?;
                                    let (caller_memory, data) =
                                        memory.data_and_store_mut(&mut caller);
                                    let (_, _wasi_ctx, _tables) = get(data);
                                    caller_memory.store_many(ptr4, vec4.as_bytes())?;
                                    caller_memory.store(
                                        arg2 + 16,
                                        wit_bindgen_wasmtime::rt::as_i32(vec4.len() as i32),
                                    )?;
                                    caller_memory
                                        .store(arg2 + 12, wit_bindgen_wasmtime::rt::as_i32(ptr4))?;
                                }
                            };
                        }
                    };
                    Ok(())
                })
            },
        )?;
        linker.func_wrap8_async(
            "icmp",
            "icmp::ping-with-options",
            move |mut caller: wasmtime::Caller<'_, T>,
                  arg0: i32,
                  arg1: i32,
                  arg2: i32,
                  arg3: i32,
                  arg4: i32,
                  arg5: i32,
                  arg6: i32,
                  arg7: i32| {
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
                    let param1 = _bc.slice_str(ptr1, len1)?;
                    let param2 = u16::try_from(arg4).map_err(bad_int)?;
                    let param3 = u8::try_from(arg5).map_err(bad_int)?;
                    let param4 = u16::try_from(arg6).map_err(bad_int)?;
                    let result = host
                        .icmp_ping_with_options(wasi_ctx, param0, param1, param2, param3, param4)
                        .await;
                    match result {
                        Ok(e) => {
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory
                                .store(arg7 + 0, wit_bindgen_wasmtime::rt::as_i32(0i32) as u8)?;
                            let IcmpPingResult {
                                src_ip: src_ip2,
                                dst_ip: dst_ip2,
                                icmp_code: icmp_code2,
                                identifier: identifier2,
                                seq: seq2,
                                ttl: ttl2,
                                packet_size: packet_size2,
                                duration_in_microseconds: duration_in_microseconds2,
                            } = e;
                            let vec3 = src_ip2;
                            let ptr3 = func_canonical_abi_realloc
                                .call_async(&mut caller, (0, 0, 1, vec3.len() as i32))
                                .await?;
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory.store_many(ptr3, vec3.as_bytes())?;
                            caller_memory.store(
                                arg7 + 12,
                                wit_bindgen_wasmtime::rt::as_i32(vec3.len() as i32),
                            )?;
                            caller_memory
                                .store(arg7 + 8, wit_bindgen_wasmtime::rt::as_i32(ptr3))?;
                            let vec4 = dst_ip2;
                            let ptr4 = func_canonical_abi_realloc
                                .call_async(&mut caller, (0, 0, 1, vec4.len() as i32))
                                .await?;
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory.store_many(ptr4, vec4.as_bytes())?;
                            caller_memory.store(
                                arg7 + 20,
                                wit_bindgen_wasmtime::rt::as_i32(vec4.len() as i32),
                            )?;
                            caller_memory
                                .store(arg7 + 16, wit_bindgen_wasmtime::rt::as_i32(ptr4))?;
                            caller_memory.store(
                                arg7 + 24,
                                wit_bindgen_wasmtime::rt::as_i32(wit_bindgen_wasmtime::rt::as_i32(
                                    icmp_code2,
                                )) as u8,
                            )?;
                            caller_memory.store(
                                arg7 + 26,
                                wit_bindgen_wasmtime::rt::as_i32(wit_bindgen_wasmtime::rt::as_i32(
                                    identifier2,
                                )) as u16,
                            )?;
                            caller_memory.store(
                                arg7 + 28,
                                wit_bindgen_wasmtime::rt::as_i32(wit_bindgen_wasmtime::rt::as_i32(
                                    seq2,
                                )) as u16,
                            )?;
                            caller_memory.store(
                                arg7 + 30,
                                wit_bindgen_wasmtime::rt::as_i32(wit_bindgen_wasmtime::rt::as_i32(
                                    ttl2,
                                )) as u8,
                            )?;
                            caller_memory.store(
                                arg7 + 32,
                                wit_bindgen_wasmtime::rt::as_i64(wit_bindgen_wasmtime::rt::as_i64(
                                    packet_size2,
                                )),
                            )?;
                            caller_memory.store(
                                arg7 + 40,
                                wit_bindgen_wasmtime::rt::as_i64(wit_bindgen_wasmtime::rt::as_i64(
                                    duration_in_microseconds2,
                                )),
                            )?;
                        }
                        Err(e) => {
                            let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                            let (_, _wasi_ctx, _tables) = get(data);
                            caller_memory
                                .store(arg7 + 0, wit_bindgen_wasmtime::rt::as_i32(1i32) as u8)?;
                            match e {
                                Error::ErrorWithDescription(e) => {
                                    caller_memory.store(
                                        arg7 + 8,
                                        wit_bindgen_wasmtime::rt::as_i32(0i32) as u8,
                                    )?;
                                    let vec5 = e;
                                    let ptr5 = func_canonical_abi_realloc
                                        .call_async(&mut caller, (0, 0, 1, vec5.len() as i32))
                                        .await?;
                                    let (caller_memory, data) =
                                        memory.data_and_store_mut(&mut caller);
                                    let (_, _wasi_ctx, _tables) = get(data);
                                    caller_memory.store_many(ptr5, vec5.as_bytes())?;
                                    caller_memory.store(
                                        arg7 + 16,
                                        wit_bindgen_wasmtime::rt::as_i32(vec5.len() as i32),
                                    )?;
                                    caller_memory
                                        .store(arg7 + 12, wit_bindgen_wasmtime::rt::as_i32(ptr5))?;
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
            "resource_drop_icmp",
            move |mut caller: wasmtime::Caller<'_, T>, handle: u32| {
                let (host, _wasi_ctx, tables) = get(caller.data_mut());
                let handle = tables
                    .icmp_table
                    .remove(handle)
                    .map_err(|e| wasmtime::Trap::new(format!("failed to remove handle: {}", e)))?;
                host.drop_icmp(handle);
                Ok(())
            },
        )?;
        Ok(())
    }
    use core::convert::TryFrom;
    use wit_bindgen_wasmtime::rt::bad_int;
    use wit_bindgen_wasmtime::rt::RawMem;
}
