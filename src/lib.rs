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
    let regex = format!("(?i)({}.*{}|{}.*{})", os, arch, arch, os);
    Regex::new(&regex).unwrap()
}

pub fn current_os() -> Regex {
	let os = std::env::consts::OS;
	let regex = format!("(?i)({})", os);
	Regex::new(&regex).unwrap()
}

pub fn current_arch() -> Regex {
	let arch = std::env::consts::ARCH;
	let mut regex = format!("(?i)({})", arch);
	if arch == "x86_64" {
		regex = format!("(?i)({}|amd64)", arch);
	} else if arch == "aarch64" {
		regex = format!("(?i)({}|arm64)", arch);
	} else if arch == "arm" {
		regex = format!("(?i)({}|armv7)", arch);
	}
	Regex::new(&regex).unwrap()
}