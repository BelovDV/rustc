#![feature(rustc_private)]

use std::fs::File;
use std::os::fd::AsRawFd as _;
use std::os::unix::process::CommandExt as _;
use std::process::Command;

extern crate libc;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    test(
        &args,
        |cmd| {
            cmd.env("MAKEFLAGS", "--jobserver-auth=");
        },
        b"error: Cannot access jobserver: PlatformSpecific { err: ParseEnvVar, env: \"MAKEFLAGS\", var: \"--jobserver-auth=\" }\n\n",
    );
    test(
        &args,
        |cmd| {
            cmd.env("MAKEFLAGS", "--jobserver-auth=100,100");
        },
        b"error: Cannot access jobserver: PlatformSpecific { err: InvalidDescriptor(100, 100), env: \"MAKEFLAGS\", var: \"--jobserver-auth=100,100\" }\n\n",
    );
    test_wrong_pipe(get_compiler(&args));
}

fn get_compiler(args: &Vec<String>) -> Command {
    let mut cmd = Command::new(&args[1]);
    cmd.args(["--out-dir", &args[2]]);
    cmd.arg(&args[3]);
    cmd
}

fn test(args: &Vec<String>, f: fn(&mut Command), err: &[u8]) {
    let mut cmd = get_compiler(args);
    f(&mut cmd);
    dbg!(std::str::from_utf8(&cmd.output().unwrap().stderr).unwrap());
    assert!(cmd.output().unwrap().stderr == err);
}

fn test_wrong_pipe(mut cmd: Command) {
    let file = File::open("/dev/null").unwrap();
    let fd = file.as_raw_fd();
    unsafe {
        cmd.pre_exec(move || {
            libc::fcntl(fd, libc::F_SETFD, 0);
            Ok(())
        });
    }

    cmd.env("MAKEFLAGS", format!("--jobserver-auth={},{}", fd, fd));

    dbg!(std::str::from_utf8(&cmd.output().unwrap().stderr).unwrap());

    // let f_out = File::open("/dev/null").unwrap();
    // let fd_out = f_out.as_raw_fd() as i32;
    // cmd.env("MAKEFLAGS", format!("--jobserver-auth={},{}", fd_empty, fd_out));
    // // cmd.env("MAKEFLAGS", format!("--jobserver-auth={},{}", 0, 1));
    // let output = cmd.output().unwrap();
    // dbg!(std::str::from_utf8(&output.stderr).unwrap());
    // assert_eq!(output.stderr, b"error: Cannot access jobserver: InvalidStream(100, 100)\n\n");
    assert!(
        cmd.output().unwrap().stderr
            == b"error: failed to acquire jobserver token: early EOF on jobserver pipe\
            \n\nerror: aborting due to previous error\n\n"
    );
}
