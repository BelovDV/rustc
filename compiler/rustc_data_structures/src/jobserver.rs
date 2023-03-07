pub use jobserver_crate::Client;
pub use jobserver_crate::ErrFromEnv;

use std::sync::LazyLock;

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
static GLOBAL_CLIENT: LazyLock<(Client, Option<ErrFromEnv>)> = LazyLock::new(|| unsafe {
    match Client::from_env_ext() {
        Ok(Some(c)) => (c, None),
        Ok(None) => (Client::new(32).expect("failed to create jobserver"), None),
        Err(e) => (Client::new(1).unwrap(), Some(e)),
    }
});

pub fn client() -> Result<Client, ErrFromEnv> {
    match GLOBAL_CLIENT.clone().1 {
        Some(e) => Err(e),
        None => Ok(GLOBAL_CLIENT.0.clone()),
    }
}

pub fn acquire_thread() {
    GLOBAL_CLIENT.0.acquire_raw().ok();
}

pub fn release_thread() {
    GLOBAL_CLIENT.0.release_raw().ok();
}
