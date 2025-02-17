pub fn format_permissions(mode: u32) -> String {
    let mut result = String::with_capacity(10);
    result.push(if mode & 0o4000 != 0 { 's' } else { '-' });
    result.push(if mode & 0o0400 != 0 { 'r' } else { '-' });
    result.push(if mode & 0o0200 != 0 { 'w' } else { '-' });
    result.push(if mode & 0o0100 != 0 { 'x' } else { '-' });
    result.push(if mode & 0o0040 != 0 { 'r' } else { '-' });
    result.push(if mode & 0o0020 != 0 { 'w' } else { '-' });
    result.push(if mode & 0o0010 != 0 { 'x' } else { '-' });
    result.push(if mode & 0o0004 != 0 { 'r' } else { '-' });
    result.push(if mode & 0o0002 != 0 { 'w' } else { '-' });
    result.push(if mode & 0o0001 != 0 { 'x' } else { '-' });
    result
}

pub fn format_size(size: u64, human_readable: bool) -> String {
    if !human_readable {
        return format!("{:>8}", size);
    }

    const UNITS: [&str; 6] = ["B", "K", "M", "G", "T", "P"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{:>4}{:}", size as u64, UNITS[unit_index])
    } else {
        format!("{:>4.1}{:}", size, UNITS[unit_index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_permissions() {
        assert_eq!(format_permissions(0o755), "-rwxr-xr-x");
        assert_eq!(format_permissions(0o644), "-rw-r--r--");
        assert_eq!(format_permissions(0o777), "-rwxrwxrwx");
    }

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(1024, true), " 1.0K");
        assert_eq!(format_size(1024 * 1024, true), " 1.0M");
        assert_eq!(format_size(500, false), "     500");
    }
}
