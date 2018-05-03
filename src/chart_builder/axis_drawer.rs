//! Generic axis drawing methods


use chart_builder::*;


pub(in chart_builder) fn calc_zero_intercept(axis_min: f64, axis_max: f64) -> f64 {
    get_percentage_in_bounds(0.0, axis_min, axis_max)
}

pub(in chart_builder) fn calc_x_intercept(zero_intercept: f64, _vertical_scaling: f64, _lower_bound: f64, _upper_bound: f64) -> f64 {
    if zero_intercept < 0.0 {
        _lower_bound
    } else if zero_intercept > 1.0 {
        _upper_bound
    } else {
        _lower_bound - zero_intercept * _vertical_scaling
    }
}

pub(in chart_builder) fn calc_y_intercept(zero_intercept: f64, _horizontal_scaling: f64, _left_bound: f64, _right_bound: f64) -> f64 {
    if zero_intercept < 0.0 {
        _left_bound
    } else if zero_intercept > 1.0 {
        _right_bound
    } else {
        _left_bound + zero_intercept * _horizontal_scaling
    }
}

// Responsible for drawing a catagorical x-axis
pub(in chart_builder) fn draw_x_axis_cat(cr: &Context, scalings: (f64, f64, f64, f64 ,f64, f64),
        data_labels: &Vec<String>, x_axis_scale: f64, zero_intercept: f64, axis_title: &String,
        screen_size: (f64, f64), fill: bool) {
        let _horizontal_scaling = scalings.0;
        let _vertical_scaling = scalings.1;
        let _left_bound = scalings.2;
        let _right_bound = scalings.3;
        let _lower_bound = scalings.4;
        let _upper_bound = scalings.5;

    let intercept = calc_x_intercept(zero_intercept, _vertical_scaling, _lower_bound, _upper_bound);

    let mut h_scale = screen_size.1 / screen_size.0;
    let mut v_scale = screen_size.0 / screen_size.1;

    // Always make text and objects smaller rather than bigger as guarnteed to fit on screen
    if h_scale < v_scale {
        v_scale = 1.0;
    } else {
        h_scale = 1.0;
    }

    let x_delimiter_interval: f64 = _horizontal_scaling * x_axis_scale;
    let x_delimiter_length = 0.015 * v_scale;

    cr.set_source_rgb(0.0, 0.0, 0.0);

    // Display x-axis full line
    cr.set_line_width(0.002 * v_scale);
    cr.move_to(_left_bound, intercept);
    cr.rel_line_to(_horizontal_scaling, 0.0);
    cr.stroke();
    // Display x-axis delimiters - catagorical
    cr.set_line_width(0.002 * h_scale);
    for i in 0..data_labels.len() {
        if fill == false {
            cr.move_to(_left_bound - (x_delimiter_interval / 2.0) + x_delimiter_interval * ((i + 1) as f64), intercept - (x_delimiter_length / 2.0));
        } else {
            cr.move_to(_left_bound + x_delimiter_interval * (i as f64), intercept - (x_delimiter_length / 2.0));
        }
        cr.rel_line_to(0.0, x_delimiter_length);
    }
    cr.stroke();

    // Display x-axis strings - catagorical
    cr.set_font_size(0.02);
    let mut font_matrix = cr.get_font_matrix();
    font_matrix.scale(h_scale, v_scale);
    cr.set_font_matrix(font_matrix);

    let num_labels = data_labels.len();
    for i in 0..num_labels {
        let mut axis_string = data_labels[i].clone();
        // stores the partial string as the characters are popped off
        let mut axis_string_temp = axis_string.clone();
        loop {
            // create a variable that that does not require borrowing of axis_string
            let mut fits = true;
            {
                let axis_str = axis_string.as_str();
                let text_width = cr.text_extents(axis_str).width;
                if text_width > x_delimiter_interval {fits = false}
            }

            if !fits {
                axis_string_temp.pop();
                axis_string = axis_string_temp.clone();
                axis_string.push_str("...");
            } else {
                break;
            }
        }

        let axis_str = axis_string.as_str();
        let text_height = cr.text_extents(axis_str).height;
        let text_width = cr.text_extents(axis_str).width;


        // flip the side that the text is on
        let axis_side_disp: f64;
        if zero_intercept < 0.55 {
            axis_side_disp = x_delimiter_length + text_height;
        } else {
            axis_side_disp = - x_delimiter_length;
        }

        let mut x_pos = 0.;
        if fill == false {
            x_pos += _left_bound - (x_delimiter_interval / 2.0) + x_delimiter_interval * ((i + 1) as f64);
        } else {
            x_pos += _left_bound + x_delimiter_interval * (i as f64);
        }

        x_pos -= text_width / 2.0;

        cr.move_to(x_pos, intercept + axis_side_disp);
        cr.show_text(axis_str);
    }

    // draw x axis titles
    let axis_title_str = axis_title.as_str();
    font_matrix.scale(1.1, 1.1);
    cr.set_font_matrix(font_matrix);
    let axis_title_width = cr.text_extents(axis_title_str).width;
    let axis_title_height = cr.text_extents(axis_title_str).height;

    cr.move_to(_right_bound - axis_title_width, _lower_bound + x_delimiter_length * 1.5 + axis_title_height * 2.0); // + 0.1 * (_vertical_scaling)
    cr.show_text(axis_title_str);
}

