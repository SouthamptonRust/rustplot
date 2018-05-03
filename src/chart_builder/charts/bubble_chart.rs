//!


use chart_builder::charts::*;

/// Structure used for storing chart related data and the drawing of a Bubble Chart.
///
/// Shows the relationship between sets of values, comparing at least 3 sets of data.
#[derive(Clone)]
pub struct BubbleChart {
    data_x: Vec<Vec<f64>>,
    data_y: Vec<Vec<f64>>,
    data_magnitude: Vec<Vec<f64>>,
    pub chart_prop: ChartProp,
    pub axis_prop: AxisProp,
}

impl BubbleChart {
    /// Creates a new instance of a BubbleChart.
    ///
    /// ```chart_title``` is the String to specify the name of the chart displayed at the top of the window.
    ///
    /// ```new_data_x``` is the number data placed on the x-axis of the chart, specifying horizontal positions of bubbles.
    ///
    /// ```new_data_y``` is the number data placed on the y-axis of the chart, specifying vertical positions of bubbles, with indexes corresponding to the same index in new_data_x.
    ///
    /// ```new_data_magnitude``` is the number data used to scale size of bubble proportionally, where indexes correspond to the same index in new_data_x and new_data_y.
    pub fn new(chart_title: String, new_data_x: Vec<Vec<f64>>, new_data_y: Vec<Vec<f64>>, new_data_magnitude: Vec<Vec<f64>>) -> BubbleChart {
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

        BubbleChart {
            data_x: new_data_x,
            data_y: new_data_y,
            data_magnitude: new_data_magnitude,
            chart_prop: ChartProp::new(chart_title, &axis_type),
            axis_prop: AxisProp::new(x_axis_bounds, y_axis_bounds, x_axis_scale, y_axis_scale),
        }
    }
    pub(in chart_builder) fn draw_chart(&self, drawing_area: &DrawingArea) {
        let data_x = self.data_x.clone();
        let data_y = self.data_y.clone();
        let data_mag = self.data_magnitude.clone();
        let legend_values = self.chart_prop.legend_values.clone();

        let chart_title = self.chart_prop.chart_title.clone();

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

            // Drawing Bubble Chart Components

            let min_mag: f64 = data_mag.iter().fold(-0./0., |vec_cur_min, ref x| vec_cur_min.min(x.iter().fold(-0./0., |cur_min, &x| cur_min.min(x))));
            let max_mag: f64 = data_mag.iter().fold(0./0., |vec_cur_max, ref x| vec_cur_max.max(x.iter().fold(0./0., |cur_max, &x| cur_max.max(x))));

            // radius scaling (determining size) always goes with the smaller scaling so guarnteed to fit into screen.
            let radius_scaling;
            if screen_size.1 > screen_size.0 {
                radius_scaling = _horizontal_scaling.min(_vertical_scaling);
            } else {
                radius_scaling = _horizontal_scaling.max(_vertical_scaling);
            }
            use std::f64::consts::PI;

            for j in 0..data_x.len() {
                set_nth_colour_opacity(cr, j, 0.7);

                for i in 0..data_x[j].len() {
                    let x_val = data_x[j][i];
                    let y_val = data_y[j][i];
                    let mag_val = data_mag[j][i];
                    let x = _left_bound + (get_percentage_in_bounds(x_val, x_axis_min, x_axis_max) * _horizontal_scaling);
                    let y = _lower_bound - (get_percentage_in_bounds(y_val, y_axis_min, y_axis_max) * _vertical_scaling);

                    // draw bubble at (x,y)
                    let bubble_radius = ((mag_val - min_mag) / (max_mag - min_mag) * 0.1 + 0.01) * radius_scaling * 1.1;

                    cr.save();
                    // Moving drawing origin to (x,y)
                    cr.translate(x, y);
                    // Scaling the current transformation matrix by different amounts in the X and Y directions.
                    // This is done to assure a circlular object in a rectangular screen.
                    cr.scale(h_scale, v_scale);
                    cr.arc(0., 0., bubble_radius, 0.0, 2.0 * PI);
                    cr.fill();
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

impl Chart for BubbleChart {
    fn draw(&self) {
        build_window(ChartType::Bubble(self.clone()));
    }
}
