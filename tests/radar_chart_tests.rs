extern crate rustplot;

use rustplot::data_parser;
use rustplot::chart_builder;
use rustplot::chart_builder::Chart;

#[test]
fn radar_chart_tests() {
    let data_1 = data_parser::get_str_col(0, 0, 3, "./resources/radar_chart_tests.csv");
    let data_2 = data_parser::get_num_col(1, 0, 3, "./resources/radar_chart_tests.csv");
    let data_3 = data_parser::get_str_col(0, 0, 5, "./resources/radar_chart_tests.csv");
    let data_4 = data_parser::get_num_col(2, 0, 5, "./resources/radar_chart_tests.csv");
    let data_5 = data_parser::get_str_col(0, 0, 6, "./resources/radar_chart_tests.csv");
    let data_6 = data_parser::get_num_col(3, 0, 6, "./resources/radar_chart_tests.csv");
    let data_7 = data_parser::get_str_col(0, 0, 9, "./resources/radar_chart_tests.csv");
    let data_8 = data_parser::get_num_col(4, 0, 9, "./resources/radar_chart_tests.csv");
    let data_9 = data_parser::get_num_col(5, 0, 5, "./resources/radar_chart_tests.csv");
    let data_10 = data_parser::get_num_col(6, 0, 5, "./resources/radar_chart_tests.csv");
    let data_11 = data_parser::get_num_col(7, 0, 5, "./resources/radar_chart_tests.csv");

    let radar = chart_builder::RadarChart::new(String::from("Test Radar Chart 1"), data_1.clone(), vec![data_2.clone()]);
    radar.draw();

    let radar = chart_builder::RadarChart::new(String::from("Test Radar Chart 2"), data_3.clone(), vec![data_4.clone()]);
    radar.draw();

    let radar = chart_builder::RadarChart::new(String::from("Test Radar Chart 3"), data_5.clone(), vec![data_6.clone()]);
    radar.draw();

    let radar = chart_builder::RadarChart::new(String::from("Test Radar Chart 4"), data_7.clone(), vec![data_8.clone()]);
    radar.draw();

    let mut radar = chart_builder::RadarChart::new(String::from("Test Radar Chart 5"), data_3.clone(), vec![data_9.clone(), data_10.clone(), data_11.clone()]);
    radar.chart_prop.set_legend_values(vec![String::from("Person 1"), String::from("Person 2"), String::from("Person 3")]);
    radar.chart_prop.set_show_legend(true);
    radar.draw();
}
