
// enum used for determing window size dependent on data provided
#[derive(Clone)]
pub(in chart_builder) enum AxisType {
    NoAxis,
    Single,
    DoubleHorizontal,
    DoubleVertical,
    Full,
}

/// Structure allowing manipulation of functionality common to all charts.
#[derive(Clone)]
pub struct ChartProp {
    pub(in chart_builder) chart_title: String,
    pub(in chart_builder) screen_size: (f64, f64),
    pub(in chart_builder) legend_values: Vec<String>,
    pub(in chart_builder) show_legend: bool,
}

impl ChartProp {
    pub(in chart_builder) fn new(new_chart_title: String, axis_type: &AxisType) -> ChartProp {
        let screen_size = match axis_type {
            &AxisType::NoAxis => (700.0, 700.0),
            &AxisType::Single => (700.0, 700.0),
            &AxisType::DoubleHorizontal => (800.0, 700.0),
            &AxisType::DoubleVertical => (700.0, 800.0),
            &AxisType::Full => (800.0, 800.0),
        };
        ChartProp {
            chart_title: new_chart_title,
            screen_size: screen_size,
            legend_values: Vec::new(),
            show_legend: false,
        }
    }

    /// Sets screen size of chart BEFORE the portion of the screen for legend is added.
    ///
    /// ```width``` is a f64 number specifying the horizontal size.
    ///
    /// ```height``` is a f64 number specifying the vertical size.
    ///
    /// Defaults for both width and height vary from 700-800 depending on data input.
    pub fn set_screen_size(&mut self, width: f64, height: f64) {
        self.screen_size = (width, height);
    }
    /// Sets strings that will be displayed in legend (empty by default).
    ///
    /// ```new_legend_values``` is a Vec<String> containing the desired strings to be displayed in the legend.
    pub fn set_legend_values(&mut self, new_legend_values: Vec<String>) {
        self.legend_values = new_legend_values;
    }
    /// Sets whether a legend will be displayed (false by default).
    ///
    /// ```new_show_legend``` is a boolean value, set to true to show legend and false to hide.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustplot::chart_builder;
    /// use rustplot::chart_builder::Chart;
    ///
    /// // Create instance of pie chart.
    /// let mut pie_chart = chart_builder::PieChart::new(String::from("PIE CHART"), vec![30.0, 50.0, 80.0]);
    ///
    /// // Create a Vec<String> with names of series to be placed in legend.
    /// let legend_values = vec![String::from("Series 1"), String::from("Series 2"), String::from("Series 3")];
    /// // Set legend values.
    /// pie_chart.chart_prop.set_legend_values(legend_values);
    /// // Set legend to be shown.
    /// pie_chart.chart_prop.set_show_legend(true);
    ///
    /// // Draw the pie chart displaying legend.
    /// pie_chart.draw();
    /// ```
    pub fn set_show_legend(&mut self, new_show_legend: bool) {
        self.show_legend = new_show_legend;
    }
}
