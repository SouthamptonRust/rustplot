//!


use chart_builder::charts::*;

/// Structure used for storing chart related data and the drawing of a Box and Whisker Plot.
///
/// This chart is used for statistical analysis of data.
/// Shows variations within a set of data.
#[derive(Clone)]
pub struct BoxWhiskerPlot {
    data_labels: Vec<String>,
    data: Vec<Vec<f64>>,
    pub chart_prop: ChartProp,
    pub axis_prop: AxisProp,
}

impl BoxWhiskerPlot {
    /// Creates a new instance of a BoxWhiskerPlot.
    ///
    /// ```chart_title``` is the String to specify the name of the chart displayed at the top of the window.
    ///
    /// ```new_data_labels``` contains strings placed under each box and whisker plot naming them.
    ///
    /// ```new_data``` is the number data for which statistics are generated to create a box plot.
    /// Each inner vector represents a new box and whisker plot named by the same index element in new_data_labels.
    pub fn new(chart_title: String, new_data_labels: Vec<String>, new_data: Vec<Vec<f64>>) -> BoxWhiskerPlot {
        let x_axis_bounds = (0.0, 0.0);
        let x_axis_scale = 1.0 / (new_data_labels.len() as f64);
        let y_axis_props = calc_axis_props(&new_data, true, false); // take bar
        let y_axis_bounds = y_axis_props.0;
        let y_axis_scale = y_axis_props.1;

        let axis_type: AxisType =
            if y_axis_bounds.0 < 0.0 && y_axis_bounds.1 > 0.0 { AxisType::DoubleVertical }
            else { AxisType::Single };

        BoxWhiskerPlot {
            data_labels: new_data_labels,
            data: new_data,
            chart_prop: ChartProp::new(chart_title, &axis_type),
            axis_prop: AxisProp::new(x_axis_bounds, y_axis_bounds, x_axis_scale, y_axis_scale),
        }
    }
    pub(in chart_builder) fn draw_chart(&self, drawing_area: &DrawingArea) {
        let data_labels = self.data_labels.clone();
        let data = self.data.clone();

        let chart_title = self.chart_prop.chart_title.clone();

        let x_axis_title = self.axis_prop.x_axis_title.clone();
        let x_axis_scale = self.axis_prop.x_axis_scale;

        let y_axis_title = self.axis_prop.y_axis_title.clone();
        let y_axis_scale = self.axis_prop.y_axis_scale;
        let y_axis_bounds: (f64, f64) = self.axis_prop.y_axis_bounds;
        let y_axis_min = y_axis_bounds.0;
        let y_axis_max = y_axis_bounds.1;

        let screen_size = self.chart_prop.screen_size;

        let mut h_scale = screen_size.1 / screen_size.0;
        let mut v_scale = screen_size.0 / screen_size.1;

        // Always make text and objects smaller rather than bigger as guarnteed to fit on screen
        if h_scale < v_scale {
            v_scale = 1.0;
        } else {
            h_scale = 1.0;
        }

        let scalings: (f64, f64, f64, f64 ,f64, f64);
        scalings = get_normal_scale();

        let _horizontal_scaling = scalings.0;
        let _vertical_scaling = scalings.1;
        let _left_bound = scalings.2;
        let _right_bound = scalings.3;
        let _lower_bound = scalings.4;
        let _upper_bound = scalings.5;

        // calculate cote values for drawing box plot
        let mut lq: Vec<f64> = Vec::new();
        let mut med: Vec<f64> = Vec::new();
        let mut uq: Vec<f64> = Vec::new();
        let mut iqr: Vec<f64> = Vec::new();
        let mut outliers: Vec<Vec<f64>> = Vec::new();
        let mut min: Vec<f64> = Vec::new();
        let mut max: Vec<f64> = Vec::new();

        for i in 0..data.len() {
            // Sort data for determing percentile
            let mut sorted_data =  data[i].clone();
            sorted_data.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

            // Lower quartile is the (n + 1) ÷ 4 th value.
            lq.push(percentile(&sorted_data, 0.25));
            // Median is the (n + 1) ÷ 2 th value.
            med.push(percentile(&sorted_data, 0.5));
            // Upper quartile is the 3 (n + 1) ÷ 4 th value.
            uq.push(percentile(&sorted_data, 0.75));
            // uq - lq
            iqr.push(uq[i] - lq[i]);

            // Outliers - remove from data set and place in outliers - allow limit with default being 1.5iqr from lq and uq
            // set limits for outliers
            let lower_limit = lq[i] - iqr[i] * 1.5;
            let upper_limit = uq[i] + iqr[i] * 1.5;

            // store all outlier values
            let mut temp = sorted_data.clone();
            temp.retain(|&i|i < lower_limit || i > upper_limit);
            outliers.push(temp);

            // remove outliers vaules from main data
            sorted_data.retain(|&i|i > lower_limit && i < upper_limit);

            // get min and max from remaining data
            min.push(sorted_data.iter().fold(-0./0., |cur_min, &x| cur_min.min(x)));
            max.push(sorted_data.iter().fold(0./0., |cur_max, &x| cur_max.max(x)));
        }

        drawing_area.connect_draw(move |_, cr| {
            cr.set_dash(&[3., 2., 1.], 1.);
            assert_eq!(cr.get_dash(), (vec![3., 2., 1.], 1.));

            set_defaults(cr, screen_size);

            // Drawing box whisker plot components

            // radius scaling (determining size) always goes with the smaller scaling so guarnteed to fit into screen.
            let radius_scaling;
            if screen_size.1 > screen_size.0 {
                radius_scaling = _horizontal_scaling.min(_vertical_scaling);
            } else {
                radius_scaling = _horizontal_scaling.max(_vertical_scaling);
            }
            let mark_radius = 0.008 * radius_scaling;

            use std::f64::consts::PI;

            let intercept = calc_x_intercept(calc_zero_intercept(y_axis_min, y_axis_max), _vertical_scaling, _lower_bound, _upper_bound);
            let x_delimiter_interval: f64 = _horizontal_scaling * x_axis_scale;
            let bar_width = 0.6 / (data.len() as f64);

            for i in 0..data.len() {
                let x = _left_bound - (x_delimiter_interval / 2.0) + x_delimiter_interval * ((i + 1) as f64);

                set_nth_colour(cr, i);

                // draw outliers first
                cr.set_line_width(0.0025);
                for j in 0..outliers[i].len() {
                    let y_val = outliers[i][j];
                    let y = _lower_bound - (get_percentage_in_bounds(y_val, y_axis_min, y_axis_max) * _vertical_scaling);
                    // draw mark (round) at (x,y)
                    cr.save();
                    // Moving drawing origin to (x,y)
                    cr.translate(x, y);
                    // Scaling the current transformation matrix by different amounts in the X and Y directions.
                    // This is done to assure a circlular object in a rectangular screen.
                    cr.scale(h_scale, v_scale);
                    cr.arc(0.0, 0.0, mark_radius, 0.0, 2.0 * PI);
                    cr.stroke();
                    cr.restore();
                }

                cr.set_line_width(0.002);

                // draw lq to uq block
                cr.rectangle(
                    x - (bar_width / 2.0) * _horizontal_scaling,
                    _lower_bound - (get_percentage_in_bounds(lq[i], y_axis_min, y_axis_max) * _vertical_scaling),
                    bar_width * _horizontal_scaling,
                    _lower_bound - (get_percentage_in_bounds(iqr[i], y_axis_min, y_axis_max) * _vertical_scaling) - intercept);
                // preserve used to keep shape to draw outline
                cr.fill_preserve();
                cr.stroke_preserve();

                cr.set_source_rgb(0.0, 0.0, 0.0);

                // median line
                cr.move_to(x - bar_width * _horizontal_scaling / 2.0, _lower_bound - (get_percentage_in_bounds(med[i], y_axis_min, y_axis_max) * _vertical_scaling));
                cr.line_to(x + bar_width * _horizontal_scaling / 2.0, _lower_bound - (get_percentage_in_bounds(med[i], y_axis_min, y_axis_max) * _vertical_scaling));

                // line from lq to min
                cr.move_to(x, _lower_bound - (get_percentage_in_bounds(lq[i], y_axis_min, y_axis_max) * _vertical_scaling));
                cr.line_to(x, _lower_bound - (get_percentage_in_bounds(min[i], y_axis_min, y_axis_max) * _vertical_scaling));
                // end of min line
                cr.move_to(x - bar_width * _horizontal_scaling * 0.2, _lower_bound - (get_percentage_in_bounds(min[i], y_axis_min, y_axis_max) * _vertical_scaling));
                cr.line_to(x + bar_width * _horizontal_scaling * 0.2, _lower_bound - (get_percentage_in_bounds(min[i], y_axis_min, y_axis_max) * _vertical_scaling));

                // line from uq to max
                cr.move_to(x, _lower_bound - (get_percentage_in_bounds(uq[i], y_axis_min, y_axis_max) * _vertical_scaling));
                cr.line_to(x, _lower_bound - (get_percentage_in_bounds(max[i], y_axis_min, y_axis_max) * _vertical_scaling));
                // end of max line
                cr.move_to(x - bar_width * _horizontal_scaling * 0.2, _lower_bound - (get_percentage_in_bounds(max[i], y_axis_min, y_axis_max) * _vertical_scaling));
                cr.line_to(x + bar_width * _horizontal_scaling * 0.2, _lower_bound - (get_percentage_in_bounds(max[i], y_axis_min, y_axis_max) * _vertical_scaling));

                cr.stroke();
            }

            // Chart Title
            draw_title(cr, _left_bound, _upper_bound, h_scale, v_scale, &chart_title);

            // Draw Axis
            draw_x_axis_cat(cr, scalings,
                &data_labels, x_axis_scale, calc_zero_intercept(y_axis_min, y_axis_max), &x_axis_title,
                screen_size,
                false);
            draw_y_axis_con(cr, scalings,
                y_axis_min, y_axis_max, y_axis_scale, 0.0, &y_axis_title,
                screen_size);

            Inhibit(false)
        });
    }
    pub(in chart_builder) fn get_chart_prop(&self) -> ChartProp { self.chart_prop.clone() }
}

impl Chart for BoxWhiskerPlot {
    fn draw(&self) {
        build_window(ChartType::BoxWhisk(self.clone()));
    }
}
