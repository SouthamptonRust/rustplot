//!


use chart_builder::charts::*;

/// Structure used for storing chart related data and the drawing of an Pie Chart.
///
/// Show proportions of a whole. Can be used when total of numbers in 100%.
#[derive(Clone)]
pub struct PieChart {
    data: Vec<f64>,
    pub chart_prop: ChartProp,
}

impl PieChart {
    /// Creates a new instance of an PieChart.
    ///
    /// ```chart_title``` is the String to specify the name of the chart displayed at the top of the window.
    ///
    /// ```new_data``` is the number data for which is represented in the chart.
    /// Each value in the vector a proportion of the whole data is calculated.
    pub fn new(chart_title: String, new_data: Vec<f64>) -> PieChart {
        let axis_type: AxisType = AxisType::NoAxis;

        PieChart {
            data: new_data,
            chart_prop: ChartProp::new(chart_title, &axis_type),
        }
    }
    pub(in chart_builder) fn draw_chart(&self, drawing_area: &DrawingArea) {
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

        let mut proportions: Vec<f64> = Vec::new();
        // Get sum of all values
        let sum: f64 = data.iter().fold(0.0, |acc, &x| acc + x);
        // Calculate Percentages/Proportions for each segment
        for i in 0..data.len() {
            proportions.push(data[i] / sum);
        }

        drawing_area.connect_draw(move |_, cr| {
            cr.set_dash(&[3., 2., 1.], 1.);
            assert_eq!(cr.get_dash(), (vec![3., 2., 1.], 1.));

            set_defaults(cr, screen_size);

            // Drawing Pie Chart Components

            cr.set_font_size(0.024);
            let x = _left_bound + 0.5 * _horizontal_scaling;
            let y = _lower_bound - 0.5 * _vertical_scaling;

            let radius_scaling;
            if screen_size.1 > screen_size.0 {
                radius_scaling = _horizontal_scaling.min(_vertical_scaling);
            } else {
                radius_scaling = _horizontal_scaling.max(_vertical_scaling);
            }
            let pie_radius = 0.45 * radius_scaling;
            let text_radius = 0.5 * radius_scaling;

            use std::f64::consts::PI;
            let mut cur_rad: f64 = - PI / 2.0;
            let mut prev_rad: f64;

            cr.save();
            // Moving drawing origin to (x,y)
            cr.translate(x, y);
            // Scaling the current transformation matrix by different amounts in the X and Y directions.
            // This is done to assure a circlular object in a rectangular screen.
            cr.scale(h_scale, v_scale);

            for i in 0..proportions.len() {
                let proportion = proportions[i];
                prev_rad = cur_rad;
                cur_rad += proportion * 2.0 * PI;

                // Draw Sector of Pie
                cr.arc(0.0, 0.0, pie_radius, prev_rad, cur_rad);
                cr.line_to(0.0, 0.0);
                cr.close_path();
                set_nth_colour(cr, i);
                cr.fill();
                cr.stroke();

                // Draw percentage text
                cr.set_source_rgb(0.0, 0.0, 0.0);
                let percent_string = format!("{:.*}%", 1, proportion * 100.0).to_string();
                let percent_str = percent_string.as_str();
                let text_width = cr.text_extents(percent_str).width;
                let text_height = cr.text_extents(percent_str).height;
                cr.arc(0.0, 0.0, text_radius, prev_rad, cur_rad - proportion * PI);
                let point = cr.get_current_point();
                // point.0 will be between -0.5 and 0.5
                cr.rel_move_to(text_width * (point.0 - 0.5) ,text_height / 2.0);
                cr.show_text(percent_str);
                cr.new_path();
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

impl Chart for PieChart {
    fn draw(&self) {
        build_window(ChartType::Pie(self.clone()));
    }
}
