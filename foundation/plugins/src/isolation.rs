use crate::handle::{Plugin, PluginHandle};

#[repr(C)]
struct PollableFd {
    fd: i32,
    events: i16,
    revents: i16,
}

const POLLIN: i16 = 0x0001;

unsafe extern "C" {
    fn fork() -> i32;
    fn pipe(fds: *mut i32) -> i32;
    fn dup2(oldfd: i32, newfd: i32) -> i32;
    fn waitpid(pid: i32, status: *mut i32, options: i32) -> i32;
    fn poll(fds: *mut PollableFd, nfds: u32, timeout: i32) -> i32;
    fn read(fd: i32, buf: *mut u8, count: usize) -> isize;
    fn close(fd: i32) -> i32;
    // fn sandbox_init(profile: *const i8, flags: u64, errorbuf: *mut *mut i8) -> i32;
    fn kill(pid: i32, sig: i32) -> i32;
    fn _exit(status: i32) -> !;
    fn write(fd: i32, buf: *const u8, len: usize) -> isize;
}

pub fn spawn(plugin: &impl Plugin) -> Option<Vec<u8>> {
    let mut buf = [0u8; 4096];
    let mut out = Vec::new();
    let mut status: i32 = 0;
    let mut fds = [0i32; 2];
    if unsafe { pipe(fds.as_mut_ptr()) } != 0 {
        return None;
    }

    let pid = unsafe { fork() };
    if pid < 0 {
        unsafe { close(fds[0]); close(fds[1]); }
        return None;
    }
    if pid == 0 {
        unsafe { 
            dup2(fds[1], 1);
            close(fds[0]);
            close(fds[1]);
        }
        let Some(handle) = PluginHandle::load(plugin.path()) else {
            unsafe { _exit(1) }
        };
        let Some(sym) = handle.symbol(plugin.symbol()) else {
             unsafe { _exit(1) }
        };
        let Some(bytes) = (unsafe { plugin.call(sym)}) else {
            unsafe { _exit(1)}
        };
        unsafe {
            let _ = write(1, bytes.as_ptr(), bytes.len());
            _exit(0)
        }

    } else {
        unsafe {
            close(fds[1]);
            let mut pfd = PollableFd { fd: fds[0], events: POLLIN, revents: 0 };
            loop {
                let ready = poll(&mut pfd, 1, 5000);
                if ready <= 0 { break; }
                let n = read(fds[0], buf.as_mut_ptr(), buf.len());
                if n <= 0 { break; }
                out.extend_from_slice(&buf[..n as usize]);
            }
            close(fds[0]);
            kill(pid, 9);
            waitpid(pid, &mut status as *mut i32, 0);
        }
    }
    Some(out)
}