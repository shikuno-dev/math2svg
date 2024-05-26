pub fn plot() -> String {}

fn svg_header(width: usize, height: usize) -> String {
    format!(
        r#"<svg height="{height}" width="{width}" xmlns="http://www.w3.org/2000/svg">
<rect width="100%" height="100%" fill="white"/>"#,
        width = width,
        height = height
    )
}

pub fn svg_axes(x_start: usize, y_start: usize, x_end: usize, y_end: usize) -> String {
    format!(
        r#"<line x1="{x_start}" y1="{y_start}" x2="{x_end}" y2="{y_start}" stroke="black"/>
<line x1="{x_start}" y1="{y_start}" x2="{x_start}" y2="{y_end}" stroke="black"/>"#,
        x_start = x_start,
        y_start = y_start,
        x_end = x_end,
        y_end = y_end,
    )
}

// fn svg_labels(horizontal_labels: &[(u32, &str)], vertical_labels: &[(u32, &str)]) -> String {
//     let mut labels = String::new();

//     // Add horizontal labels
//     for &(x, label) in horizontal_labels {
//         labels.push_str(&format!(
//             r#"<text x="{x}" y="365" font-size="10" text-anchor="middle">{label}</text>"#,
//             x = x,
//             label = label
//         ));
//     }

//     // Add vertical labels
//     for &(y, label) in vertical_labels {
//         labels.push_str(&format!(
//             r#"<text x="45" y="{y}" font-size="10" text-anchor="end">{label}</text>"#,
//             y = y,
//             label = label
//         ));
//     }

//     labels
// }

// pub fn svg_grid(horizontal_lines: &[usize], vertical_lines: &[usize]) -> String {
//     let mut grid_lines = String::new();

//     for &y in horizontal_lines {
//         grid_lines.push_str(&format!(
//             r#"<line x1="50" y1="{y}" x2="350" y2="{y}" stroke="gray" stroke-dasharray="5,5"/>"#,
//             y = y
//         ));
//     }

//     for &x in vertical_lines {
//         grid_lines.push_str(&format!(
//             r#"<line x1="{x}" y1="350" x2="{x}" y2="50" stroke="gray" stroke-dasharray="5,5"/>"#,
//             x = x
//         ));
//     }

//     grid_lines
// }
