//! Provides functionality for extracting data from CSV files,
//! along with additional functionality for manipulation and cleansing of extracted data.
//!
//! This module's functions takes, processes and returns Vectors of Strings or f64(floats).
//! This means there is no restriction on manipulation and visability of data for the user.

extern crate csv;
extern crate regex;

fn vec_str_to_float(string_vec: Vec<String>) -> Vec<f64> {
    // Convert values in vector from strings to floats
    let num_vec_result: Result<Vec<f64>, _> = string_vec.iter().map(|x| x.parse()).collect();
    let num_vec = num_vec_result.expect("There was an error converting row values from string to float");
    // Returns vector of numbers
    num_vec
}

/// Returns a row of string data from a specified CSV file as a ```Vec<String>```.
///
/// ```row_num``` specifies the the row of data to take from the csv file.
///
/// ```start``` specifies the position of the first element in the row to be read.
///
/// ```end``` specifies the position of the last element in the row to be read.
///
/// ```file_name``` specifies the CSV file to read data from.
pub fn get_str_row(row_num: usize, start: usize, end: usize, file_name: &str) -> Vec<String> {
    // Creates a reader for the CSV file specified
    let mut rdr = csv::Reader::from_file(file_name).unwrap();
    // Get Nth line from CSV file
    let result = rdr.records().nth(row_num);
    let row = match result.expect("There was an error reading specified row") {
        Ok(r) => r,
        Err(error) => {
            panic!("The specified row does not exist: {:?}", error)
        },
    };
    // Takes only elements of rows specified
    let row = row[start..end].to_vec();
    // Returns row of strings
    row
}

/// Returns a row of numerical data from a specified CSV file as a ```Vec<f64>```.
///
/// ```row_num``` specifies the the row of data to take from the csv file.
///
/// ```start``` specifies the position of the first element in the row to be read.
///
/// ```end``` specifies the position of the last element in the row to be read.
///
/// ```file_name``` specifies the CSV file to read data from.
pub fn get_num_row(row_num: usize, start: usize, end: usize, file_name: &str) -> Vec<f64> {
    // Fetch specified row as Vector of strings
    let string_row = get_str_row(row_num, start, end, file_name);
    let num_row = vec_str_to_float(string_row);
    // Returns row of numbers
    num_row
}

/// Returns a column of string data from a specified CSV file as a ```Vec<String>```.
///
/// ```col_num``` specifies the the column of data to take from the csv file.
///
/// ```start``` specifies the position of the first element in the column to be read.
///
/// ```end``` specifies the position of the last element in the column to be read.
///
/// ```file_name``` specifies the CSV file to read data from.
pub fn get_str_col(col_num: usize, start: usize, end: usize, file_name: &str) -> Vec<String> {
    // Creates a reader for the CSV file specified
    let mut rdr = csv::Reader::from_file(file_name).unwrap();
    // Creates an iterator for all records
    let full_result_iter = rdr.records();
    // Trims iterator leaving records within specified bounds
    let spec_result_iter = full_result_iter.take(end).skip(start);
    // Takes the element from each record in iterator and adds them
    // to a Vector one by one producing column of values specified
    let mut col = Vec::new();
    for result in spec_result_iter {
        let record = result.expect("There was an error reading a specified row");
        let val = record[col_num].clone();
        col.push(val);
    }
    // Return collmn of values
    col
}

/// Returns a column of numerical data from a specified CSV file as a ```Vec<f64>```.
///
/// ```col_num``` specifies the the column of data to take from the csv file.
///
/// ```start``` specifies the position of the first element in the column to be read.
///
/// ```end``` specifies the position of the last element in the column to be read.
///
/// ```file_name``` specifies the CSV file to read data from.
pub fn get_num_col(row_num: usize, start: usize, end: usize, file_name: &str) -> Vec<f64> {
    // Fetch specified column as Vector of strings
    let string_col = get_str_col(row_num, start, end, file_name);
    let num_col = vec_str_to_float(string_col);
    // Returns column of numbers
    num_col
}

/// Returns the headers (top row) of string data from a specified CSV file as a ```Vec<String>```.
///
/// ```start``` specifies the position of the first element in the headers to be read.
///
/// ```end``` specifies the position of the last element in the headers to be read.
///
/// ```file_name``` specifies the CSV file to read data from.
pub fn get_headers(start: usize, end: usize, file_name: &str) -> Vec<String> {
    // Creates a reader for the CSV file specified
    let mut rdr = csv::Reader::from_file(file_name).unwrap();
    // Get header line from CSV file
    let result = rdr.headers();
    let headers = match result {
        Ok(r) => r,
        Err(error) => {
            panic!("Headers do not exist: {:?}", error)
        },
    };
    // Takes only elements of rows specified
    let headers = headers[start..end].to_vec();
    // Returns header of strings
    headers
}

