pub fn get_version_string(
    major: u32,
    minor: u32,
    patch: u32,
    postfix: &str,
    build_id: &str,
) -> String {
    let mut v = format!("{major}.{minor}.{patch}");

    if !postfix.is_empty() {
        v = format!("{v}-{postfix}");
    }

    if !build_id.is_empty() {
        v = format!("{v}+{build_id}");
    }

    v
}

pub fn get_version_int(major: u32, minor: u32, patch: u32) -> u32 {
    major * 1_000_000 + minor * 1_000 + patch
}
