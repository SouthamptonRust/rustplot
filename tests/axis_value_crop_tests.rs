extern crate rustplot;

use rustplot::data_parser;
use rustplot::chart_builder;
use rustplot::chart_builder::Chart;

#[test]
fn axis_value_crop_tests() {
    let data_1 = data_parser::get_str_col(0, 0, 5, "./resources/axis_value_crop_tests.csv");
    let data_2 = data_parser::get_num_col(1, 0, 5, "./resources/axis_value_crop_tests.csv");
    let bar1 = chart_builder::VerticalBarChart::new(String::from("Axis Crop Chart 1"), data_1.clone(), vec![data_2.clone()]);
    bar1.draw();

    let data_3 = data_parser::get_str_col(2, 0, 5, "./resources/axis_value_crop_tests.csv");
    let data_4 = data_parser::get_num_col(3, 0, 5, "./resources/axis_value_crop_tests.csv");
    let bar2 = chart_builder::VerticalBarChart::new(String::from("Axis Crop Chart 2"), data_3.clone(), vec![data_4.clone()]);
    bar2.draw();

    let data_5 = data_parser::get_str_col(4, 0, 5, "./resources/axis_value_crop_tests.csv");
    let data_6 = data_parser::get_num_col(5, 0, 5, "./resources/axis_value_crop_tests.csv");
    let bar3 = chart_builder::VerticalBarChart::new(String::from("Axis Crop Chart 3"), data_5.clone(), vec![data_6.clone()]);
    bar3.draw();

    let data_7 = data_parser::get_str_col(6, 0, 5, "./resources/axis_value_crop_tests.csv");
    let data_8 = data_parser::get_num_col(7, 0, 5, "./resources/axis_value_crop_tests.csv");
    let bar4 = chart_builder::VerticalBarChart::new(String::from("Axis Crop Chart 4"), data_7.clone(), vec![data_8.clone()]);
    bar4.draw();
}
