//!


use chart_builder::charts::*;

/// Structure used for storing chart related data and the drawing of an Histogram.
///
/// This chart is used for statistical analysis of data.
/// Shows the distribution of groups data.
#[derive(Clone)]
pub struct Histogram {
    data: Vec<f64>,
    pub chart_prop: ChartProp,
    pub axis_prop: AxisProp,
}

impl Histogram {
    /// Creates a new instance of a Histogram.
    ///
    /// ```chart_title``` is the String to specify the name of the chart displayed at the top of the window.
    ///
    /// ```new_data``` is the number data used to generate the frequencies for drawing the histogram.
    pub fn new(chart_title: String, new_data: Vec<f64>) -> Histogram {
        let x_axis_props = calc_axis_props(&vec![new_data.clone()], false, true);
        let x_axis_bounds = x_axis_props.0;
        let x_axis_scale = x_axis_props.1;
        let y_axis_bounds = (0.0, 0.0);
        let y_axis_scale = 0.0;

        let axis_type: AxisType =
            if x_axis_bounds.0 < 0.0 && x_axis_bounds.1 > 0.0 { AxisType::DoubleHorizontal }
            else { AxisType::Single };

        Histogram {
            data: new_data,
            chart_prop: ChartProp::new(chart_title, &axis_type),
            axis_prop: AxisProp::new(x_axis_bounds, y_axis_bounds, x_axis_scale, y_axis_scale),
        }
    }
    pub(in chart_builder) fn draw_chart(&self, drawing_area: &DrawingArea) {
        let data = self.data.clone();

        let chart_title = self.chart_prop.chart_title.clone();

        let x_axis_title = self.axis_prop.x_axis_title.clone();
        let x_axis_scale = self.axis_prop.x_axis_scale;
        let x_axis_bounds: (f64, f64) = self.axis_prop.x_axis_bounds;
        let x_axis_min = x_axis_bounds.0;
        let x_axis_max = x_axis_bounds.1;

        let y_axis_title = self.axis_prop.y_axis_title.clone();

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
        scalings = get_normal_scale();

        let _horizontal_scaling = scalings.0;
        let _vertical_scaling = scalings.1;
        let _left_bound = scalings.2;
        let _right_bound = scalings.3;
        let _lower_bound = scalings.4;
        let _upper_bound = scalings.5;

        // create an empty vector with counters starting at zero,
        // with size of the number of ranges used in histogram.
        let mut frequencies: Vec<f64> = Vec::new();
        let num_ranges = (1.0/x_axis_scale).trunc() as usize;
        for _i in 0..num_ranges {
            frequencies.push(0.0);
        }

        // count occurrences of each value belonging to a group/range
        let x_axis_range = x_axis_max - x_axis_min;
        let groups_range = x_axis_range * x_axis_scale;
        for i in 0..data.len() {
            let val = data[i];

            // adds small over flow to last group
            if val == x_axis_max {
                frequencies[num_ranges - 1] += 1.0;
            } else {
                let mut range_count = 0;

                loop {
                    let range_min = x_axis_min + groups_range * (range_count as f64);
                    let range_max = range_min + groups_range;

                    // following convention in dealing with bounaries
                    if val >= range_min && val < range_max {
                        frequencies[range_count] += 1.0;
                        break;
                    }
                    range_count += 1;
                }
            }
        }

        // generate y axis properties from frequencies
        let y_axis_props = calc_axis_props(&vec![frequencies.clone()], true, false);
        let y_axis_bounds = y_axis_props.0;
        let y_axis_scale = y_axis_props.1;
        let y_axis_min = y_axis_bounds.0;
        let y_axis_max = y_axis_bounds.1;

        drawing_area.connect_draw(move |_, cr| {
            cr.set_dash(&[3., 2., 1.], 1.);
            assert_eq!(cr.get_dash(), (vec![3., 2., 1.], 1.));

            set_defaults(cr, screen_size);


            // Drawing Histogram Components
            let x_delimiter_interval: f64 = _horizontal_scaling * x_axis_scale;
            let bar_width = x_delimiter_interval;
            cr.set_line_width(0.002);
            for i in 0..frequencies.len() {
                // get frequency for group
                let val = frequencies[i];

                // fill bar shape
                // use first colour for all bars as they are all for the same series
                set_nth_colour(cr, 0);
                cr.rectangle(
                    _left_bound + x_delimiter_interval * (i as f64),
                    _lower_bound,
                    bar_width,
                    - get_percentage_in_bounds(val, y_axis_min, y_axis_max) * _vertical_scaling);
                cr.fill_preserve();
                cr.stroke_preserve();

                // draw bar outline
                cr.set_source_rgb(0.0, 0.0, 0.0);
                cr.stroke();
            }

            // Chart Title
            draw_title(cr, _left_bound, _upper_bound, h_scale, v_scale, &chart_title);

            // Draw Axis
            draw_x_axis_con(cr, scalings,
                x_axis_min, x_axis_max, x_axis_scale, 0.0, &x_axis_title,
                screen_size);
            draw_y_axis_con(cr, scalings,
                y_axis_min, y_axis_max, y_axis_scale, calc_zero_intercept(x_axis_min, x_axis_max), &y_axis_title,
                screen_size);

            Inhibit(false)
        });
    }
    pub(in chart_builder) fn get_chart_prop(&self) -> ChartProp { self.chart_prop.clone() }
}

impl Chart for Histogram {
    fn draw(&self) {
        build_window(ChartType::Hist(self.clone()));
    }
}
