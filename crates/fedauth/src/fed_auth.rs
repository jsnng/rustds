use plugins::handle::Plugin;
use plugins::isolation::spawn;

pub struct FedAuth {
    pub path: String,
    pub sts_url: String,
    pub spn: String,
    pub nonce: Option<[u8; 32]>,
}

impl FedAuth {
    pub fn acquire(&self) -> Option<Vec<u8>> {
        spawn(self)
    }
}

impl Plugin for FedAuth {
    fn path(&self) -> &str {
        &self.path
    }

    fn symbol(&self) -> &'static core::ffi::CStr {
        c"acquire"
    }

    unsafe fn call(&self, sym: *mut core::ffi::c_void) -> Option<Vec<u8>> {
        type AcquireTokenFn = unsafe extern "C" fn(
            sts_url: *const u8,
            sts_url_len: usize,
            spn: *const u8,
            spn_len: usize,
            nonce: *const u8,
            nonce_len: usize,
            out_len: *mut usize,
        ) -> *mut u8;

        unsafe {
            let f: AcquireTokenFn = core::mem::transmute(sym);
            let mut out_len: usize = 0;
            let (nonce_ptr, nonce_len) = match &self.nonce {
                Some(n) => (n.as_ptr(), 32),
                None => (core::ptr::null(), 0),
            };
            let ptr = f(
                self.sts_url.as_ptr(), self.sts_url.len(),
                self.spn.as_ptr(), self.spn.len(),
                nonce_ptr, nonce_len,
                &mut out_len,
            );
            if ptr.is_null() { return None; }
            Some(core::slice::from_raw_parts(ptr, out_len).to_vec())
        }
    }
}