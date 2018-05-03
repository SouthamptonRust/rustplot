extern crate rustplot;

use rustplot::data_parser;
use rustplot::chart_builder;
use rustplot::chart_builder::Chart;

#[test]
fn stacked_area_chart_tests() {
    let data_1 = data_parser::get_str_col(0, 0, 5, "./resources/stacked_area_chart_tests.csv");
    let data_2 = data_parser::get_num_col(1, 0, 5, "./resources/stacked_area_chart_tests.csv");
    let stacked_area1 = chart_builder::StackedAreaChart::new(String::from("Test Stacked Area Chart 1"), data_1.clone(), vec![data_2.clone()]);
    stacked_area1.draw();

    let data_3 = data_parser::get_str_col(2, 0, 5, "./resources/stacked_area_chart_tests.csv");
    let data_4 = data_parser::get_num_col(3, 0, 5, "./resources/stacked_area_chart_tests.csv");
    let stacked_area2 = chart_builder::StackedAreaChart::new(String::from("Test Stacked Area Chart 2"), data_3.clone(), vec![data_4.clone()]);
    stacked_area2.draw();

    let data_7 = data_parser::get_num_col(6, 0, 5, "./resources/stacked_area_chart_tests.csv");
    let data_8 = data_parser::get_num_col(7, 0, 5, "./resources/stacked_area_chart_tests.csv");
    let data_9 = data_parser::get_num_col(8, 0, 5, "./resources/stacked_area_chart_tests.csv");
    let mut multi_stacked_area_1 = chart_builder::StackedAreaChart::new(String::from("Test Stacked Area Chart 3"), data_1.clone(), vec![data_7.clone(), data_8.clone(), data_9.clone()]);
    multi_stacked_area_1.chart_prop.set_show_legend(true);
    multi_stacked_area_1.chart_prop.set_legend_values(vec![String::from("Location 1"), String::from("Location 2"), String::from("Location 3")]);
    multi_stacked_area_1.draw();

    let data_10 = data_parser::get_num_col(9, 0, 5, "./resources/stacked_area_chart_tests.csv");
    let data_11 = data_parser::get_num_col(10, 0, 5, "./resources/stacked_area_chart_tests.csv");
    let data_12 = data_parser::get_num_col(11, 0, 5, "./resources/stacked_area_chart_tests.csv");
    let mut multi_stacked_area_1 = chart_builder::StackedAreaChart::new(String::from("Test Stacked Area Chart 4"), data_1.clone(), vec![data_10.clone(), data_11.clone(), data_12.clone()]);
    multi_stacked_area_1.chart_prop.set_show_legend(true);
    multi_stacked_area_1.chart_prop.set_legend_values(vec![String::from("Location 1"), String::from("Location 2"), String::from("Location 3")]);
    multi_stacked_area_1.draw();
}
