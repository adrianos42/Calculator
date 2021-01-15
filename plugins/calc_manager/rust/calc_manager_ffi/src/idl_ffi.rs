use idl_internal::{ffi::ffi_types::*, StreamSender};
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn method_programmer_commands(
    _instance: *mut super::idl_ffi_impl::ProgrammerInstance,
    value: *const AbiStream,
    _stream: *const AbiStream,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _value_arg_val = {
            let _value = unsafe { (value as *const AbiStream).read() };
            match _value.state.into() {
                AbiStreamSenderState::Waiting => {
                    Box::new(super::idl_ffi_impl::ProgrammerCommands {
                        callback: unsafe { std::mem::transmute(_value.wake_callback) },
                        handle: _value.wake_handle,
                        object: _value.wake_object,
                    })
                }
                _ => {
                    std::mem::forget(_ins);
                    return AbiInternalError::InvalidArg;
                }
            }
        };
        let _stream_val = {
            let _value = unsafe { (_stream as *const AbiStream).read() };
            match _value.state.into() {
                AbiStreamReceiverState::Start => {
                    Box::new(super::idl_ffi_impl::ProgrammerCommands {
                        callback: unsafe { std::mem::transmute(_value.wake_callback) },
                        handle: _value.wake_handle,
                        object: _value.wake_object,
                    })
                }
                _ => {
                    std::mem::forget(_ins);
                    return AbiInternalError::InvalidArg;
                }
            }
        };
        _instance.commands(_value_arg_val, _stream_val);
        std::mem::forget(_ins);
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn stream_programmer_commands(
    _instance: *mut super::idl_ffi_impl::ProgrammerInstance,
    _stream: *const AbiStream,
    _stream_result: *mut *mut AbiStream,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _stream_val = unsafe { (_stream as *const AbiStream).read() };
        let _result = _instance.commands_stream(
            Box::new(super::idl_ffi_impl::ProgrammerCommands {
                callback: unsafe { std::mem::transmute(_stream_val.wake_callback) },
                handle: _stream_val.wake_handle,
                object: _stream_val.wake_object,
            }),
            _stream_val.into(),
        );
        unsafe {
            *_stream_result = { Box::into_raw(Box::new({ _result.into_abi() })) as *mut AbiStream };
        }
        std::mem::forget(_ins);
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn dispose_stream_programmer_commands(
    _instance: *mut super::idl_ffi_impl::ProgrammerInstance,
    _stream: *mut AbiStream,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let _stream_val: Box<AbiStream> = unsafe { Box::from_raw(_stream) };
        _stream_val.dispose();
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn stream_sender_programmer_commands(
    _instance: *mut super::idl_ffi_impl::ProgrammerInstance,
    _stream: *const AbiStream,
    _stream_result: *mut *mut AbiStream,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let mut _ins = unsafe { Box::from_raw(_instance) };
        let mut _instance = &mut _ins.instance;
        let _stream_val = unsafe { (_stream as *const AbiStream).read() };
        let _result = _instance.commands_stream_sender(
            Box::new(super::idl_ffi_impl::ProgrammerCommands {
                callback: unsafe { std::mem::transmute(_stream_val.wake_callback) },
                handle: _stream_val.wake_handle,
                object: _stream_val.wake_object,
            }),
            _stream_val.into_sender(),
        );
        unsafe {
            *_stream_result = { Box::into_raw(Box::new({ _result.into() })) as *mut AbiStream };
        }
        std::mem::forget(_ins);
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn dispose_stream_sender_programmer_commands(
    _instance: *mut super::idl_ffi_impl::ProgrammerInstance,
    _stream: *mut AbiStream,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let _stream_val: Box<AbiStream> = unsafe { Box::from_raw(_stream) };
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn create_programmer(
    _result: *mut *const super::idl_ffi_impl::ProgrammerInstance,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        let _result_val: Box<dyn super::idl_impl::ProgrammerInstance> =
            Box::new(super::idl_ffi_impl::ProgrammerInstance::new());
        unsafe {
            *_result = {
                Box::into_raw(Box::new({
                    super::idl_ffi_impl::ProgrammerInstance::from(_result_val)
                })) as *const super::idl_ffi_impl::ProgrammerInstance
            };
        }
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
#[no_mangle]
#[allow(unused_braces)]
pub extern "C" fn dispose_programmer(
    _result: *mut *const super::idl_ffi_impl::ProgrammerInstance,
) -> i64 {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || {
        return AbiInternalError::Ok;
    })) {
        Ok(value) => value as i64,
        Err(_) => AbiInternalError::UndefinedException as i64,
    }
}
trait StreamSenderIntoAbiStream<T> {
    fn into_abi(self) -> AbiStream;
}
trait StreamAbiSenderDispose<T> {
    fn dispose(self);
}
trait AbiStreamIntoStreamSender<T> {
    fn into_sender(self) -> StreamSender<T>;
}
impl StreamSenderIntoAbiStream<i64> for StreamSender<i64> {
    #[allow(unused_braces)]
    fn into_abi(self) -> AbiStream {
        match self {
            StreamSender::Ok => AbiStream::new(AbiStreamSenderState::Ok as i64),
            StreamSender::Value(value) => {
                let mut _result = AbiStream::new(AbiStreamSenderState::Value as i64);
                _result.data = { Box::into_raw(Box::new({ value } as i64)) as *const i64 }
                    as *const ::core::ffi::c_void;
                _result
            }
            StreamSender::Request => AbiStream::new(AbiStreamSenderState::Request as i64),
            StreamSender::Waiting => AbiStream::new(AbiStreamSenderState::Waiting as i64),
            StreamSender::Done => AbiStream::new(AbiStreamSenderState::Done as i64),
        }
    }
}
impl StreamAbiSenderDispose<i64> for AbiStream {
    fn dispose(self) {
        match self.state.into() {
            AbiStreamSenderState::Value => {}
            _ => {}
        }
    }
}
impl AbiStreamIntoStreamSender<i64> for AbiStream {
    #[allow(unused_braces)]
    fn into_sender(self) -> StreamSender<i64> {
        match self.state.into() {
            AbiStreamSenderState::Ok => StreamSender::Ok,
            AbiStreamSenderState::Value => {
                StreamSender::Value({ unsafe { (self.data as *const i64).read() } } as i64)
            }
            AbiStreamSenderState::Request => StreamSender::Request,
            AbiStreamSenderState::Waiting => StreamSender::Waiting,
            AbiStreamSenderState::Done => StreamSender::Done,
        }
    }
}
