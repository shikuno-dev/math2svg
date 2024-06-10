use crate::svg::{svg_footer, svg_header};

pub struct Plot {
    output: String,
}

impl Plot {
    pub fn plot(&mut self, width: usize, height: usize, fill: &str) {
        let mut output = String::new();

        output.push_str(&svg_header(width, height, fill));

        self.output = output;
    }

    pub fn grid(&mut self) {}

    pub fn show(&mut self) -> String {
        self.output.push_str(&svg_footer());

        self.output.clone()
    }

    pub fn scatter(x: Vec<usize>, y: Vec<usize>) {}

    pub fn pie(&mut self, labels: &[&str], sizes: &[f64]) -> String {
        pie(labels, sizes)
    }

    pub fn figure() {}
    pub fn title() {}
    pub fn xlabel() {}
    pub fn ylabel() {}
}

pub fn svg_axes(
    x_start: usize,
    y_start: usize,
    x_end: usize,
    y_end: usize,
    stroke: &str,
) -> String {
    format!(
        r#"<line x1="{x_start}" y1="{y_start}" x2="{x_end}" y2="{y_start}" stroke="{stroke}"/>
<line x1="{x_start}" y1="{y_start}" x2="{x_start}" y2="{y_end}" stroke="{stroke}"/>"#,
        x_start = x_start,
        y_start = y_start,
        x_end = x_end,
        y_end = y_end,
        stroke = stroke
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

fn create_sector(
    cx: f64,
    cy: f64,
    radius: f64,
    start_angle: f64,
    end_angle: f64,
    fill_color: &str,
    label: &str,
) -> String {
    let start_rad = start_angle.to_radians();
    let end_rad = end_angle.to_radians();

    let x1 = cx + radius * start_rad.cos();
    let y1 = cy + radius * start_rad.sin();

    let x2 = cx + radius * end_rad.cos();
    let y2 = cy + radius * end_rad.sin();

    let large_arc_flag = if (end_angle - start_angle).abs() > 180.0 {
        1
    } else {
        0
    };

    let path_data = format!(
        "M {} {} L {} {} A {} {} 0 {} 1 {} {} Z",
        cx, cy, x1, y1, radius, radius, large_arc_flag, x2, y2
    );

    let path_element = format!(
        r#"<path d="{}" fill="{}" stroke="black" />"#,
        path_data, fill_color
    );

    let mid_angle = (start_rad + end_rad) / 2.0;

    let label_x = cx + (radius * 0.7) * mid_angle.cos();
    let label_y = cy + (radius * 0.7) * mid_angle.sin();

    let text_element = format!(
        r#"<text x="{}" y="{}" text-anchor="middle" alignment-baseline="middle" font-size="20" fill="black">{}</text>"#,
        label_x, label_y, label
    );

    format!("{}{}", path_element, text_element)
}

fn pie(labels: &[&str], sizes: &[f64]) -> String {
    let cx = 500.0;
    let cy = 500.0;
    let radius = 300.0;
    let mut current_angle = -90.0;
    let total: f64 = sizes.iter().sum();

    // let colors = [
    //     "red", "green", "blue", "yellow", "orange", "purple", "cyan", "magenta",
    // ];
    let colors = [
        "#3498db", // blue
        "#e74c3c", // red
        "#2ecc71", // green
        "#f1c40f", // yellow
        "#9b59b6", // purple
        "#e67e22", // orange
        "#1abc9c", // cyan
        "#34495e", // dark bule
    ];

    let mut svg_content = String::new();

    for (i, (&size, &label)) in sizes.iter().zip(labels.iter()).enumerate() {
        let percentage = size / total;
        let angle = percentage * 360.0;
        let end_angle = current_angle + angle;

        let fill_color = colors[i % colors.len()];

        let sector_path =
            create_sector(cx, cy, radius, current_angle, end_angle, fill_color, label);
        svg_content.push_str(&sector_path);

        current_angle = end_angle;
    }

    let svg_document = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="1000" viewBox="0 0 1000 1000">
{}
</svg>"#,
        svg_content
    );

    svg_document
}
