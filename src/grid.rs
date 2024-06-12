// use std::fs::File;
// use std::io::{self, Write};

// fn create_grid_svg(size: u32, cell_size: u32, output_file: &str) -> io::Result<()> {
pub fn create_grid_svg(size: u32, cell_size: u32) -> String {
    let width = size * cell_size;
    let height = size * cell_size;

    let mut svg_data = String::new();

    // Add SVG header
    svg_data.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width="{}" height="{}">"#,
        width, height, width, height
    ));

    // Create horizontal lines
    for i in 0..=size {
        let y = i * cell_size;
        svg_data.push_str(&format!(
            r#"<line x1="0" y1="{y}" x2="{width}" y2="{y}" stroke="black" stroke-width="1"/>"#
        ));
    }

    // Create vertical lines
    for i in 0..=size {
        let x = i * cell_size;
        svg_data.push_str(&format!(
            r#"<line x1="{x}" y1="0" x2="{x}" y2="{height}" stroke="black" stroke-width="1"/>"#
        ));
    }

    // Add SVG footer
    svg_data.push_str("</svg>");

    svg_data

    // Write to file
    // let mut file = File::create(output_file)?;
    // file.write_all(svg_data.as_bytes())?;

    // Ok(())
}