/// Performs a specified numerical transformation on each element of a ```Vec<f64>```.
///
/// ```vec1``` is the vector to be transformed.
///
/// ```trans_func``` is the transformation function applied to each element of the vector.
///
/// # Examples
///
/// ```should_panic
/// use rustplot::data_parser;
///
/// let num_row = data_parser::get_num_row(0, 0, 10, "./test.csv");
///
/// fn transform(num1: f64) -> f64 {
///     num1 + 10.0
/// }
/// let f: fn(f64) -> f64 = transform;
///
/// let new_vec = data_parser::vec_num_transform(&num_row, f);
/// ```
pub fn vec_num_transform(vec1: &Vec<f64>, trans_func: fn(f64) -> f64) -> Vec<f64> {
     vec1.iter().map(|x| trans_func(*x)).collect()
 }

 /// Transforms all vectors elements to their log base 10 values.
 ///
 /// ```vec1``` is the vector to be transformed.
pub fn vec_log(vec1: &Vec<f64>) -> Vec<f64> {
    fn log_f(num1: f64) -> f64 {
        num1.log(10.0)
    }
    let f: fn(f64) -> f64 = log_f;
    vec_num_transform(vec1, f)
 }

 /// Transforms all vectors elements to the natural log of their values.
 ///
 /// ```vec1``` is the vector to be transformed.
pub fn vec_ln(vec1: &Vec<f64>) -> Vec<f64> {
    fn ln_f(num1: f64) -> f64 {
     num1.ln()
    }

    let f: fn(f64) -> f64 = ln_f;
    vec_num_transform(vec1, f)
 }

 /// Performs a specified numerical transformation on corresponding elements of two ```Vec<f64>```s.
 ///
 /// ```vec1``` is the vector to be transformed.
 ///
 /// ```trans_func``` is the transformation function applied to each element of both vectors.
 ///
 /// # Examples
 ///
 /// ```should_panic
 /// use rustplot::data_parser;
 ///
 /// let num_row_1 = data_parser::get_num_row(0, 0, 10, "./test.csv");
 /// let num_row_2 = data_parser::get_num_row(1, 0, 10, "./test.csv");
 ///
 /// fn transform(num1: f64, num2: f64) -> f64 {
 ///     (num1 + num2) / 2.0
 /// }
 /// let f: fn(f64, f64) -> f64 = transform;
 ///
 /// let new_vec = data_parser::vecs_num_transform(&num_row_1, &num_row_2, f);
 /// ```
pub fn vecs_num_transform(vec1: &Vec<f64>, vec2: &Vec<f64>, trans_func: fn(f64, f64) -> f64) -> Vec<f64> {
     if vec1.len() != vec2.len() {
         panic!("Vectors must have the same number of elements");
     }

     let mut res_vec = Vec::new();
     for i in 0..vec1.len() {
         let ele1 = vec1[i];
         let ele2 = vec2[i];

         let res = trans_func(ele1, ele2);

         res_vec.push(res);
     }

     res_vec
}

/// Returns a vector with elements with values of the sum of the corresponding elements of the two provided vectors.
pub fn vec_add(vec1: &Vec<f64>, vec2: &Vec<f64>) -> Vec<f64> {
    fn add(num1: f64, num2: f64) -> f64 {
        num1 + num2
    }
    let f: fn(f64, f64) -> f64 = add;
    vecs_num_transform(vec1, vec2, f)
}

/// Returns a vector with elements with values of the difference between the corresponding elements of the two provided vectors.
///
/// ```vec1``` is the vector whos elements will be the minuend.
///
/// ```vec2``` is the vector whos elements will be the subtrahend.
pub fn vec_sub(vec1: &Vec<f64>, vec2: &Vec<f64>) -> Vec<f64> {
    fn sub(num1: f64, num2: f64) -> f64 {
        num1 - num2
    }
    let f: fn(f64, f64) -> f64 = sub;
    vecs_num_transform(vec1, vec2, f)
}

/// Returns a vector with elements with values of the product of the corresponding elements of the two provided vectors.
pub fn vec_mul(vec1: &Vec<f64>, vec2: &Vec<f64>) -> Vec<f64> {
    fn mul(num1: f64, num2: f64) -> f64 {
        num1 * num2
    }
    let f: fn(f64, f64) -> f64 = mul;
    vecs_num_transform(vec1, vec2, f)
}

