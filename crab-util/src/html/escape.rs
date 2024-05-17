//! 转义和反转义工具

pub const RE_HTML_MARK: &str = "(<[^<]*?>)|(<[\\s]*?/[^<]*?>)|(<[^<]*?/[\\s]*?>)";
pub const TEXT: [[char; 1]; 64] = [[' '; 1]; 64];

fn init_text() -> [[char; 1]; 64] {
    let text = [[' '; 1]; 64];
    (0..64).for_each(|_i| {});
    text
}
