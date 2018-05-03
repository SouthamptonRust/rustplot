
/// Structure allowing manipulation of chart axis.
#[derive(Clone)]
pub struct AxisProp {
    pub(in chart_builder) x_axis_bounds: (f64, f64),
    pub(in chart_builder) y_axis_bounds: (f64, f64),
    pub(in chart_builder) x_axis_scale: f64,
    pub(in chart_builder) y_axis_scale: f64,
    pub(in chart_builder) x_axis_title: String,
    pub(in chart_builder) y_axis_title: String,
}

impl AxisProp {
    pub(in chart_builder) fn new(new_x_axis_bounds: (f64, f64), new_y_axis_bounds: (f64, f64), new_x_axis_scale: f64, new_y_axis_scale: f64) -> AxisProp {
        AxisProp {
            x_axis_bounds: new_x_axis_bounds,
            y_axis_bounds: new_y_axis_bounds,
            x_axis_scale: new_x_axis_scale,
            y_axis_scale: new_y_axis_scale,
            x_axis_title: String::from("x-axis"),
            y_axis_title: String::from("y-axis"),
        }
    }
    /// Sets the upper and lower bounds (maximum and minimum value) of the x-axis.
    /// After new bounds are selected, a new scale is auto calcuated.
    ///
    /// ```min``` is the low bound (minimum value) on the x-axis.
    ///
    /// ```max``` is the upper bound (maximum value) on the x-axis.
    ///
    /// Defaults are intelligently selected with use of the data provided.
    /// Any changes made to a categorical axis (e.g. bar chart) will have no effect.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustplot::chart_builder;
    ///
    /// // Create instance of pie chart.
    /// let mut xy_scatter_plot = chart_builder::XYScatterPlot::new(String::from("SCATTER CHART"), vec![vec![30.0, 50.0, 80.0]], vec![vec![35.0, 45.0, 70.0]]);
    ///
    /// // Set axis bounds.
    /// xy_scatter_plot.axis_prop.set_x_axis_bounds(100.0, 300.0);
    /// ```
    pub fn set_x_axis_bounds(&mut self, min: f64, max: f64) {
        self.x_axis_bounds = (min, max);
        self.x_axis_scale = calc_scale(min, max);
    }
    /// Sets the upper and lower bounds (maximum and minimum value) of the y-axis.
    /// After new bounds are selected, a new scale is auto calcuated.
    ///
    /// ```min``` is the low bound (minimum value) on the y-axis.
    ///
    /// ```max``` is the upper bound (maximum value) on the y-axis.
    ///
    /// Defaults are intelligently selected with use of the data provided.
    /// Any changes made to a histogram will have no effect as frequencies are calculated during drawing.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustplot::chart_builder;
    ///
    /// // Create instance of pie chart.
    /// let mut xy_scatter_plot = chart_builder::XYScatterPlot::new(String::from("SCATTER CHART"), vec![vec![30.0, 50.0, 80.0]], vec![vec![35.0, 45.0, 70.0]]);
    ///
    /// // Set axis bounds.
    /// xy_scatter_plot.axis_prop.set_x_axis_bounds(100.0, 300.0);
    /// ```
    pub fn set_y_axis_bounds(&mut self, min: f64, max: f64) {
        self.y_axis_bounds = (min, max);
        self.y_axis_scale = calc_scale(min, max);
    }
    /// Sets the (scale) interval for numbered delimiters shown on the x-axis.
    ///
    /// ```new_interval``` interval set bewteen each numbered delimiter on axis.
    ///
    /// Defaults are intelligently selected with use of the data provided.
    /// It is important to use this after calling set_x_axis_bounds() as this will recalculate the scale.
    pub fn set_x_axis_interval(&mut self, new_interval: f64) {
        self.x_axis_scale = new_interval / (self.x_axis_bounds.1 - self.x_axis_bounds.0);
    }
    /// Sets the (scale) interval for numbered delimiters shown on the y-axis.
    ///
    /// ```new_interval``` interval set bewteen each numbered delimiter on axis.
    ///
    /// Defaults are intelligently selected with use of the data provided.
    /// It is important to use this after calling set_y_axis_bounds() as this will recalculate the scale.
    pub fn set_y_axis_interval(&mut self, new_interval: f64) {
        self.y_axis_scale = new_interval / (self.y_axis_bounds.1 - self.y_axis_bounds.0);
    }
    /// Sets x-axis title.
    ///
    /// ```new_title``` the String to be displayed on the x-axis.
    pub fn set_x_axis_title(&mut self, new_title: String) {
        self.x_axis_title = new_title;
    }
    /// Sets y-axis title.
    ///
    /// ```new_title``` the String to be displayed on the y-axis.
    pub fn set_y_axis_title(&mut self, new_title: String) {
        self.y_axis_title = new_title;
    }
}

