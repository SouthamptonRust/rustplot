extern crate rustplot;

use rustplot::data_parser;
use rustplot::chart_builder;
use rustplot::chart_builder::Chart;

#[test]
fn axis_tests() {
    let mut row_num_1 = data_parser::get_num_col(0, 0, 5, "./resources/axis_tests.csv");
    let mut row_num_2 = data_parser::get_num_col(1, 0, 5, "./resources/axis_tests.csv");
    chart_builder::XYScatterPlot::new(String::from("Test Axis 1"), vec![row_num_1.clone()], vec![row_num_2.clone()]).draw();

    row_num_1 = data_parser::get_num_col(2, 0, 5, "./resources/axis_tests.csv");
    row_num_2 = data_parser::get_num_col(3, 0, 5, "./resources/axis_tests.csv");
    chart_builder::XYScatterPlot::new(String::from("Test Axis 2"), vec![row_num_1.clone()], vec![row_num_2.clone()]).draw();

    row_num_1 = data_parser::get_num_col(4, 0, 5, "./resources/axis_tests.csv");
    row_num_2 = data_parser::get_num_col(5, 0, 5, "./resources/axis_tests.csv");
    chart_builder::XYScatterPlot::new(String::from("Test Axis 3"), vec![row_num_1.clone()], vec![row_num_2.clone()]).draw();

    row_num_1 = data_parser::get_num_col(6, 0, 5, "./resources/axis_tests.csv");
    row_num_2 = data_parser::get_num_col(7, 0, 5, "./resources/axis_tests.csv");
    chart_builder::XYScatterPlot::new(String::from("Test Axis 4"), vec![row_num_1.clone()], vec![row_num_2.clone()]).draw();

    row_num_1 = data_parser::get_num_col(8, 0, 5, "./resources/axis_tests.csv");
    row_num_2 = data_parser::get_num_col(9, 0, 5, "./resources/axis_tests.csv");
    chart_builder::XYScatterPlot::new(String::from("Test Axis 5"), vec![row_num_1.clone()], vec![row_num_2.clone()]).draw();

    row_num_1 = data_parser::get_num_col(10, 0, 5, "./resources/axis_tests.csv");
    row_num_2 = data_parser::get_num_col(11, 0, 5, "./resources/axis_tests.csv");
    chart_builder::XYScatterPlot::new(String::from("Test Axis 6"), vec![row_num_1.clone()], vec![row_num_2.clone()]).draw();

    row_num_1 = data_parser::get_num_col(12, 0, 5, "./resources/axis_tests.csv");
    row_num_2 = data_parser::get_num_col(13, 0, 5, "./resources/axis_tests.csv");
    chart_builder::XYScatterPlot::new(String::from("Test Axis 7"), vec![row_num_1.clone()], vec![row_num_2.clone()]).draw();

    row_num_1 = data_parser::get_num_col(14, 0, 5, "./resources/axis_tests.csv");
    row_num_2 = data_parser::get_num_col(15, 0, 5, "./resources/axis_tests.csv");
    chart_builder::XYScatterPlot::new(String::from("Test Axis 8"), vec![row_num_1.clone()], vec![row_num_2.clone()]).draw();

    row_num_1 = data_parser::get_num_col(16, 0, 5, "./resources/axis_tests.csv");
    row_num_2 = data_parser::get_num_col(17, 0, 5, "./resources/axis_tests.csv");
    chart_builder::XYScatterPlot::new(String::from("Test Axis 9"), vec![row_num_1.clone()], vec![row_num_2.clone()]).draw();

    row_num_1 = data_parser::get_num_col(18, 0, 5, "./resources/axis_tests.csv");
    row_num_2 = data_parser::get_num_col(19, 0, 5, "./resources/axis_tests.csv");
    chart_builder::XYScatterPlot::new(String::from("Test Axis 10"), vec![row_num_1.clone()], vec![row_num_2.clone()]).draw();

    row_num_1 = data_parser::get_num_col(20, 0, 5, "./resources/axis_tests.csv");
    row_num_2 = data_parser::get_num_col(21, 0, 5, "./resources/axis_tests.csv");
    chart_builder::XYScatterPlot::new(String::from("Test Axis 11"), vec![row_num_1.clone()], vec![row_num_2.clone()]).draw();

    row_num_1 = data_parser::get_num_col(22, 0, 10, "./resources/axis_tests.csv");
    row_num_2 = data_parser::get_num_col(23, 0, 10, "./resources/axis_tests.csv");
    chart_builder::XYScatterPlot::new(String::from("Test Axis 12"), vec![row_num_1.clone()], vec![row_num_2.clone()]).draw();

    row_num_1 = data_parser::get_num_col(24, 0, 10, "./resources/axis_tests.csv");
    row_num_2 = data_parser::get_num_col(25, 0, 10, "./resources/axis_tests.csv");
    chart_builder::XYScatterPlot::new(String::from("Test Axis 13"), vec![row_num_1.clone()], vec![row_num_2.clone()]).draw();
}
