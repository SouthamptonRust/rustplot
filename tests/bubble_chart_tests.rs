extern crate rustplot;

use rustplot::data_parser;
use rustplot::chart_builder;
use rustplot::chart_builder::Chart;

#[test]
fn bubble_chart_tests() {
    let data_1 = data_parser::get_num_col(0, 0, 5, "./resources/bubble_chart_tests.csv");
    let data_2 = data_parser::get_num_col(1, 0, 5, "./resources/bubble_chart_tests.csv");
    let data_3 = data_parser::get_num_col(2, 0, 5, "./resources/bubble_chart_tests.csv");

   let mut multi_bubble = chart_builder::BubbleChart::new(String::from("Test Bubble Chart 1"),
       vec![data_1.clone()],
       vec![data_2.clone()],
       vec![data_3.clone()]
        );
    multi_bubble.axis_prop.set_x_axis_title(String::from("Temperature (C)"));
    multi_bubble.axis_prop.set_y_axis_title(String::from("Sales (£)"));
    multi_bubble.draw();

    let data_4 = data_parser::get_num_col(3, 0, 5, "./resources/bubble_chart_tests.csv");
    let data_5 = data_parser::get_num_col(4, 0, 5, "./resources/bubble_chart_tests.csv");
    let data_6 = data_parser::get_num_col(5, 0, 5, "./resources/bubble_chart_tests.csv");

   let mut multi_bubble = chart_builder::BubbleChart::new(String::from("Test Bubble Chart 2"),
       vec![data_4.clone()],
       vec![data_5.clone()],
       vec![data_6.clone()]
        );
    multi_bubble.axis_prop.set_x_axis_title(String::from("Temperature (C)"));
    multi_bubble.axis_prop.set_y_axis_title(String::from("Sales (£)"));
    multi_bubble.draw();

    let data_7 = data_parser::get_num_col(6, 0, 5, "./resources/bubble_chart_tests.csv");
    let data_8 = data_parser::get_num_col(7, 0, 5, "./resources/bubble_chart_tests.csv");
    let data_9 = data_parser::get_num_col(8, 0, 5, "./resources/bubble_chart_tests.csv");

   let mut multi_bubble = chart_builder::BubbleChart::new(String::from("Test Bubble Chart 3"),
       vec![data_4.clone(), data_7.clone()],
       vec![data_5.clone(), data_8.clone()],
       vec![data_6.clone(), data_9.clone()]
        );
    multi_bubble.axis_prop.set_x_axis_title(String::from("Temperature (C)"));
    multi_bubble.axis_prop.set_y_axis_title(String::from("Sales (£)"));
    multi_bubble.draw();

    let data_10 = data_parser::get_num_col(9, 0, 5, "./resources/bubble_chart_tests.csv");
    let data_11 = data_parser::get_num_col(10, 0, 5, "./resources/bubble_chart_tests.csv");
    let data_12 = data_parser::get_num_col(11, 0, 5, "./resources/bubble_chart_tests.csv");
    let data_13= data_parser::get_num_col(12, 0, 5, "./resources/bubble_chart_tests.csv");
    let data_14 = data_parser::get_num_col(13, 0, 5, "./resources/bubble_chart_tests.csv");
    let data_15 = data_parser::get_num_col(14, 0, 5, "./resources/bubble_chart_tests.csv");
    let data_16 = data_parser::get_num_col(15, 0, 5, "./resources/bubble_chart_tests.csv");
    let data_17 = data_parser::get_num_col(16, 0, 5, "./resources/bubble_chart_tests.csv");
    let data_18 = data_parser::get_num_col(17, 0, 5, "./resources/bubble_chart_tests.csv");

   let mut multi_bubble = chart_builder::BubbleChart::new(String::from("Test Bubble Chart 4"),
        vec![data_10.clone(), data_13.clone(), data_16.clone()],
        vec![data_11.clone(), data_14.clone(), data_17.clone()],
        vec![data_12.clone(), data_15.clone(), data_18.clone()]
        );
    multi_bubble.axis_prop.set_x_axis_title(String::from("Temperature (C)"));
    multi_bubble.axis_prop.set_y_axis_title(String::from("Sales (£)"));
    multi_bubble.chart_prop.set_legend_values(vec![String::from("Location 1"), String::from("Location 2"), String::from("Location 3")]);
    multi_bubble.chart_prop.set_show_legend(true);
    multi_bubble.draw();
}
