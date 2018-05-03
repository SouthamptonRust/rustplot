extern crate rustplot;

use rustplot::data_parser;
use rustplot::chart_builder;
use rustplot::chart_builder::Chart;

#[test]
fn doughnut_chart_tests() {
    let data_1 = data_parser::get_str_col(0, 0, 5, "./resources/doughnut_chart_tests.csv");
    let data_2 = data_parser::get_num_col(1, 0, 5, "./resources/doughnut_chart_tests.csv");
    let data_3 = data_parser::get_num_col(2, 0, 5, "./resources/doughnut_chart_tests.csv");
    let mut doughnut_chart = chart_builder::DoughnutChart::new(String::from("Test Doughnut Chart 1"), vec![data_2.clone(), data_3.clone()]);
    doughnut_chart.chart_prop.set_legend_values(data_1.clone());
    doughnut_chart.chart_prop.set_show_legend(true);
    doughnut_chart.draw();

    let data_4 = data_parser::get_num_col(3, 0, 5, "./resources/doughnut_chart_tests.csv");
    let mut doughnut_chart = chart_builder::DoughnutChart::new(String::from("Test Doughnut Chart 2"), vec![data_2.clone(), data_3.clone(), data_4.clone()]);
    doughnut_chart.chart_prop.set_legend_values(data_1.clone());
    doughnut_chart.chart_prop.set_show_legend(true);
    doughnut_chart.draw();
}