/// Returns a vector with elements with values of the quotient of the corresponding elements of the two provided vectors.
///
/// ```vec1``` is the vector whos elements will be the dividend.
///
/// ```vec2``` is the vector whos elements will be the divisor.
pub fn vec_div(vec1: &Vec<f64>, vec2: &Vec<f64>) -> Vec<f64> {
    fn div(num1: f64, num2: f64) -> f64 {
        num1 / num2
    }
    let f: fn(f64, f64) -> f64 = div;
    vecs_num_transform(vec1, vec2, f)
}

/// Returns a vector with elements with values of the sum of the corresponding elements of the two provided vectors.
///
/// ```vec1``` is the vector whos elements will be the base.
///
/// ```vec2``` is the vector whos elements will be the exponent.
pub fn vec_pow(vec1: &Vec<f64>, vec2: &Vec<f64>) -> Vec<f64> {
    fn pow(num1: f64, num2: f64) -> f64 {
        num1.powf(num2)
    }
    let f: fn(f64, f64) -> f64 = pow;
    vecs_num_transform(vec1, vec2, f)
}

/// Returns a Vec<usize> (integer vector) with the index of each element matching the predicate function provided.
///
/// ```vec1``` is a numerical vector for which each element will be checked with the predicate function.
///
/// ```pred_func``` is the predicate function applied to each element of the vector.
///
/// This function is designed for use with ```vec_remove_where()``` and ```vec_keep_where()```.
///
/// # Examples
///
/// ```should_panic
/// use rustplot::data_parser;
///
/// let num_row = data_parser::get_num_row(0, 0, 10, "./test.csv");
///
/// fn pred(num1: f64) -> bool {
///     num1 <= 7.0
/// }
/// let f: fn(f64) -> bool = pred;
///
/// let matching_indexes = data_parser::num_pred(&num_row, f);
/// ```
pub fn num_pred(vec1: &Vec<f64>, pred_func: fn(f64) -> bool) -> Vec<usize> {
    let mut match_vec = Vec::new();
    for i in 0..vec1.len() {
        let ele1 = vec1[i];

        let pred = pred_func(ele1);

        if pred {
            match_vec.push(i);
        }
    }
    match_vec
}

/// Returns a Vec<usize> (string vector) with the index of each element matching the regular expression provided.
///
/// ```vec1``` is a string vector for which each element will be checked against the regular expression.
///
/// ```regex_string``` is the regular expression applied to each element of the vector.
///
/// This function is designed for use with ```vec_remove_where()``` and ```vec_keep_where()```.
pub fn reg_match(vec1: &Vec<String>, regex_string: &str) -> Vec<usize> {
    let mut match_vec = Vec::new();
    for i in 0..vec1.len() {
        use self::regex::Regex;
        let re = Regex::new(regex_string).unwrap();

        let ele1: &str = &vec1[i];
        let pred = re.is_match(ele1);

        if pred {
            match_vec.push(i);
        }
    }
    match_vec
}

/// Returns a copy of the provided vector with elements specified by a list of indexes removed.
///
/// ```vec1``` can be any vector implementing the Clone Trait, which will have elements removed from it.
///
/// ```index_vec``` is a list of integers specifying the elements that should be removed.
///
/// This function is designed for use with ```num_pred()``` and ```reg_match()```.
pub fn vec_remove_where<T: Clone>(vec1: &Vec<T>, index_vec:  &Vec<usize>) -> Vec<T> {
    let mut mut_index_vec = index_vec.clone();
    mut_index_vec.reverse();

    let mut keep_vec = vec1.to_vec();

    for i in 0..mut_index_vec.len() {
        keep_vec.remove(mut_index_vec[i]);
    }
    keep_vec
}

