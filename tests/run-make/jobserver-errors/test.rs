use std::process::Command;
use std::fs::File;
// use std::io::Read as _;
use std::os::fd::AsRawFd as _;
use std::path::Path;


fn main() {
    let args: Vec<_> = std::env::args().collect();
    let compiler = &args[1];
    let out_dir = &args[2];
    let code = &args[3];
    dbg!(compiler, out_dir, code);

    let f_empty = File::open(Path::new(out_dir).join("empty")).unwrap();
    let fd_empty = f_empty.as_raw_fd() as i32;

    test_wrong_var(get_compiler(compiler, out_dir, code));
    test_no_pipe(get_compiler(compiler, out_dir, code));
    test_wrong_pipe(get_compiler(compiler, out_dir, code), fd_empty);
}

fn get_compiler(compiler: &str, out_dir: &str, code: &str) -> Command {
    let mut cmd = Command::new(compiler);
    cmd.args(["--out-dir", out_dir]);
    cmd.arg(code);
    cmd
}

fn test_wrong_var(mut cmd: Command) {
    cmd.env("MAKEFLAGS", "--jobserver-auth=");
    let output = cmd.output().unwrap();
    assert_eq!(output.stderr, b"error: Cannot access jobserver: ParseEnvVar(\"\")\n\n");
}

fn test_no_pipe(mut cmd: Command) {
    cmd.env("MAKEFLAGS", "--jobserver-auth=100,100");
    let output = cmd.output().unwrap();
    assert_eq!(output.stderr, b"error: Cannot access jobserver: InvalidStream(100, 100)\n\n");
}

fn test_wrong_pipe(mut _cmd: Command, _fd_empty: i32) {
    // let f_out = File::open("/dev/null").unwrap();
    // let fd_out = f_out.as_raw_fd() as i32;
    // cmd.env("MAKEFLAGS", format!("--jobserver-auth={},{}", fd_empty, fd_out));
    // // cmd.env("MAKEFLAGS", format!("--jobserver-auth={},{}", 0, 1));
    // let output = cmd.output().unwrap();
    // dbg!(std::str::from_utf8(&output.stderr).unwrap());
    // assert_eq!(output.stderr, b"error: Cannot access jobserver: InvalidStream(100, 100)\n\n");
}
