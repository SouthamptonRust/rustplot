//!


use chart_builder::charts::*;

/// Structure used for storing chart related data and the drawing of an Area Chart.
///
/// Show trends over time or in categories, highlighting the magnitude of the change.
#[derive(Clone)]
pub struct AreaChart {
    data_labels: Vec<String>,
    data: Vec<Vec<f64>>,
    pub chart_prop: ChartProp,
    pub axis_prop: AxisProp,
}

impl AreaChart {
    /// Creates a new instance of an AreaChart.
    ///
    /// ```chart_title``` is the String to specify the name of the chart displayed at the top of the window.
    ///
    /// ```new_data_labels``` is the string data placed on the x-axis of the chart, each at a point of an area.
    ///
    /// ```new_data``` is the number data on the y-axis of the chart specifying the position of each point,
    /// with indexes corresponding to the same index in new_data_labels.
    pub fn new(chart_title: String, new_data_labels: Vec<String>, new_data: Vec<Vec<f64>>) -> AreaChart {
        let x_axis_bounds = (0.0, 0.0);
        let x_axis_scale = 1.0 / ((new_data_labels.len() - 1) as f64);
        let y_axis_props = calc_axis_props(&new_data, true, false); // take bar
        let y_axis_bounds = y_axis_props.0;
        let y_axis_scale = y_axis_props.1;

        let axis_type: AxisType =
            if y_axis_bounds.0 < 0.0 && y_axis_bounds.1 > 0.0 { AxisType::DoubleVertical }
            else { AxisType::Single };

        AreaChart {
            data_labels: new_data_labels,
            data: new_data,
            chart_prop: ChartProp::new(chart_title, &axis_type),
            axis_prop: AxisProp::new(x_axis_bounds, y_axis_bounds, x_axis_scale, y_axis_scale),
        }
    }
    pub(in chart_builder) fn draw_chart(&self, drawing_area: &DrawingArea) {
        let data_labels = self.data_labels.clone();
        let data_y = self.data.clone();
        let legend_values = self.chart_prop.legend_values.clone();

        let chart_title = self.chart_prop.chart_title.clone();

        let x_axis_title = self.axis_prop.x_axis_title.clone();
        let x_axis_scale = self.axis_prop.x_axis_scale;

        let y_axis_title = self.axis_prop.y_axis_title.clone();
        let y_axis_scale = self.axis_prop.y_axis_scale;
        let y_axis_bounds: (f64, f64) = self.axis_prop.y_axis_bounds;
        let y_axis_min = y_axis_bounds.0;
        let y_axis_max = y_axis_bounds.1;

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

        drawing_area.connect_draw(move |_, cr| {
            cr.set_dash(&[3., 2., 1.], 1.);
            assert_eq!(cr.get_dash(), (vec![3., 2., 1.], 1.));

            set_defaults(cr, screen_size);

            // Drawing Area chart components
            let intercept = calc_x_intercept(calc_zero_intercept(y_axis_min, y_axis_max), _vertical_scaling, _lower_bound, _upper_bound);
            let x_delimiter_interval: f64 = _horizontal_scaling * x_axis_scale;
            cr.set_line_width(0.005);
            cr.set_line_cap(cairo::LineCap::Round);
            for j in 0..data_y.len() {
                set_nth_colour_opacity(cr, j, 0.7);

                cr.move_to(_left_bound, intercept);

                for i in 0..data_labels.len() {
                    let y_val = data_y[j][i];
                    let x = _left_bound + x_delimiter_interval * (i as f64);
                    let y = _lower_bound - (get_percentage_in_bounds(y_val, y_axis_min, y_axis_max) * _vertical_scaling);

                    // draw lines from previous coordinate to current
                    cr.line_to(x, y);

                }
                // closes off area to fill shape
                cr.line_to(_left_bound + x_delimiter_interval * ((data_labels.len() - 1) as f64), intercept);
                cr.close_path();
                cr.fill();
                cr.stroke();
            }

            // Chart Title
            draw_title(cr, _left_bound, _upper_bound, h_scale, v_scale, &chart_title);

            // Draw Axis
            draw_x_axis_cat(cr, scalings,
                &data_labels, x_axis_scale, calc_zero_intercept(y_axis_min, y_axis_max), &x_axis_title,
                screen_size,
                true);
            draw_y_axis_con(cr, scalings,
                y_axis_min, y_axis_max, y_axis_scale, 0.0, &y_axis_title,
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

impl Chart for AreaChart {
    fn draw(&self) {
        build_window(ChartType::Area(self.clone()));
    }
}