// Responsible for drawing a continious x-axis
pub(in chart_builder) fn draw_x_axis_con(cr: &Context, scalings: (f64, f64, f64, f64 ,f64, f64),
        x_axis_min: f64, x_axis_max: f64, x_axis_scale: f64, zero_intercept: f64, axis_title: &String,
        screen_size: (f64, f64)) {
        let _horizontal_scaling = scalings.0;
        let _vertical_scaling = scalings.1;
        let _left_bound = scalings.2;
        let _right_bound = scalings.3;
        let _lower_bound = scalings.4;
        let _upper_bound = scalings.5;

    let intercept = calc_x_intercept(zero_intercept, _vertical_scaling, _lower_bound, _upper_bound);
    let other_zero_intercept = calc_zero_intercept(x_axis_min, x_axis_max);
    let other_intercept = calc_y_intercept(other_zero_intercept, _horizontal_scaling, _left_bound, _right_bound);

    let mut h_scale = screen_size.1 / screen_size.0;
    let mut v_scale = screen_size.0 / screen_size.1;

    // Always make text and objects smaller rather than bigger as guarnteed to fit on screen
    if h_scale < v_scale {
        v_scale = 1.0;
    } else {
        h_scale = 1.0;
    }

    let x_num_delimiters = ((1.0/x_axis_scale).round() as usize) + 1;
    let x_delimiter_interval: f64 = _horizontal_scaling * x_axis_scale;
    let x_delimiter_length = 0.015 * v_scale;
    let y_delimiter_length = 0.015 * h_scale;

    cr.set_source_rgb(0.0, 0.0, 0.0);

    // Display x-axis full line
    cr.set_line_width(0.002 * v_scale);
    cr.move_to(_left_bound, intercept);
    cr.rel_line_to(_horizontal_scaling, 0.0);
    cr.stroke();
    // Display x-axis delimiters - continious
    cr.set_line_width(0.002 * h_scale);
    for i in 0..x_num_delimiters {
        cr.move_to(_left_bound + x_delimiter_interval * (i as f64), intercept - (x_delimiter_length / 2.0));
        cr.rel_line_to(0.0, x_delimiter_length);
    }
    cr.stroke();

    // Display x-axis strings - continious
    cr.set_font_size(0.02);
    let mut font_matrix = cr.get_font_matrix();
    font_matrix.scale(h_scale, v_scale);
    cr.set_font_matrix(font_matrix);

    let x_dps: usize;
    if x_axis_max.abs().max(x_axis_min.abs()) <= 0.1 { x_dps = 4; }
    else if x_axis_max.abs().max(x_axis_min.abs()) >= 100.0  { x_dps = 0; }
    else { x_dps = 2; }

    let e_format: bool;
    if x_axis_max.abs().max(x_axis_min.abs()) >= 10000.0 || x_axis_max.abs().max(x_axis_min.abs()) <= 0.001 { e_format = true } else { e_format = false }

    for i in 0..x_num_delimiters {
        let axis_num = x_axis_min + ((x_axis_max - x_axis_min) * x_axis_scale * (i as f64));

        let mut axis_num_string: String;
        if e_format == true {
            axis_num_string = format!("{:e}", format!("{:.*}", 15, axis_num).parse::<f64>().unwrap() ).to_string();
        } else {
            axis_num_string = format!("{:.*}", x_dps, axis_num).to_string();
        }

        let axis_num_str = axis_num_string.as_str();
        let text_width = cr.text_extents(axis_num_str).width;
        let text_height = cr.text_extents(axis_num_str).height;

        let mut x_pos = _left_bound + x_delimiter_interval * (i as f64);
        // avoid clash with axis
        if x_pos < other_intercept + 0.01 && x_pos > other_intercept - 0.01 {
            // move by amount based on where it was meant to be originally
            if other_zero_intercept < 0.55 {
                x_pos += (y_delimiter_length / 2.0) + (x_pos - other_intercept);
            } else {
                x_pos += - text_width - (y_delimiter_length / 2.0) + (other_intercept - x_pos);
            }
        } else {
            x_pos -= text_width / 2.0
        }
        // flip the side that the text is on
        let axis_side_disp: f64;
        if zero_intercept < 0.55 {
            axis_side_disp = x_delimiter_length + text_height;
        } else {
            axis_side_disp = - x_delimiter_length;
        }

        cr.move_to(x_pos, intercept + axis_side_disp);
        cr.show_text(axis_num_str);
    }

    // draw x axis titles
    let axis_title_str = axis_title.as_str();
    font_matrix.scale(1.1, 1.1);
    cr.set_font_matrix(font_matrix);

    let axis_title_width = cr.text_extents(axis_title_str).width;
    let axis_title_height = cr.text_extents(axis_title_str).height;

    cr.move_to(_right_bound - axis_title_width, _lower_bound + x_delimiter_length * 1.5 + axis_title_height * 2.0); // + 0.1 * (_vertical_scaling)
    cr.show_text(axis_title_str);
}

