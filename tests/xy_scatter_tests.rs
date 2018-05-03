extern crate rustplot;

use rustplot::data_parser;
use rustplot::chart_builder;
use rustplot::chart_builder::Chart;

#[test]
fn xy_scatter_tests() {
    let data_1 = data_parser::get_num_col(0, 0, 5, "./resources/xy_scatter_tests.csv");
    let data_2 = data_parser::get_num_col(1, 0, 5, "./resources/xy_scatter_tests.csv");
    let mut xy = chart_builder::XYScatterPlot::new(String::from("Test XY Scatter Chart"), vec![data_1.clone()], vec![data_2.clone()]);
    xy.draw();
    xy.set_best_fit_line(true);
    xy.draw();
    assert_eq!(true, true);
}
