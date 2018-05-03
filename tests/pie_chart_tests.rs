extern crate rustplot;

use rustplot::data_parser;
use rustplot::chart_builder;
use rustplot::chart_builder::Chart;

#[test]
fn pie_chart_tests() {
    let data_1 = data_parser::get_str_col(0, 0, 5, "./resources/pie_chart_tests.csv");
    let data_2 = data_parser::get_num_col(1, 0, 5, "./resources/pie_chart_tests.csv");
    let mut pie_1 = chart_builder::PieChart::new(String::from("Test Pie Chart 1"), data_2.clone());
    pie_1.chart_prop.set_legend_values(data_1.clone());
    pie_1.chart_prop.set_show_legend(true);
    pie_1.draw();

    let data_3 = data_parser::get_num_col(2, 0, 5, "./resources/pie_chart_tests.csv");
    let mut pie_2 = chart_builder::PieChart::new(String::from("Test Pie Chart 2"), data_3.clone());
    pie_2.chart_prop.set_legend_values(data_1.clone());
    pie_2.chart_prop.set_show_legend(true);
    pie_2.draw();

    let data_4 = data_parser::get_num_col(3, 0, 5, "./resources/pie_chart_tests.csv");
    let mut pie_3 = chart_builder::PieChart::new(String::from("Test Pie Chart 3"), data_4.clone());
    pie_3.chart_prop.set_legend_values(data_1.clone());
    pie_3.chart_prop.set_show_legend(true);
    pie_3.draw();
}
