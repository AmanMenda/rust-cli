#![allow(unused)]

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write)
{
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}