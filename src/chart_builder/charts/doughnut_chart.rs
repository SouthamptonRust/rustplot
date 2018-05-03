//!


use chart_builder::charts::*;

/// Structure used for storing chart related data and the drawing of an Doughnut Chart.
///
/// Show proportions of a whole for multiple sets of data. Can be used when total of numbers in 100%.
#[derive(Clone)]
pub struct DoughnutChart {
    data: Vec<Vec<f64>>,
    pub chart_prop: ChartProp,
}

impl DoughnutChart {
    /// Creates a new instance of a DoughnutChart.
    ///
    /// ```chart_title``` is the String to specify the name of the chart displayed at the top of the window.
    ///
    /// ```new_data``` is the number data for which is represented in the chart.
    /// Each inner vector is a new ring in the doughnut chart.represnents a proportion within the whole data is calculated.
    /// Each value within an inner vector a proportion of the whole data (within its inner vector) is calculated.
    pub fn new(chart_title: String, new_data: Vec<Vec<f64>>) -> DoughnutChart {
        let axis_type: AxisType = AxisType::NoAxis;

        DoughnutChart {
            data: new_data,
            chart_prop: ChartProp::new(chart_title, &axis_type),
        }
    }
    pub(in chart_builder) fn draw_chart(&self, drawing_area: &DrawingArea) {
        let data = self.data.clone();
        let legend_values = self.chart_prop.legend_values.clone();

        let chart_title = self.chart_prop.chart_title.clone();

        let show_legend = self.chart_prop.show_legend;
        let mut screen_size = self.chart_prop.screen_size;
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

        let mut proportions: Vec<Vec<f64>> = Vec::new();
        // Get sum of all values
        for i in 0..data.len() {
            let sum: f64 = data[i].iter().fold(0.0, |acc, &x| acc + x);
            proportions.push(Vec::new());
            for j in 0..data[i].len() {
                proportions[i].push(data[i][j] / sum);
            }
        }

        // Can you multiple times for same image.
        drawing_area.connect_draw(move |_, cr| {
            cr.set_dash(&[3., 2., 1.], 1.);
            assert_eq!(cr.get_dash(), (vec![3., 2., 1.], 1.));

            set_defaults(cr, screen_size);

            // Drawing Doughnut chart Components

            cr.set_line_width(0.003);
            let x = _left_bound + 0.5 * _horizontal_scaling;
            let y = _lower_bound - 0.5 * _vertical_scaling;

            let radius_scaling;
            if screen_size.1 > screen_size.0 {
                radius_scaling = _horizontal_scaling.min(_vertical_scaling);
            } else {
                radius_scaling = _horizontal_scaling.max(_vertical_scaling);
            }
            let max_radius = 0.45 * radius_scaling;
            let min_radius = 0.20 * radius_scaling;
            let sector_width = 0.25 / (proportions.len() as f64) * radius_scaling;
            let mut outer_radius: f64;

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
                outer_radius = max_radius - (i as f64) * sector_width;
                for j in 0..proportions[i].len() {
                    let proportion = proportions[i][j];
                    prev_rad = cur_rad;
                    cur_rad += proportion * 2.0 * PI;

                    // Draw Sector of Doughnut
                    cr.arc(0.0, 0.0, outer_radius, prev_rad, cur_rad);
                    cr.line_to(0.0, 0.0);
                    cr.close_path();
                    set_nth_colour(cr, j);
                    cr.fill_preserve();
                    cr.stroke_preserve();
                    cr.set_source_rgb(255.0, 255.0, 255.0);
                    cr.stroke();
                }
            }
            cr.close_path();
            // white area in center
            cr.arc(0.0, 0.0, min_radius, 0.0, 2.0 * PI);
            cr.set_source_rgb(255.0, 255.0, 255.0);
            cr.fill();
            cr.stroke();
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

impl Chart for DoughnutChart {
    fn draw(&self) {
        build_window(ChartType::Doughnut(self.clone()));
    }
}