// takes a sorted f64 vector
pub(in chart_builder) fn percentile(data: &Vec<f64>, percentile: f64) -> f64{
    let len = data.len();
    let n = ((len - 1) as f64) * percentile + 1.0;
    // Another method: double n = (N + 1) * excelPercentile;
    if n == 1.0 {
        data[0]
    } else if n == (len as f64) {
        data[len - 1]
    } else {
         let k: usize = n.floor() as usize;
         let d = n - (k as f64);
         data[k - 1] + d * (data[k] - data[k - 1])
    }
}

fn check_outliers(data: &Vec<Vec<f64>>, x_axis: bool) {
    // check for outliers
    let mut outliers: bool = false;

    for i in 0..data.len() {
        // Sort data for determing percentile
        let mut sorted_data =  data[i].clone();
        sorted_data.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        // Lower quartile is the (n + 1) ÷ 4 th value.
        let lq = percentile(&sorted_data, 0.25);
        // Upper quartile is the 3 (n + 1) ÷ 4 th value.
        let uq = percentile(&sorted_data, 0.75);
        // uq - lq
        let iqr = uq - lq;

        // set limits for outliers
        let lower_limit = lq - iqr * 1.5;
        let upper_limit = uq + iqr * 1.5;

        for j in 0..sorted_data.len() {
            if sorted_data[j] < lower_limit || sorted_data[j] > upper_limit {
                outliers = true;
            }
        }
    }
    if outliers {
        if x_axis {
            println!("WARNING - There are possible outliers in the data that could cause distorted x axis scaling.");
        } else {
            println!("WARNING - There are possible outliers in the data that could cause distorted y axis scaling.");
        }
    }
}

fn calc_scale(min: f64, max: f64) -> f64 {
    // Calculate scale
    let axis_range: f64 = (max - min).abs();
    let mag: f64 = (10.0 as f64).powf(axis_range.log10().round());

    let mut interval = mag / 10.0;
    let mut scale = interval / axis_range;

    // attempt to fix intervals not fitting in axis range.
    while axis_range % interval != 0.0 {
        interval = interval / 10.0;
        scale = interval / axis_range;
    }

    scale
}

pub(in chart_builder) fn calc_axis_props(data: &Vec<Vec<f64>>, start_zero: bool, x_axis: bool) -> ((f64, f64), f64) {
    check_outliers(data, x_axis);
    calc_data_range(data, start_zero, 0.8, 0.08, 0.2)
}


