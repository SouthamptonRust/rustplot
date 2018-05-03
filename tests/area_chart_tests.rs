extern crate rustplot;

use rustplot::data_parser;
use rustplot::chart_builder;
use rustplot::chart_builder::Chart;

#[test]
fn area_chart_tests() {
    let data_1 = data_parser::get_str_col(0, 0, 5, "./resources/area_chart_tests.csv");
    let data_2 = data_parser::get_num_col(1, 0, 5, "./resources/area_chart_tests.csv");
    let area1 = chart_builder::AreaChart::new(String::from("Test Area Chart 1"), data_1.clone(), vec![data_2.clone()]);
    area1.draw();

    let data_3 = data_parser::get_str_col(2, 0, 5, "./resources/area_chart_tests.csv");
    let data_4 = data_parser::get_num_col(3, 0, 5, "./resources/area_chart_tests.csv");
    let area2 = chart_builder::AreaChart::new(String::from("Test Area Chart 2"), data_3.clone(), vec![data_4.clone()]);
    area2.draw();

    let data_5 = data_parser::get_str_col(4, 0, 5, "./resources/area_chart_tests.csv");
    let data_6 = data_parser::get_num_col(5, 0, 5, "./resources/area_chart_tests.csv");
    let area3 = chart_builder::AreaChart::new(String::from("Test Area Chart 3"), data_5.clone(), vec![data_6.clone()]);
    area3.draw();

    let data_7 = data_parser::get_num_col(6, 0, 5, "./resources/area_chart_tests.csv");
    let data_8 = data_parser::get_num_col(7, 0, 5, "./resources/area_chart_tests.csv");
    let data_9 = data_parser::get_num_col(8, 0, 5, "./resources/area_chart_tests.csv");
    let mut multi_area_1 = chart_builder::AreaChart::new(String::from("Test Area Chart 4"), data_1.clone(), vec![data_7.clone(), data_8.clone(), data_9.clone()]);
    multi_area_1.chart_prop.set_show_legend(true);
    multi_area_1.chart_prop.set_legend_values(vec![String::from("Location 1"), String::from("Location 2"), String::from("Location 3")]);
    multi_area_1.draw();

    let data_10 = data_parser::get_num_col(9, 0, 5, "./resources/area_chart_tests.csv");
    let data_11 = data_parser::get_num_col(10, 0, 5, "./resources/area_chart_tests.csv");
    let data_12 = data_parser::get_num_col(11, 0, 5, "./resources/area_chart_tests.csv");
    let mut multi_area_1 = chart_builder::AreaChart::new(String::from("Test Area Chart 5"), data_1.clone(), vec![data_10.clone(), data_11.clone(), data_12.clone()]);
    multi_area_1.chart_prop.set_show_legend(true);
    multi_area_1.chart_prop.set_legend_values(vec![String::from("Location 1"), String::from("Location 2"), String::from("Location 3")]);
    multi_area_1.draw();

    let mut multi_area_3 = chart_builder::AreaChart::new(String::from("Test Area Chart 6"), data_1.clone(), vec![data_2.clone(), data_4.clone(), data_6.clone()]);
    multi_area_3.chart_prop.set_show_legend(true);
    multi_area_3.chart_prop.set_legend_values(vec![String::from("Location 1"), String::from("Location 2"), String::from("Location 3")]);
    multi_area_3.draw();
}
