// Unexpected linker modifier should fail with an error

// compile-flags: -l link-arg:+bundle=arg
// error-pattern: linking modifier `bundle` is only compatible with `static` linking kind

fn main() {}