/// Returns a copy of the provided vector only keeping elements specified by a list of indexes.
///
/// ```vec1``` can be any vector implementing the Clone Trait, which will have specified elements kept.
///
/// ```index_vec``` is a list of integers specifying the elements that should be kept.
///
/// This function is designed for use with ```num_pred()``` and ```reg_match()```.
pub fn vec_keep_where<T: Clone>(vec1: &Vec<T>, index_vec:  &Vec<usize>) -> Vec<T> {
    let mut keep_vec = Vec::new();
    for i in 0..index_vec.len() {
        keep_vec.push(vec1[index_vec[i]].clone());
    }
    keep_vec
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_tests() {
        let vec_1: Vec<f64> = vec![3.000000000000000,5.000000000000000,4.000000000000000,0.010000000000000,0.050000000000000,2000.000000000000000,10000.000000000000000];
        let vec_2: Vec<f64> = vec![8.000000000000000,2.000000000000000,4.000000000000000,0.060000000000000,1000.000000000000000,0.100000000000000,10000.000000000000000];
        let add_result = vec![11.000000000000000,7.000000000000000,8.000000000000000,0.06999999999999999,1000.050000000000000,2000.100000000000000,20000.000000000000000];
        assert_eq!(vec_add(&vec_1, &vec_2), add_result);
    }
    #[test]
    fn sub_tests() {
        let vec_1: Vec<f64> = vec![3.000000000000000,5.000000000000000,4.000000000000000,0.010000000000000,0.050000000000000,2000.000000000000000,10000.000000000000000];
        let vec_2: Vec<f64> = vec![8.000000000000000,2.000000000000000,4.000000000000000,0.060000000000000,1000.000000000000000,0.100000000000000,10000.000000000000000];
        let sub_result = vec![-5.000000000000000,3.000000000000000,0.000000000000000,-0.049999999999999996,-999.950000000000000,1999.900000000000000,0.000000000000000];
        assert_eq!(vec_sub(&vec_1, &vec_2), sub_result);
    }
    #[test]
    fn mul_tests() {
        let vec_1: Vec<f64> = vec![3.000000000000000,5.000000000000000,4.000000000000000,0.010000000000000,0.050000000000000,2000.000000000000000,10000.000000000000000];
        let vec_2: Vec<f64> = vec![8.000000000000000,2.000000000000000,4.000000000000000,0.060000000000000,1000.000000000000000,0.100000000000000,10000.000000000000000];
        let mul_result = vec![24.000000000000000,10.000000000000000,16.000000000000000,0.000600000000000,50.000000000000000,200.000000000000000,100000000.000000000000000];
        assert_eq!(vec_mul(&vec_1, &vec_2), mul_result);
    }
    #[test]
    fn div_tests() {
        let vec_1: Vec<f64> = vec![3.000000000000000,5.000000000000000,4.000000000000000,0.010000000000000,0.050000000000000,2000.000000000000000,10000.000000000000000];
        let vec_2: Vec<f64> = vec![8.000000000000000,2.000000000000000,4.000000000000000,0.060000000000000,1000.000000000000000,0.100000000000000,10000.000000000000000];
        let div_result = vec![0.375000000000000,2.500000000000000,1.000000000000000,0.16666666666666669,0.000050000000000,20000.000000000000000,1.000000000000000];
        assert_eq!(vec_div(&vec_1, &vec_2), div_result);
    }
    #[test]
    fn pow_tests() {
        let vec_1: Vec<f64> = vec![3.000000000000000,5.000000000000000,4.000000000000000,0.010000000000000,0.050000000000000,2000.000000000000000,10000.000000000000000];
        let vec_2: Vec<f64> = vec![8.000000000000000,2.000000000000000,4.000000000000000,0.060000000000000,1000.000000000000000,0.100000000000000,10000.000000000000000];
        use std::f64;
        let pow_result = vec![6561.000000000000000,25.000000000000000,256.000000000000000,0.7585775750291838,0.000000000000000,2.138469199982376,f64::INFINITY];
        assert_eq!(vec_pow(&vec_1, &vec_2), pow_result);
    }
    #[test]
    fn log_tests() {
        let vec_1: Vec<f64> = vec![3.000000000000000,5.000000000000000,4.000000000000000,0.010000000000000,0.050000000000000,2000.000000000000000,10000.000000000000000];
        let log_result = vec![0.47712125471966244,0.6989700043360187,0.6020599913279623,-1.9999999999999996,-1.301029995663981,3.301029995663981,4.000000000000000];
        assert_eq!(vec_log(&vec_1), log_result);
    }
    #[test]
    fn ln_tests() {
        let vec_1: Vec<f64> = vec![3.000000000000000,5.000000000000000,4.000000000000000,0.010000000000000,0.050000000000000,2000.000000000000000,10000.000000000000000];
        let ln_result = vec![1.0986122886681098,1.6094379124341003,1.3862943611198906,-4.605170185988091,-2.995732273553991,7.600902459542082,9.210340371976184];
        assert_eq!(vec_ln(&vec_1), ln_result);
    }
    #[test]
    fn trans_vec_tests() {
        let vec_1: Vec<f64> = vec![16.0,5.0,4.0,0.01,0.5,2000.0,10000.0];
        let result = vec![26.0,15.0,14.0,10.01,10.5,2010.0,10010.0];
        fn test_trans(num1: f64) -> f64 {
            num1 + 10.0
        }
        let f: fn(f64) -> f64 = test_trans;
        let trans_res = vec_num_transform(&vec_1, f);
        assert_eq!(trans_res, result);
    }
    #[test]
    fn trans_vecs_tests() {
        let vec_1: Vec<f64> = vec![16.0,5.0,4.0,0.01,0.5,2000.0,10000.0];
        let vec_2: Vec<f64> = vec![8.0,3.0,4.0,0.06,1000.0,0.1,10000.0];
        let result = vec![12.0,4.0,4.0,0.034999999999999996,500.25,1000.05,10000.0];
        fn test_trans(num1: f64, num2: f64) -> f64 {
            (num1 + num2) / 2.0
        }
        let f: fn(f64, f64) -> f64 = test_trans;
        let trans_res = vecs_num_transform(&vec_1, &vec_2, f);
        assert_eq!(trans_res, result);
    }
    #[test]
    fn num_pred_tests() {
        let vec_1: Vec<f64> = vec![2.0, 10.0, 7.0, 8.0, 4.0];
        let result = vec![0, 2, 4];
        fn test_pred(num1: f64) -> bool {
            num1 <= 7.0
        }
        let f: fn(f64) -> bool = test_pred;
        let pred_res = num_pred(&vec_1, f);
        assert_eq!(pred_res, result);
    }
    #[test]
    fn reg_match_tests() {
        let vec_1: Vec<String> = vec![String::from("123456789"), String::from("12345"), String::from("123567"), String::from("6830498"), String::from("5937468")];
        let result = vec![3, 4];
        let reg_res = reg_match(&vec_1, r"^.*(68)+.*$");
        assert_eq!(reg_res, result);

    }
    #[test]
    fn remove_where_tests() {
        let vec_1: Vec<f64> = vec![2.0, 7.0, 6.0, 4.0, 10.0];
        let index_1 = vec![0, 2, 3];
        let result_1 = vec![7.0, 10.0];
        let rm_res_1 = vec_remove_where(&vec_1, &index_1);
        assert_eq!(rm_res_1, result_1);

        let vec_2: Vec<String> = vec![String::from("123456789"), String::from("12345"), String::from("123567"), String::from("6830498"), String::from("5937468")];
        let index_2 = vec![0, 2, 3];
        let result_2 = vec![String::from("12345"), String::from("5937468")];
        let rm_res_2 = vec_remove_where(&vec_2, &index_2);
        assert_eq!(rm_res_2, result_2);
    }
    #[test]
    fn keep_where_tests() {
        let vec_1: Vec<f64> = vec![2.0, 7.0, 6.0, 4.0, 10.0];
        let index_1 = vec![0, 2, 3];
        let result_1 = vec![2.0, 6.0, 4.0];
        let kp_res_1 = vec_keep_where(&vec_1, &index_1);
        assert_eq!(kp_res_1, result_1);

        let vec_2: Vec<String> = vec![String::from("123456789"), String::from("12345"), String::from("123567"), String::from("6830498"), String::from("5937468")];
        let index_2 = vec![0, 2, 3];
        let result_2 = vec![String::from("123456789"), String::from("123567"), String::from("6830498")];
        let kp_res_2 = vec_keep_where(&vec_2, &index_2);
        assert_eq!(kp_res_2, result_2);
    }
    #[test]
    fn get_str_row_tests() {
        let result = vec![String::from("75"),String::from("16"),String::from("15"),String::from("96")];
        let num_row_res = get_str_row(2, 1, 5, "./resources/data_parser_tests.csv");
        assert_eq!(num_row_res, result);
    }
    #[test]
    fn get_str_col_tests() {
        let result = vec![String::from("6"),String::from("33"),String::from("15"),String::from("40"),String::from("48")];
        let num_col_res = get_str_col(3, 0, 5, "./resources/data_parser_tests.csv");
        assert_eq!(num_col_res, result);
    }
    #[test]
    fn get_num_row_tests() {
        let result = vec![75.0,16.0,15.0,96.0];
        let num_row_res = get_num_row(2, 1, 5, "./resources/data_parser_tests.csv");
        assert_eq!(num_row_res, result);
    }
    #[test]
    fn get_num_col_tests() {
        let result = vec![6.0,33.0,15.0,40.0,48.0];
        let num_col_res = get_num_col(3, 0, 5, "./resources/data_parser_tests.csv");
        assert_eq!(num_col_res, result);
    }
    #[test]
    fn get_headers_tests() {
        let result = vec![String::from("h2"), String::from("h3"), String::from("h4")];
        let headers_res = get_headers(1, 4, "./resources/data_parser_tests.csv");
        assert_eq!(headers_res, result);
    }
}
