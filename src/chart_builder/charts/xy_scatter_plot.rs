//!


use chart_builder::charts::*;

/// Structure used for storing chart related data and the drawing of an XY Scatter Plot.
///
/// Shows the relationship between sets of values, comparing at least 2 sets of data.
#[derive(Clone)]
pub struct XYScatterPlot {
    data_x: Vec<Vec<f64>>,
    data_y: Vec<Vec<f64>>,
    best_fit_line: bool,
    pub chart_prop: ChartProp,
    pub axis_prop: AxisProp,
}

impl XYScatterPlot {
    /// Creates a new instance of a BubbleChart.
    ///
    /// ```chart_title``` is the String to specify the name of the chart displayed at the top of the window.
    ///
    /// ```new_data_x``` is the number data placed on the x-axis of the chart, specifying horizontal positions of marks.
    ///
    /// ```new_data_y``` is the number data placed on the y-axis of the chart, specifying vertical positions of marks, with indexes corresponding to the same index in new_data_x.
    pub fn new(chart_title: String, new_data_x: Vec<Vec<f64>>, new_data_y: Vec<Vec<f64>>) -> XYScatterPlot {
        let x_axis_props = calc_axis_props(&new_data_x, false, true);
        let x_axis_bounds = x_axis_props.0;
        let x_axis_scale = x_axis_props.1;

        let y_axis_props = calc_axis_props(&new_data_y, false, false);
        let y_axis_bounds = y_axis_props.0;
        let y_axis_scale = y_axis_props.1;

        let axis_type: AxisType =
            if (x_axis_bounds.0 < 0.0 && x_axis_bounds.1 > 0.0) && (y_axis_bounds.0 < 0.0 && y_axis_bounds.1 > 0.0) { AxisType::Full }
            else if x_axis_bounds.0 < 0.0 && x_axis_bounds.1 > 0.0 { AxisType::DoubleHorizontal }
            else if y_axis_bounds.0 < 0.0 && y_axis_bounds.1 > 0.0 { AxisType::DoubleVertical }
            else { AxisType::Single };

        XYScatterPlot {
            data_x: new_data_x,
            data_y: new_data_y,
            best_fit_line: false,
            chart_prop: ChartProp::new(chart_title, &axis_type),
            axis_prop: AxisProp::new(x_axis_bounds, y_axis_bounds, x_axis_scale, y_axis_scale),
        }
    }
    /// Set if line of best fit should be shown.
    ///
    /// ```best_fit_line``` is a boolean value that should be set to true to show the line of best fit.
    pub fn set_best_fit_line(&mut self, best_fit_line: bool) {
        self.best_fit_line = best_fit_line;
    }
    pub(in chart_builder) fn draw_chart(&self, drawing_area: &DrawingArea) {
        let data_x = self.data_x.clone();
        let data_y = self.data_y.clone();
        let legend_values = self.chart_prop.legend_values.clone();

        let chart_title = self.chart_prop.chart_title.clone();

        let best_fit_line = self.best_fit_line;

        let x_axis_title = self.axis_prop.x_axis_title.clone();
        let x_axis_scale = self.axis_prop.x_axis_scale;
        let x_axis_bounds: (f64, f64) = self.axis_prop.x_axis_bounds;
        let x_axis_min = x_axis_bounds.0;
        let x_axis_max = x_axis_bounds.1;

        let y_axis_title = self.axis_prop.y_axis_title.clone();
        let y_axis_scale = self.axis_prop.y_axis_scale;
        let y_axis_bounds: (f64, f64) = self.axis_prop.y_axis_bounds;
        let y_axis_min = y_axis_bounds.0;
        let y_axis_max = y_axis_bounds.1;

        // Actual size of screen generate if legend section is to be shown.
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

        let mut m: Vec<f64> = Vec::new();
        let mut c: Vec<f64> = Vec::new();
        for i in 0..data_x.len() {
            let n = data_x[i].len() as f64;

            let sum_x = data_x[i].iter().fold(0.0, |acc, &x| acc + x);
            let sum_y = data_y[i].iter().fold(0.0, |acc, &y| acc + y);

            let mut sum_xy: f64 = 0.0;
            for j in 0..data_x[i].len() {
                sum_xy += data_x[i][j] * data_y[i][j];
            }

            let top = n * sum_xy - sum_x * sum_y;

            let sum_x2 = data_x[i].iter().fold(0.0, |acc, &x| acc + x.powf(2.0));

            let bottom = n * sum_x2 - sum_x.powf(2.0);

            m.push(top / bottom);

            c.push((sum_y - m[i] * sum_x) / n);
        }

        drawing_area.connect_draw(move |_, cr| {
            cr.set_dash(&[3., 2., 1.], 1.);
            assert_eq!(cr.get_dash(), (vec![3., 2., 1.], 1.));

            set_defaults(cr, screen_size);

            // Drawing scatter plot chart Components

            // radius scaling (determining size) always goes with the smaller scaling so guarnteed to fit into screen.
            let radius_scaling;
            if screen_size.1 > screen_size.0 {
                radius_scaling = _horizontal_scaling.min(_vertical_scaling);
            } else {
                radius_scaling = _horizontal_scaling.max(_vertical_scaling);
            }
            let mark_radius = 0.009 * radius_scaling;

            use std::f64::consts::PI;

            for j in 0..data_x.len() {
                set_nth_colour(cr, j);

                for i in 0..data_x[j].len() {
                    let x_val = data_x[j][i];
                    let y_val = data_y[j][i];
                    let x = _left_bound + (get_percentage_in_bounds(x_val, x_axis_min, x_axis_max) * _horizontal_scaling);
                    let y = _lower_bound - (get_percentage_in_bounds(y_val, y_axis_min, y_axis_max) * _vertical_scaling);

                    // draw mark (round) at (x,y)
                    cr.save();
                    // Moving drawing origin to (x,y)
                    cr.translate(x, y);
                    // Scaling the current transformation matrix by different amounts in the X and Y directions.
                    // This is done to assure a circlular object in a rectangular screen.
                    cr.scale(h_scale, v_scale);
                    // Draw a 360deg (circular) mark
                    cr.arc(0.0, 0.0, mark_radius, 0.0, 2.0 * PI);
                    cr.fill();
                    cr.stroke();
                    cr.restore();
                }
            }

            // Draw line of best fit
            if best_fit_line == true {
                for i in 0..m.len() {

                    let mut start_x_val = get_percentage_in_bounds((y_axis_min - c[i]) / m[i], x_axis_min, x_axis_max);
                    let mut start_y_val = get_percentage_in_bounds(m[i] * x_axis_min + c[i], y_axis_min, y_axis_max);
                    let mut end_x_val = get_percentage_in_bounds((y_axis_max - c[i]) / m[i], x_axis_min, x_axis_max);
                    let mut end_y_val = get_percentage_in_bounds(m[i] * x_axis_max + c[i], y_axis_min, y_axis_max);

                    if start_x_val < 0.0 {
                        start_x_val = 0.0;
                    }
                    if start_y_val < 0.0 {
                        start_y_val = 0.0;
                    }
                    if end_x_val > 1.0 {
                        end_x_val = 1.0;
                    }
                    if end_y_val > 1.0 {
                        end_y_val = 1.0;
                    }

                    let start_x;
                    let start_y;
                    let end_x;
                    let end_y;
                    start_x = _left_bound - (start_x_val * _horizontal_scaling);;
                    start_y = _lower_bound - (start_y_val * _vertical_scaling);
                    end_x = _left_bound + (end_x_val * _horizontal_scaling);
                    end_y = _lower_bound - (end_y_val * _vertical_scaling);

                    cr.save();
                    cr.set_line_width(0.002);
                    let x_len = (end_x - start_x).abs();
                    let y_len = (end_y - start_y).abs();
                    cr.set_line_width(0.002 *
                        (((x_len/y_len).atan() / (PI / 2.0) * v_scale) +
                        ((y_len/x_len).atan() / (PI / 2.0) * h_scale)));
                    set_nth_colour(cr, i);
                    let dash_array = [0.03, 0.01];
                    cr.set_dash(&dash_array, 1.0);

                    cr.move_to(start_x, start_y);
                    cr.line_to(end_x, end_y);
                    cr.stroke();
                    cr.restore();
                }
            }

            // Chart Title
            draw_title(cr, _left_bound, _upper_bound, h_scale, v_scale, &chart_title);

            // Draw Axis
            draw_x_axis_con(cr, scalings,
                x_axis_min, x_axis_max, x_axis_scale, calc_zero_intercept(y_axis_min, y_axis_max), &x_axis_title,
                screen_size);
            draw_y_axis_con(cr, scalings,
                y_axis_min, y_axis_max, y_axis_scale, calc_zero_intercept(x_axis_min, x_axis_max), &y_axis_title,
                screen_size);

            // Draw legend if chosen
            if show_legend == true {
                draw_legend(cr, &legend_values, screen_size, legend_size);
            }

            Inhibit(false)
        });
    }
    pub(in chart_builder) fn get_chart_prop(&self) -> ChartProp { self.chart_prop.clone() }
}

impl Chart for XYScatterPlot {
    fn draw(&self) {
        build_window(ChartType::XYScat(self.clone()));
    }
}
