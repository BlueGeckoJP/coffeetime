use std::f64::consts::PI;

use gtk::{
    DrawingArea, cairo,
    prelude::{DrawingAreaExt, DrawingAreaExtManual, StyleContextExt, WidgetExt},
};

fn draw_rounded_rectangle_top(
    context: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    radius: f64,
) {
    let degrees = PI / 180.0;

    context.new_path();

    // Top left corner
    context.arc(
        x + width - radius,
        y + radius,
        radius,
        -90.0 * degrees,
        0.0 * degrees,
    );

    // Bottom right corner
    context.line_to(x + width, y + height);

    // Bottom left corner
    context.line_to(x, y + height);

    // Top left corner
    context.line_to(x, y + radius);
    context.arc(
        x + radius,
        y + radius,
        radius,
        180.0 * degrees,
        270.0 * degrees,
    );

    context.close_path();
}

pub fn draw_graph() -> DrawingArea {
    let drawing_area = DrawingArea::new();
    drawing_area.set_content_width(400);
    drawing_area.set_content_height(300);

    drawing_area.set_draw_func(move |_, context, width, height| {
        let label_for_get_color = gtk::Label::new(None);
        let label_style = label_for_get_color.style_context();
        let label_color = label_style.color();
        let label_red = label_color.red() as f64;
        let label_green = label_color.green() as f64;
        let label_blue = label_color.blue() as f64;

        // test data
        let data = [
            ("Mon", 5.5),
            ("Tue", 6.0),
            ("Wed", 4.0),
            ("Thu", 7.5),
            ("Fri", 3.0),
            ("Sat", 8.0),
            ("Sun", 2.5),
        ];

        let bar_count = data.len() as f64;
        let padding_left = 30.0;
        let padding_right = 50.0;
        let padding_top = 40.0;
        let padding_bottom = 40.0;
        let chart_width = width as f64 - padding_left - padding_right;
        let chart_height = height as f64 - padding_top - padding_bottom;
        let bar_width = (chart_width / bar_count) * 0.7;
        let bar_spacing = chart_width / bar_count;
        let max_value = 24.0; // 24 hours max

        // draw grid lines and y-axis labels
        context.set_line_width(1.0);
        for i in 0..=5 {
            context.set_source_rgb(1.0, 1.0, 1.0);
            let y = padding_top + (chart_height / 5.0) * i as f64;
            context.move_to(padding_left, y);
            context.line_to(width as f64 - padding_right, y);
            context.stroke().unwrap();

            // draw y-axis labels
            context.set_source_rgb(label_red, label_green, label_blue);
            let value = max_value - (max_value / 5.0) * i as f64;
            context.move_to(width as f64 - padding_right + 5.0, y + 5.0);
            context.show_text(&format!("{:.0}h", value)).unwrap();
        }

        // draw bars
        for (i, (label, value)) in data.iter().enumerate() {
            let x = padding_left + (i as f64 * bar_spacing) + (bar_spacing - bar_width) / 2.0;
            let bar_height = (value / max_value) * chart_height;
            let y = padding_top + chart_height - bar_height;

            // draw bar
            context.set_source_rgb(0.3, 0.6, 0.9);
            draw_rounded_rectangle_top(context, x, y, bar_width, bar_height + 1.0, 10.0);
            context.fill().unwrap();

            // draw x-axis labels and values
            context.set_source_rgb(label_red, label_green, label_blue);
            context.move_to(x + bar_width / 4.0, height as f64 - padding_bottom + 20.0);
            context.show_text(label).unwrap();
        }
    });

    drawing_area
}
