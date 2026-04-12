extern crate alloc;

#[cfg(target_os = "macos")]
unsafe extern "C" {
    fn dlopen(path: *const i8, flags: i32) -> *mut core::ffi::c_void;
    fn dlsym(handle: *mut core::ffi::c_void, symbol: *const i8) -> *mut core::ffi::c_void;
    fn dlclose(handle: *mut core::ffi::c_void) -> i32;
}

pub trait Plugin {
    fn path(&self) -> &str;
    fn symbol(&self) -> &'static core::ffi::CStr;
    /// # Safety
    /// The caller must ensure that `sym` is a valid function pointer obtained from `dlsym`.
    unsafe fn call(&self, sym: *mut core::ffi::c_void) -> Option<Vec<u8>>;
}

  pub(crate) struct PluginHandle {
    handle: *mut core::ffi::c_void,
}

impl PluginHandle {
    #[cfg(target_os = "macos")]
    /// load a `.dylib/.so` into the process at runtime
    pub(crate) fn load(path: &str) -> Option<Self> {
        let path = alloc::ffi::CString::new(path).ok()?;
        let handle = unsafe { dlopen(path.as_ptr(), 1) };
        if handle.is_null() { return None; }
        Some(Self { handle })
    }

    #[cfg(target_os = "macos")]
    /// lookup symbols by name with `dlsym`
    pub(crate) fn symbol(&self, symbol: &'static core::ffi::CStr) -> Option<*mut core::ffi::c_void> {
        let sym = unsafe { dlsym(self.handle, symbol.as_ptr())};
        if sym.is_null() { return None; }
        Some(sym)
    }
}

impl Drop for PluginHandle {
    #[cfg(target_os = "macos")]
    fn drop(&mut self) {
        unsafe { dlclose(self.handle); }
    }
}