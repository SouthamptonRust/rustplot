extern crate rustplot;

use rustplot::data_parser;
use rustplot::chart_builder;
use rustplot::chart_builder::Chart;

#[test]
fn window_size_tests() {
    let data_1 = data_parser::get_str_col(0, 0, 5, "./resources/bar_chart_tests.csv");
    let data_2 = data_parser::get_num_col(1, 0, 5, "./resources/bar_chart_tests.csv");
    let mut bar = chart_builder::VerticalBarChart::new(String::from("Test Window Size 1"), data_1.clone(), vec![data_2.clone()]);
    bar.draw();
    bar.chart_prop.set_screen_size(900.0,900.0);
    bar.draw();
    bar.chart_prop.set_screen_size(300.0,300.0);
    bar.draw();
    bar.chart_prop.set_screen_size(500.0,700.0);
    bar.draw();
    bar.chart_prop.set_screen_size(700.0,500.0);
    bar.draw();
    bar.chart_prop.set_screen_size(500.0,900.0);
    bar.draw();
    bar.chart_prop.set_screen_size(900.0,500.0);
    bar.draw();



    let data_1 = data_parser::get_str_col(0, 0, 5, "./resources/pie_chart_tests.csv");
    let data_2 = data_parser::get_num_col(1, 0, 5, "./resources/pie_chart_tests.csv");
    let mut pie = chart_builder::PieChart::new(String::from("Test Window Size 2"), data_2.clone());
    pie.chart_prop.set_legend_values(data_1.clone());
    pie.chart_prop.set_show_legend(true);
    pie.draw();
    pie.chart_prop.set_screen_size(900.0,900.0);
    pie.draw();
    pie.chart_prop.set_screen_size(300.0,300.0);
    pie.draw();
    pie.chart_prop.set_screen_size(500.0,700.0);
    pie.draw();
    pie.chart_prop.set_screen_size(700.0,500.0);
    pie.draw();
    pie.chart_prop.set_screen_size(500.0,900.0);
    pie.draw();
    pie.chart_prop.set_screen_size(900.0,500.0);
    pie.draw();
}
