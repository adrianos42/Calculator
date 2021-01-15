use calc_manager_types::idl_impl;
use idl_internal::ffi::ffi_types::*;
use idl_internal::*;
pub(super) struct ProgrammerCommands {
    pub(super) callback: extern "C" fn(i64, *const ::core::ffi::c_void),
    pub(super) object: *const ::core::ffi::c_void,
    pub(super) handle: i64,
}
impl StreamInstance for ProgrammerCommands {
    fn wake_client(&self) {
        let run = self.callback;
        run(self.handle, self.object);
    }
    fn get_handle(&self) -> i64 {
        self.handle
    }
}
pub struct ProgrammerInstance {
    pub(super) instance: Box<dyn idl_impl::ProgrammerInstance>,
}
impl ProgrammerInstance {
    pub(super) fn new() -> Self {
        calc_manager_impl::create_programmer_instance()
    }
}
impl From<Box<dyn idl_impl::ProgrammerInstance>> for ProgrammerInstance {
    fn from(value: Box<dyn idl_impl::ProgrammerInstance>) -> Self {
        Self { instance: value }
    }
}
