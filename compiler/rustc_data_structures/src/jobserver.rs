pub use jobserver_crate::Client;
pub use jobserver_crate::ErrFromEnv;

use std::sync::LazyLock;
use std::sync::Mutex;

// We can only call `from_env` once per process

// Note that this is unsafe because it may misinterpret file descriptors
// on Unix as jobserver file descriptors. We hopefully execute this near
// the beginning of the process though to ensure we don't get false
// positives, or in other words we try to execute this before we open
// any file descriptors ourselves.
//
// Pick a "reasonable maximum" if we don't otherwise have
// a jobserver in our environment, capping out at 32 so we
// don't take everything down by hogging the process run queue.
// The fixed number is used to have deterministic compilation
// across machines.
//
// Also note that we stick this in a global because there could be
// multiple rustc instances in this process, and the jobserver is
// per-process.
static GLOBAL_CLIENT: LazyLock<Mutex<(Client, Option<ErrFromEnv>)>> = LazyLock::new(|| unsafe {
    match Client::from_env() {
        Ok(c) => Mutex::new((c, None)),
        Err(ErrFromEnv::IsNotConfigured) => {
            Mutex::new((Client::new(32).expect("failed to create jobserver"), None))
        }
        Err(e) => Mutex::new((Client::new(1).unwrap(), Some(e))),
    }
});

pub fn client() -> Result<Client, ErrFromEnv> {
    let err = std::mem::replace(&mut GLOBAL_CLIENT.lock().unwrap().1, None);
    match err {
        Some(e) => Err(e),
        None => Ok(GLOBAL_CLIENT.lock().unwrap().0.clone()),
    }
}

pub fn acquire_thread() {
    GLOBAL_CLIENT.lock().unwrap().0.acquire_raw().ok();
}

pub fn release_thread() {
    GLOBAL_CLIENT.lock().unwrap().0.release_raw().ok();
}