pub(in chart_builder) fn calc_data_range(data: &Vec<Vec<f64>>, start_zero: bool, data_fill: f64, min_delim_scale: f64, max_delim_scale: f64) -> ((f64, f64), f64) {

    // Calculate axis bounds
    let min: f64 = data.iter().fold(-0./0., |vec_cur_min, ref x| vec_cur_min.min(x.iter().fold(-0./0., |cur_min, &x| cur_min.min(x))));
    let max: f64 = data.iter().fold(0./0., |vec_cur_max, ref x| vec_cur_max.max(x.iter().fold(0./0., |cur_max, &x| cur_max.max(x))));

    let mut axis_min: f64;
    let mut axis_max: f64;

    if min == max {
        if min == 0.0 {
            axis_min = 0.0;
            axis_max = 1.0;
        } else {
            if min > 0.0 {
                axis_min = 0.0;
                axis_max = max * 2.0;
            } else {
                axis_min = min * 2.0;
                axis_max = 0.0;
            }
        }
    } else {
        let data_range: f64 = (max - min).abs();
        let mut exp: f64 = data_range.log10().round();
        let mut mag;

        loop {
            let ten: f64 = 10.0;
            mag = ten.powf(exp);

            axis_min = (min / mag).floor() * mag;
            axis_max = (max / mag).ceil() * mag;
            // axis must be filled with 80% of data
            if data_range / (axis_max - axis_min) > data_fill { break; }
            exp -= 1.0;
        }

        if axis_min > 0.0 && start_zero { axis_min = 0.0; }
        if axis_max < 0.0 && start_zero { axis_max = 0.0; }
    }
    // Calculate scale
    axis_min = format!("{:.*}", 15, axis_min).parse::<f64>().unwrap();
    axis_max = format!("{:.*}", 15, axis_max).parse::<f64>().unwrap();

    let mut axis_range: f64 = (axis_max - axis_min).abs();
    axis_range = format!("{:.*}", 15, axis_range).parse::<f64>().unwrap();
    let mag: f64 = (10.0 as f64).powf(axis_range.log10().round());

    let mut interval = mag / 10.0;
    let mut scale = interval / axis_range;

    // makes decimals usable in mod by multiplication to whole numbers based on magnitude of interval
    fn decimal_mod(axis_range: f64, interval: f64) -> bool {
        let mut mod_check_range = axis_range;
        let mut mod_check_interval = interval;
        if interval < 1.0 {
            let interval_mag = (10.0 as f64).powf(interval.log10().ceil());
            let interval_mag_inverse = 1.0 / interval_mag * 1000.0; // 1000 is a probably uneeded precausion
            mod_check_range = axis_range * interval_mag_inverse;
            mod_check_interval = interval * interval_mag_inverse;
        }
        // removes accuracy of check but required due to inaccuracies due to using floating point.
        mod_check_range = format!("{:.*}", 12, mod_check_range).parse::<f64>().unwrap();
        mod_check_interval = format!("{:.*}", 12, mod_check_interval).parse::<f64>().unwrap();
        mod_check_range % mod_check_interval != 0.0
    }

    // attempt to fix intervals not fitting in axis range.
    while decimal_mod(axis_range, interval) {
        interval = interval / 10.0;
        scale = interval / axis_range;
    }

    // FOLLOWING IS EXCLUDED IF MIN MAX IS SELECTED as scaling fixes can change min and max
    while scale < min_delim_scale {
        scale = scale * 2.0;
        interval = scale * axis_range;

        if decimal_mod(axis_range, interval) { // axis_range % interval != 0.0
            let half_interval = interval / 2.0;
            if axis_min == 0.0 || (axis_min > 0.0 && (axis_min - half_interval) < 0.0) {
                axis_max += half_interval;
            } else {
                // Extend bound with data closest to it
                // check to find if max - max value or min - min value is smaller
                // add half new interval to smallest result(min or max)
                if (axis_max - max) > (axis_min - min) {
                    axis_max += half_interval;
                } else {
                    axis_min -= half_interval;
                }
            }
            axis_range = (axis_max - axis_min).abs();
            axis_range = format!("{:.*}", 15, axis_range).parse::<f64>().unwrap();

            scale = interval / axis_range;
        }
    }

    while scale > max_delim_scale {
        scale = scale / 2.0;
    }

    ((axis_min, axis_max), scale)
}
