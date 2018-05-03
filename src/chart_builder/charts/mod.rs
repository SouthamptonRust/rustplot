//!


use chart_builder::*;
use chart_builder::window::*;
use chart_builder::chart_prop::AxisType;
use chart_builder::axis_prop::calc_axis_props;
use chart_builder::axis_prop::calc_data_range;
use chart_builder::axis_prop::percentile;
use chart_builder::axis_drawer::*;


pub(in chart_builder) mod histogram;

pub(in chart_builder) mod box_whisker_plot;

pub(in chart_builder) mod doughnut_chart;

pub(in chart_builder) mod pie_chart;

pub(in chart_builder) mod vertical_bar_chart;

pub(in chart_builder) mod radar_chart;

pub(in chart_builder) mod area_chart;

pub(in chart_builder) mod stacked_area_chart;

pub(in chart_builder) mod line_chart;

pub(in chart_builder) mod xy_scatter_plot;

pub(in chart_builder) mod bubble_chart;


/*
 * Default colours - from a list of 20 distinct colours - values are rgb/255
 * https://sashat.me/2017/01/11/list-of-20-simple-distinct-colors/
 */

pub(self) static COLOURS: [(f64, f64, f64); 20] = [
    (0.9019607843,0.0980392157,0.2941176471),   // Red
    (0.2352941176,0.7058823529,0.2941176471),   // Green
    (1.0,0.8823529412,0.0980392157),            // Yellow
    (0.0,0.5098039216,0.7843137255),            // Blue
    (0.9607843137,0.5098039216,0.1882352941),   // Orange
    (0.568627451,0.1176470588,0.7058823529),    // Purple
    (0.2745098039,0.9411764706,0.9411764706),   // Cyan
    (0.9411764706,0.1960784314,0.9019607843),   // Magenta
    (0.8235294118,1.0,0.2352941176),            // Lime
    (0.9803921569,0.7450980392,0.7450980392),   // Pink
    (0.0,0.5019607843,0.5019607843),            // Teal
    (0.9019607843,0.7450980392,1.0),            // Lavender
    (0.6666666667,0.431372549,0.1568627451),    // Brown
    (1.0,0.9803921569,0.7843137255),            // Beige
    (0.5019607843,0.0,0.0),                     // Maroon
    (0.6666666667,1.0,0.7647058824),            // Mint
    (0.5019607843,0.5019607843,0.0),            // Olive
    (1.0,0.8431372549,0.7058823529),            // Coral
    (0.0,0.0,0.5019607843),                     // Navey
    (0.5019607843,0.5019607843,0.5019607843),   // Grey
];

/*
 * Get scales used drawing in proportions of the screen
 */

pub(self) fn get_normal_scale() -> (f64, f64, f64, f64, f64, f64) {
    (0.76, 0.76, 0.12, 0.88, 0.88, 0.12)
}

pub(self) fn get_legend_scale(screen_size: (f64, f64), legend_size: f64) -> (f64, f64, f64, f64, f64, f64) {
    let h_scale = (screen_size.0 - legend_size) / screen_size.0;
    let _horizontal_scaling: f64 = 0.76 * h_scale;
    let _left_bound = 0.12 * h_scale;
    let _right_bound = 0.88 * h_scale;
    (_horizontal_scaling, 0.76, _left_bound, _right_bound, 0.88, 0.12)
}

// Set defaults for drawing
pub(self) fn set_defaults(cr: &Context, screen_size: (f64, f64)) {
    cr.scale(screen_size.0, screen_size.1);
    cr.set_source_rgb(0.0, 0.0, 0.0);
    cr.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
}

pub(self) fn set_nth_colour(cr: &Context, n: usize) {
    let colour = COLOURS[n];
    cr.set_source_rgb(colour.0, colour.1, colour.2);
}

pub(self) fn set_nth_colour_opacity(cr: &Context, n: usize, opacity: f64) {
    let colour = COLOURS[n];
    cr.set_source_rgba(colour.0, colour.1, colour.2, opacity);
}

pub(self) fn draw_title(cr: &Context, _left_bound: f64, _upper_bound: f64, h_scale: f64, v_scale: f64, chart_title: &String) {
    cr.set_source_rgb(0.0, 0.0, 0.0);
    cr.set_font_size(0.025);
    let mut font_matrix = cr.get_font_matrix();
    font_matrix.scale(h_scale, v_scale);
    cr.set_font_matrix(font_matrix);
    let chart_str = chart_title.as_str();
    let text_height = cr.text_extents(chart_str).height;
    cr.move_to(_left_bound , _upper_bound * 0.5 + text_height * 0.5);
    cr.show_text(chart_str);
}

pub(self) fn draw_legend(cr: &Context, legend_values: &Vec<String>, screen_size: (f64, f64), legend_size: f64) {
    let mut h_scale = screen_size.1 / screen_size.0;
    let mut v_scale = screen_size.0 / screen_size.1;

    let scale_boundary = (screen_size.0 - legend_size) / screen_size.0;
    let scale_width = legend_size / screen_size.0;

    // Always make text and objects smaller rather than bigger as guarnteed to fit on screen
    if h_scale < v_scale {
        v_scale = 1.0;
    } else {
        h_scale = 1.0;
    }

    cr.set_font_size(0.022);
    let mut font_matrix = cr.get_font_matrix();
    font_matrix.scale(h_scale, v_scale);
    cr.set_font_matrix(font_matrix);

    let max_text_height = cr.text_extents("ABCDEFGHIJKLMNOPQRSTUVWXYZ").height;

    for i in 0..legend_values.len() {
        set_nth_colour(cr, i);
        let legend_str = &legend_values[i].as_str();
        let text_height = cr.text_extents(legend_str).height;

        cr.rectangle(
            scale_boundary + scale_width * 0.1,
            0.5 - ((legend_values.len() as f64) * max_text_height * 1.5) / 2.0 + (i as f64) * max_text_height * 1.5 - max_text_height * 0.4,
            max_text_height * 0.8 * h_scale,
            max_text_height * 0.8);
        cr.fill();
        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.move_to(
            scale_boundary + scale_width * 0.1 + max_text_height * 1.5 * h_scale,
            0.5 - ((legend_values.len() as f64) * max_text_height * 1.5) / 2.0 + (i as f64) * max_text_height * 1.5 + text_height/2.0);
        cr.show_text(legend_str);
        cr.stroke();
    }
}
