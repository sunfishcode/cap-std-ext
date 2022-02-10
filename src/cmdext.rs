//! Extensions for [`std::process::Command`].

use rustix::fd::{FromRawFd, IntoRawFd};
use rustix::io::OwnedFd;
use std::os::unix::process::CommandExt;
use std::sync::Arc;

/// Extension trait for [`std::process::Command`].
pub trait CapStdExtCommandExt {
    /// Pass a file descriptor into the target process.
    fn take_fd_n(&mut self, fd: Arc<OwnedFd>, target: i32) -> &mut Self;
}

#[allow(unsafe_code)]
impl CapStdExtCommandExt for std::process::Command {
    fn take_fd_n(&mut self, fd: Arc<OwnedFd>, target: i32) -> &mut Self {
        unsafe {
            self.pre_exec(move || {
                let target = rustix::io::OwnedFd::from_raw_fd(target);
                dbg!("hi");
                dbg!(rustix::io::dup2(&*fd, &target))?;
                dbg!("hi");
                // Intentionally leak into the child.
                let _ = target.into_raw_fd();
                Ok(())
            });
        }
        self
    }
}
