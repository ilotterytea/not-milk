pub fn humanize_timestamp_like_timer(timestamp: i32) -> String {
    let d = (timestamp as f64 / (60.0 * 60.0 * 24.0)).round();
    let h = (timestamp as f64 / (60.0 * 60.0) % 24.0).round();
    let m = (timestamp as f64 % (60.0 * 60.0) / 60.0).round();
    let s = (timestamp as f64 % 60.0).round();

    if d == 0.0 && h == 0.0 && m == 0.0 {
        format!("{}s", s)
    } else if d == 0.0 && h == 0.0 {
        format!("{}m{}s", m, s)
    } else if d == 0.0 {
        format!("{}h{}m", h, m)
    } else {
        format!("{}d{}h", d, h)
    }
}
