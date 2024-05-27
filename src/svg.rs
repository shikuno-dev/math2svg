pub fn svg_header(width: usize, height: usize) -> String {
    format!(
        r#"<svg height="{height}" width="{width}" xmlns="http://www.w3.org/2000/svg">
<rect width="100%" height="100%" fill="white"/>"#,
        width = width,
        height = height
    )
}

pub fn svg_footer() -> String {
    "</svg>".to_string()
}
