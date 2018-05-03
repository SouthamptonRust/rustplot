//! This module provides all functionality for creating and drawing customizable charts.
//!
//! Each chart that can drawn has its own structure which can be created with data provided to it and a name.
//!
//! All charts can be drawn using the draw() method provided by the Chart trait that each chart implements.
//!
//! All charts are composed of the ChartProp structure allowing manipulation of functionality common to all charts.
//!
//! Charts that require axis are composed of the AxisProp structure allowing manipulation of axis.
//!
//! # Example of General Use
//!
//! ```
//! // Imports for libary use.
//! use rustplot::chart_builder;
//! use rustplot::chart_builder::Chart;
//!
//! // Data used to build chart (data_parser can be used to fetch this data from a csv file).
//! let x_data = vec![vec![38.0, 67.0, 80.0],     // x data of first series
//!                   vec![29.0, 48.0, 94.0]];    // x data of second series
//! let y_data = vec![vec![27.0, 50.0, 80.0],     // y data of first series
//!                   vec![45.0, 55.0, 78.0]];    // y data of second series
//! let mag_data = vec![vec![2.0, 8.0, 15.0],     // magnitude data of first series
//!                   vec![6.0, 10.0, 18.0]];     // magnitude data of second series
//!
//! // Create instance of a chart with data.
//! let mut example_chart = chart_builder::BubbleChart::new(String::from("Example Bubble Chart"),
//!     x_data,
//!     y_data,
//!     mag_data
//!     );
//!
//! // Add legend data and show ledgend (optional)
//! let legend_values = vec![String::from("Series 1"), String::from("Series 2")];
//! example_chart.chart_prop.set_legend_values(legend_values);
//! example_chart.chart_prop.set_show_legend(true);
//!
//! // Draw the chart displayed in a window.
//! example_chart.draw();
//! ```

use std::env::args;

// Imports from gtk-rs for drawing
extern crate cairo;
extern crate gio;
extern crate gtk;

// warning suppressed as gio::prelude::* is used but a warning is still thrown.
#[allow(unused_imports)]
use self::gio::prelude::*;

use self::gtk::prelude::*;
use self::gtk::DrawingArea;

use self::cairo::Context;
use self::cairo::enums::{FontSlant, FontWeight};
#[allow(unused_imports)]
use self::cairo::MatrixTrait;

/*
 * Defining general Chart component structures and thier constructors.
 */

mod chart_prop;
pub use self::chart_prop::ChartProp;

mod axis_prop;
pub use self::axis_prop::AxisProp;

/*
 * Generic Chart Trait specified.
 */

mod chart;
pub use self::chart::Chart;

/*
 * Window drawing functionality
 */

mod window;

/*
 * Seperated functionality for readability
 */

mod axis_drawer;

/*
 * Defining specific Chart structures.
 */

mod charts;
pub use self::charts::histogram::Histogram;
pub use self::charts::box_whisker_plot::BoxWhiskerPlot;
// pub use self::charts::tree_map::TreeMap;
pub use self::charts::doughnut_chart::DoughnutChart;
pub use self::charts::pie_chart::PieChart;
pub use self::charts::vertical_bar_chart::VerticalBarChart;
pub use self::charts::radar_chart::RadarChart;
pub use self::charts::area_chart::AreaChart;
pub use self::charts::stacked_area_chart::StackedAreaChart;
pub use self::charts::line_chart::LineChart;
pub use self::charts::xy_scatter_plot::XYScatterPlot;
pub use self::charts::bubble_chart::BubbleChart;

/*
 * Helper functions
 */

pub(in chart_builder) fn get_percentage_in_bounds(value: f64, min: f64, max: f64) -> f64 {
    (value - min) / (max - min)
}

/*
 * Public helper functions
 */

 /// Removes possible outliers from a Vector of numbers.
 ///
 /// ```data``` is a ```Vec<f64>``` for which the possible outliers will be removed.
pub fn remove_outliers(data: &Vec<f64>) -> Vec<f64> {
    // Sort data for determing percentile
    let mut sorted_data =  data.clone();
    sorted_data.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    use chart_builder::axis_prop::percentile;

    // Lower quartile is the (n + 1) ÷ 4 th value.
    let lq = percentile(&sorted_data, 0.25);
    // Upper quartile is the 3 (n + 1) ÷ 4 th value.
    let uq = percentile(&sorted_data, 0.75);
    // uq - lq
    let iqr = uq - lq;

    // set limits for outliers
    let lower_limit = lq - iqr * 1.5;
    let upper_limit = uq + iqr * 1.5;

    // remove outliers vaules from data
    sorted_data.retain(|&i|i >= lower_limit && i <= upper_limit);

    sorted_data
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_percentage_in_bounds_tests() {
        assert_eq!(get_percentage_in_bounds(0.0, 0.0, 10.0), 0.0);
        assert_eq!(get_percentage_in_bounds(10.0, 0.0, 10.0), 1.0);
        assert_eq!(get_percentage_in_bounds(5.0, 0.0, 10.0), 0.5);
        assert_eq!(get_percentage_in_bounds(2.0, 0.0, 10.0), 0.2);
        assert_eq!(get_percentage_in_bounds(7.0, 0.0, 10.0), 0.7);
        assert_eq!(get_percentage_in_bounds(-2.0, 0.0, 10.0), -0.2);
        assert_eq!(get_percentage_in_bounds(12.0, 0.0, 10.0), 1.2);
    }
    #[test]
    fn remove_outliers_tests() {
        // No outlier
        let vec_1: Vec<f64> = vec![1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0];
        let result_1 = vec![1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0];
        let outlier_res_1 = remove_outliers(&vec_1);
        assert_eq!(outlier_res_1, result_1);

        // Outlier outside upper limit
        let vec_2: Vec<f64> = vec![1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,40.0];
        let result_2 = vec![1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0];
        let outlier_res_2 = remove_outliers(&vec_2);
        assert_eq!(outlier_res_2, result_2);

        // Outlier outside lower limit
        let vec_3: Vec<f64> = vec![1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,-40.0];
        let result_3= vec![1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0];
        let outlier_res_3 = remove_outliers(&vec_3);
        assert_eq!(outlier_res_3, result_3);
    }
}
