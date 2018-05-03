extern crate rustplot;

use rustplot::data_parser;
use rustplot::chart_builder;
use rustplot::chart_builder::Chart;

#[test]
fn xy_scatter_tests() {
    let data_1 = data_parser::get_str_col(0, 0, 1, "./resources/box_plot_tests.csv");
    let data_2 = data_parser::get_num_col(3, 0, 15, "./resources/box_plot_tests.csv");
    let box_whisker = chart_builder::BoxWhiskerPlot::new(String::from("Test Box Whisker Plot 1"), data_1.clone(), vec![data_2.clone()]);
    box_whisker.draw();

    let data_3 = data_parser::get_str_col(0, 0, 2, "./resources/box_plot_tests.csv");
    let data_4 = data_parser::get_num_col(2, 0, 10, "./resources/box_plot_tests.csv");
    let box_whisker = chart_builder::BoxWhiskerPlot::new(String::from("Test Box Whisker Plot 2"), data_3.clone(), vec![data_2.clone(), data_4.clone()]);
    box_whisker.draw();

    let data_5 = data_parser::get_str_col(0, 0, 3, "./resources/box_plot_tests.csv");
    let data_6 = data_parser::get_num_col(1, 0, 10, "./resources/box_plot_tests.csv");
    let box_whisker = chart_builder::BoxWhiskerPlot::new(String::from("Test Box Whisker Plot 3"), data_5.clone(), vec![data_2.clone(), data_6.clone(), data_4.clone()]);
    box_whisker.draw();
}
