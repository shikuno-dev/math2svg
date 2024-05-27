use crate::svg::{svg_footer, svg_header};

pub struct Plot {
    output: String,
}

impl Plot {
    pub fn plot(&mut self, width: usize, height: usize) {
        let mut output = String::new();

        output.push_str(&svg_header(width, height));

        self.output = output;
    }

    pub fn grid(&mut self) {}

    pub fn show(&mut self) -> String {
        self.output.push_str(&svg_footer());

        self.output.clone()
    }

    pub fn scatter(x: Vec<usize>, y: Vec<usize>) {}

    pub fn pie(labels: &[&str], sizes: &[f64]) -> String {
        use std::f64::consts::PI;

        let total: f64 = sizes.iter().sum();
        let mut current_angle = -PI / 2.0;

        let center_x = 100.0;
        let center_y = 100.0;
        let radius = 100.0;

        let mut output = String::new();

        output.push_str(r#"<svg viewBox="0 0 200 200" width="200" height="200" xmlns="http://www.w3.org/2000/svg">"#);

        output.push_str("</svg>");

        output
    }

    pub fn title() {}
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
