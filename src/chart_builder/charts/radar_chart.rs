//!


use chart_builder::charts::*;

/// Structure used for storing chart related data and the drawing of an Radar Chart.
///
/// Shows values relative to a center point. Use this when categories are not directly comparable.
#[derive(Clone)]
pub struct RadarChart {
    data_labels: Vec<String>,
    data: Vec<Vec<f64>>,
    pub chart_prop: ChartProp,
}

impl RadarChart {
    /// Creates a new instance of a RadarChart.
    ///
    /// ```chart_title``` is the String to specify the name of the chart displayed at the top of the window.
    ///
    /// ```new_data_labels``` specifies the labels placed on each point of the chart (each string is a point).
    ///
    /// ```new_data``` is the number data quanities for each point of the chart corresponding to the labels with the same index in new_data_labels.
    pub fn new(chart_title: String, new_data_labels: Vec<String>, new_data: Vec<Vec<f64>>) -> RadarChart {
        let axis_type: AxisType = AxisType::NoAxis;

        RadarChart {
            data_labels: new_data_labels,
            data: new_data,
            chart_prop: ChartProp::new(chart_title, &axis_type),
        }
    }
    pub(in chart_builder) fn draw_chart(&self, drawing_area: &DrawingArea) {
        let data_labels = self.data_labels.clone();
        let data = self.data.clone();
        let legend_values = self.chart_prop.legend_values.clone();

        let chart_title = self.chart_prop.chart_title.clone();

        let mut screen_size = self.chart_prop.screen_size;
        let show_legend = self.chart_prop.show_legend;
        let legend_size = (screen_size.0 * 0.30).ceil();
        screen_size.0 = if show_legend == false { screen_size.0 } else { screen_size.0 + legend_size };

        let mut h_scale = screen_size.1 / screen_size.0;
        let mut v_scale = screen_size.0 / screen_size.1;

        // Always make text and objects smaller rather than bigger as guarnteed to fit on screen
        if h_scale < v_scale {
            v_scale = 1.0;
        } else {
            h_scale = 1.0;
        }

        // Scaling used dependant use of a legend
        let scalings: (f64, f64, f64, f64 ,f64, f64);
        if show_legend == true {
            scalings = get_legend_scale(screen_size, legend_size);
        } else {
            scalings = get_normal_scale();
        }
        let _horizontal_scaling = scalings.0;
        let _vertical_scaling = scalings.1;
        let _left_bound = scalings.2;
        let _right_bound = scalings.3;
        let _lower_bound = scalings.4;
        let _upper_bound = scalings.5;

        // calculate properties for scaling used in outline
        let outline_prop = calc_data_range(&data, true, 0.7, 0.12, 0.25);
        let outline_min = (outline_prop.0).0;
        let outline_max = (outline_prop.0).1;
        let outline_scale = outline_prop.1;

        drawing_area.connect_draw(move |_, cr| {
            cr.set_dash(&[3., 2., 1.], 1.);
            assert_eq!(cr.get_dash(), (vec![3., 2., 1.], 1.));

            set_defaults(cr, screen_size);

            // Drawing Radar chart components

            let x = _left_bound + 0.5 * _horizontal_scaling;
            let y = _lower_bound - 0.5 * _vertical_scaling;

            let radius_scaling;
            if screen_size.1 > screen_size.0 {
                radius_scaling = _horizontal_scaling.min(_vertical_scaling);
            } else {
                radius_scaling = _horizontal_scaling.max(_vertical_scaling);
            }
            let max_radius = 0.45 * radius_scaling;
            let text_radius = 0.5 * radius_scaling;

            use std::f64::consts::PI;

            cr.save();
            cr.translate(x, y);

            cr.scale(h_scale, v_scale);
            cr.set_line_cap(cairo::LineCap::Round);

            let num_delimiters = ((1.0/outline_scale).trunc() as usize) + 1;

            cr.set_line_width(0.0015);
            cr.set_source_rgba(0.0, 0.0, 0.0, 0.5);

            // get all points of rings/scaling lines
            let mut shape_points: Vec<Vec<(f64, f64)>> = Vec::new();
            for j in 0..num_delimiters {
                shape_points.push(Vec::new());
                for i in 0..data_labels.len() {
                    let radians = (i as f64) / (data_labels.len() as f64) * 2.0 * PI - PI / 2.0;
                    cr.arc(0.0, 0.0, max_radius - (j as f64) * outline_scale * max_radius, 0.0, radians);
                    let point = cr.get_current_point();
                    shape_points[j].push(point);
                }
            }
            cr.new_path();

            // draw outline rings/scaling lines
            for j in 0..shape_points.len() {
                let start_x: f64 = shape_points[j][0].0;
                let start_y: f64 = shape_points[j][0].1;
                let mut prev_x: f64 = start_x; // initalisation as start not needed
                let mut prev_y: f64 = start_y; // initalisation as start not needed

                cr.move_to(start_x, start_y);
                for i in 0..shape_points[j].len() {
                    let x = shape_points[j][i].0;
                    let y = shape_points[j][i].1;

                    if i != 0 {
                        cr.move_to(prev_x, prev_y);
                        cr.line_to(x, y);
                        cr.stroke();
                    }

                    prev_x = x;
                    prev_y = y;
                }
                // join back to start
                cr.move_to(prev_x, prev_y);
                cr.line_to(start_x, start_y);
                cr.stroke();
            }

            // draw text and lines from outter to center
            cr.set_font_size(0.024);
            for i in 0..data_labels.len() {
                let radians = (i as f64) / (data_labels.len() as f64) * 2.0 * PI - PI / 2.0;

                // draw line to center from outter points
                cr.set_source_rgba(0.0, 0.0, 0.0, 0.5);
                cr.arc(0.0, 0.0, max_radius, 0.0, radians);
                let point = cr.get_current_point();
                cr.new_path();
                cr.move_to(point.0, point.1);
                cr.line_to(0.0, 0.0);
                cr.stroke();

                // draw text on outter point
                cr.set_source_rgb(0.0, 0.0, 0.0);
                cr.arc(0.0, 0.0, text_radius, 0.0, radians);
                let point = cr.get_current_point();
                let point_str = data_labels[i].as_str();
                let text_width = cr.text_extents(point_str).width;
                let text_height = cr.text_extents(point_str).height;
                cr.rel_move_to(text_width * (point.0 - 0.5), text_height / 2.0);
                cr.show_text(point_str);
                cr.new_path();
            }

            // draw data rings/scaling lines
            cr.set_line_width(0.007);
            for j in 0..data.len() {
                set_nth_colour(cr, j);

                let mut start_x: f64 = 0.0;
                let mut start_y: f64 = 0.0;
                let mut prev_x: f64 = 0.0;
                let mut prev_y: f64 = 0.0;

                for i in 0..data_labels.len() {
                    let val = data[j][i];
                    let val_radius = get_percentage_in_bounds(val, outline_min, outline_max) * max_radius;
                    let radians = (i as f64) / (data_labels.len() as f64) * 2.0 * PI - PI / 2.0;
                    cr.arc(0.0, 0.0, val_radius, 0.0, radians);
                    let point = cr.get_current_point();
                    cr.new_path();

                    let x = point.0;
                    let y = point.1;

                    if i != 0 {
                        cr.move_to(prev_x, prev_y);
                        cr.line_to(x, y);
                        cr.stroke();
                    } else {
                        start_x = x;
                        start_y = y;
                    }

                    prev_x = x;
                    prev_y = y;
                }
                cr.move_to(prev_x, prev_y);
                cr.line_to(start_x, start_y);
                cr.stroke();
            }

            // draw number labels
            cr.set_source_rgb(0.0, 0.0, 0.0);
            cr.set_font_size(0.016);
            let dps: usize;
            if outline_max >= 100.0 || outline_min <= -100.0 { dps = 0; } else { dps = 2; }
            for j in 0..num_delimiters {
                let num = outline_max - ((outline_max - outline_min) * outline_scale * (j as f64));
                let num_string = format!("{:.*}", dps, num).to_string();
                let num_str = num_string.as_str();
                let text_width = cr.text_extents(num_str).width;
                let text_height = cr.text_extents(num_str).height;

                let x = shape_points[j][0].0;
                let y = shape_points[j][0].1;

                cr.move_to(x - text_width - 0.012 * _horizontal_scaling, y + text_height * 0.5);
                cr.show_text(num_str);
            }

            cr.restore();

            // Chart Title
            draw_title(cr, _left_bound, _upper_bound, h_scale, v_scale, &chart_title);

            // Draw legend if chosen
            if show_legend == true {
                draw_legend(cr, &legend_values, screen_size, legend_size);
            }

            Inhibit(false)
        });
    }
    pub(in chart_builder) fn get_chart_prop(&self) -> ChartProp { self.chart_prop.clone() }
}

impl Chart for RadarChart {
    fn draw(&self) {
        build_window(ChartType::Radar(self.clone()));
    }
}
