pub fn svg_header(width: usize, height: usize, fill: &str) -> String {
    format!(
        r#"<svg height="{height}" width="{width}" xmlns="http://www.w3.org/2000/svg">
<rect width="100%" height="100%" fill="{fill}"/>"#,
        width = width,
        height = height,
        fill = fill
    )
}

pub fn svg_footer() -> String {
    "</svg>".to_string()
}
