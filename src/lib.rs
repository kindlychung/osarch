pub use regex::Regex;

pub fn current_os_arch() -> Regex {
    let os = std::env::consts::OS;
    let mut arch = std::env::consts::ARCH;
    if arch == "x86_64" {
        arch = "(amd64|x86_64)";
    } else if arch == "aarch64" {
        arch = "(arm64|aarch64)";
    } else if arch == "arm" {
        arch = "(arm|armv7)";
    }
    let regex = format!("(?i){}.*{}.*", os, arch);
    Regex::new(&regex).unwrap()
}