// Responsible for drawing a continious y-axis
pub(in chart_builder) fn draw_y_axis_con(cr: &Context, scalings: (f64, f64, f64, f64 ,f64, f64),
        y_axis_min: f64, y_axis_max: f64, y_axis_scale: f64, zero_intercept: f64, axis_title: &String,
        screen_size: (f64, f64)) {
        let _horizontal_scaling = scalings.0;
        let _vertical_scaling = scalings.1;
        let _left_bound = scalings.2;
        let _right_bound = scalings.3;
        let _lower_bound = scalings.4;
        let _upper_bound = scalings.5;

    let intercept = calc_y_intercept(zero_intercept, _horizontal_scaling, _left_bound, _right_bound);
    let other_zero_intercept = calc_zero_intercept(y_axis_min, y_axis_max);
    let other_intercept = calc_x_intercept(other_zero_intercept, _vertical_scaling, _lower_bound, _upper_bound);

    let mut h_scale = screen_size.1 / screen_size.0;
    let mut v_scale = screen_size.0 / screen_size.1;

    // Always make text and objects smaller rather than bigger as guarnteed to fit on screen
    if h_scale < v_scale {
        v_scale = 1.0;
    } else {
        h_scale = 1.0;
    }

    let y_num_delimiters = ((1.0/y_axis_scale).round() as usize) + 1;
    let y_delimiter_interval: f64 = _vertical_scaling * y_axis_scale;
    let y_delimiter_length = 0.015 * h_scale;

    cr.set_source_rgb(0.0, 0.0, 0.0);

    // Display y-axis full line
    cr.set_line_width(0.002 * h_scale);
    cr.move_to(intercept, _lower_bound);
    cr.rel_line_to(0.0, -_vertical_scaling);
    cr.stroke();
    // Display y-axis delimiters
    cr.set_line_width(0.002 * v_scale);
    for i in 0..y_num_delimiters {
        cr.move_to(intercept - (y_delimiter_length / 2.0), _lower_bound - y_delimiter_interval * (i as f64));
        cr.rel_line_to(y_delimiter_length, 0.0);
    }
    cr.stroke();

    // Display y-axis numbers
    cr.set_font_size(0.02);
    let mut font_matrix = cr.get_font_matrix();
    font_matrix.scale(h_scale, v_scale);
    cr.set_font_matrix(font_matrix);

    let mut max_str: f64 = 0.0;

    let y_dps: usize;
    if y_axis_max.abs().max(y_axis_min.abs()) <= 0.1 { y_dps = 4; }
    else if y_axis_max.abs().max(y_axis_min.abs()) >= 100.0  { y_dps = 0; }
    else { y_dps = 2; }

    let e_format: bool;
    if y_axis_max.abs().max(y_axis_min.abs()) >= 10000.0 || y_axis_max.abs().max(y_axis_min.abs()) <= 0.001 { e_format = true } else { e_format = false }

    for i in 0..y_num_delimiters {
        let axis_num = y_axis_min + ((y_axis_max - y_axis_min) * y_axis_scale * (i as f64));

        let mut axis_num_string: String;
        if e_format == true {
            axis_num_string = format!("{:e}", format!("{:.*}", 15, axis_num).parse::<f64>().unwrap() ).to_string();
            if axis_num == 0.0 {
                axis_num_string = format!("{:.*}", 0, axis_num).to_string();
            }
        } else {
            axis_num_string = format!("{:.*}", y_dps, axis_num).to_string();
        }

        let axis_num_str = axis_num_string.as_str();
        let text_width = cr.text_extents(axis_num_str).width;
        let text_height = cr.text_extents(axis_num_str).height;

        // stops clashing labels where axis meet
        let mut y_pos = _lower_bound - y_delimiter_interval * (i as f64);
        if y_pos < other_intercept + 0.01 && y_pos > other_intercept - 0.01 {
            // move by amount based on where it was meant to be
            if other_zero_intercept < 0.55 {
                y_pos -= text_height - (y_pos - other_intercept);
            } else {
                y_pos += text_height - (other_intercept - y_pos);
            }
        }
        // flip side label is on
        let axis_side_disp: f64;
        if zero_intercept < 0.55 {
            axis_side_disp = - y_delimiter_length - text_width;
        } else {
            axis_side_disp = y_delimiter_length;
        }

        cr.move_to(intercept + axis_side_disp, y_pos + text_height / 2.0);
        cr.show_text(axis_num_str);
        // keep track of longest string to know when to axis title
        max_str = max_str.max(text_width);
    }

    // draw y axis titles
    let axis_title_str = axis_title.as_str();
    font_matrix.scale(1.1, 1.1);
    font_matrix.rotate(- 3.14159265359 * 0.5);
    cr.set_font_matrix(font_matrix);

    let axis_title_width = cr.text_extents(axis_title_str).width;
    let axis_title_height = cr.text_extents(axis_title_str).height;

    cr.move_to(_left_bound - y_delimiter_length * 0.5 - max_str - axis_title_width, _upper_bound + axis_title_height);
    cr.show_text(axis_title_str);
}
