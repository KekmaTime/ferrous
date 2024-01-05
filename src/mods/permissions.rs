use std::fs;
use std::os::unix::fs::PermissionsExt;

pub fn check_permissions(path: &str, check_write: bool) -> std::io::Result<bool> {
    let metadata = fs::metadata(path)?;
    let permissions = metadata.permissions();
    let is_readable = permissions.mode() & 0o444 != 0;
    let is_writable = permissions.mode() & 0o222 != 0;
    Ok(!check_write || is_writable && is_readable)
}