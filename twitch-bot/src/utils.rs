pub fn humanize_timestamp_like_timer(timestamp: i32) -> String {
    let d = (timestamp as f64 / (60.0 * 60.0 * 24.0)) as i32;
    let h = (timestamp as f64 / (60.0 * 60.0) % 24.0) as i32;
    let m = (timestamp as f64 % (60.0 * 60.0) / 60.0) as i32;
    let s = (timestamp as f64 % 60.0) as i32;

    if d == 0 && h == 0 && m == 0 {
        format!("{}s", s)
    } else if d == 0 && h == 0 {
        format!("{}m{}s", m, s)
    } else if d == 0 {
        format!("{}h{}m", h, m)
    } else {
        format!("{}d{}h", d, h)
    }
}
