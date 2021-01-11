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
pub(super) struct ProgrammerCommandss {
    pub(super) callback: extern "C" fn(i64, *const ::core::ffi::c_void),
    pub(super) object: *const ::core::ffi::c_void,
    pub(super) handle: i64,
}
impl StreamInstance for ProgrammerCommandss {
    fn wake_client(&self) {
        let run = self.callback;
        run(self.handle, self.object);
    }
    fn get_handle(&self) -> i64 {
        self.handle
    }
}
pub struct ProgrammerInstance {
    pub(super) instance: Box<dyn super::idl_impl::ProgrammerInstance>,
}
impl From<Box<dyn super::idl_impl::ProgrammerInstance>> for ProgrammerInstance {
    fn from(value: Box<dyn super::idl_impl::ProgrammerInstance>) -> Self {
        Self { instance: value }
    }
}
pub struct ProgrammerStatic {
    pub(super) instance: Box<dyn super::idl_impl::ProgrammerStatic + Sync>,
}
impl ProgrammerStatic {
    pub(super) fn get_instance<F: FnOnce() -> Box<dyn super::idl_impl::ProgrammerStatic>>(
        fn_init: F,
    ) -> &'static ProgrammerStatic {
        
    }
}

lazy_static! {
    static ref INSTANCE_PROGRAMMER: ProgrammerStatic = ProgrammerStatic {
        instance: Box::new(super::impl_calc_manager::Programmer::new()),
    };
}