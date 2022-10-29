use std::io;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    type HTMLDocument;
    type Element;
    static document: HTMLDocument;

    #[wasm_bindgen(method)]
    fn createElement(this: &HTMLDocument, tagName: &str) -> Element;

    #[wasm_bindgen(method, getter)]
    fn body(this: &HTMLDocument) -> Element;

    #[wasm_bindgen(method, js_name = appendChild)]
    fn append(this: &Element, item: Element);

    #[wasm_bindgen(method, setter = innerHTML)]
    fn set_inner(this: &Element, html: &str);
}

#[wasm_bindgen]
pub fn rust_calculate(expr: &str) -> f64 {
    // Declare used variables
    let mut expr = String::new();
    let mut int_a: i64 = 0;
    let mut int_b: i64 = 0;
    let mut int_result: i64 = 0;
    let mut float_a: f64 = 0.0;
    let mut float_b: f64 = 0.0;
    let mut float_result: f64 = 0.0;
    let mut op: &str;

    let line_split = expr.trim().split(" ");

    let mut stack: Vec<&str> = line_split.collect();

    // let mut count: usize = 0;
    let mut parsed_vec: Vec<f64> = Vec::new();

    for item in stack {
        /*
        // Error Checking
        println!(
            "is int({0}){1}, is float({0}){2}, is op({0}){3}",
            item,
            is_int(item.to_string()),
            is_float(item.to_string()),
            is_op_unary(item.to_string())
        );
        */

        if is_int(item.to_string()) {
            // Populate Stack
            parsed_vec.push(item.trim().parse().expect("Invalid Integer"));
        } else if is_float(item.to_string()) {
            // Populate Stack
            parsed_vec.push(item.trim().parse().expect("Invalid Float"));
        } else if is_op_unary(item.to_string()) {
            // Get operand & operator
            op = item.trim();
            int_a = int_value(parsed_vec.pop());
            // Calculate & push result to stack
            int_result = rust_calc_unary(int_a, op);
            parsed_vec.push(int_result as f64);
        } else if is_op_float(item.to_string()) {
            // Get operands & operator
            op = item.trim();
            float_a = float_value(parsed_vec.pop());
            float_b = float_value(parsed_vec.pop());
            // Calculate & push result to stack
            float_result = rust_calc_float(float_a, float_b, op);
            parsed_vec.push(float_result);
        } else if is_op_int(item.to_string()) {
            // Get operands & operator
            op = item.trim();
            int_a = int_value(parsed_vec.pop());
            int_b = int_value(parsed_vec.pop());
            // Calculate & push ressult to stack
            int_result = rust_calc_int(int_a, int_b, op);
            parsed_vec.push(int_result as f64);
        } else {
            panic!("Error: Invalid syntax!");
        }
        /*
        // Error Checking
        println!("{:?}", parsed_vec);
        */
    }

    // Print Final Result
    let final_result = float_value(parsed_vec.pop());
    if final_result.trunc() == final_result {
        return final_result.trunc();
    } else {
        return final_result;
    }
}

#[wasm_bindgen]
pub fn is_int(str: String) -> bool {
    let mut result: bool = false;
    for char in str.chars() {
        if char.is_numeric() {
            result = true;
        } else if char == '.' {
            result = false;
            break;
        } else {
            result = false;
            break;
        }
    }
    return result;
}
#[wasm_bindgen]
pub fn is_float(str: String) -> bool {
    let mut dec_point: i64 = 0;
    let mut result: bool = false;
    for char in str.chars() {
        if char.is_numeric() {
            result = true;
        } else if char == '.' {
            dec_point = dec_point + 1;
        } else {
            result = false;
            return result;
        }
    }
    if dec_point > 1 || dec_point < 1 {
        result = false;
    }
    return result;
}
#[wasm_bindgen]
pub fn is_op_int(str: String) -> bool {
    let mut result: bool = false;
    if str.eq("+") {
        result = true;
    } else if str.eq("*") {
        result = true;
    } else if str.eq("<<") {
        result = true;
    } else if str.eq("-") {
        result = true;
    } else if str.eq("/") {
        result = true;
    } else if str.eq(">>") {
        result = true;
    } else if str.eq("AND") {
        result = true;
    } else if str.eq("OR") {
        result = true;
    } else if str.eq("%") {
        result = true;
    }
    return result;
}
#[wasm_bindgen]
pub fn is_op_float(str: String) -> bool {
    let mut result: bool = false;
    if str.eq("+") {
        result = true;
    } else if str.eq("*") {
        result = true;
    } else if str.eq("-") {
        result = true;
    } else if str.eq("/") {
        result = true;
    }
    return result;
}
#[wasm_bindgen]
pub fn is_op_unary(str: String) -> bool {
    let mut result: bool = false;
    if str.eq("NOT") {
        result = true;
    } else if str.eq("NEG") {
        result = true;
    }
    return result;
}
#[wasm_bindgen]
pub fn rust_calc_int(value_a: i64, value_b: i64, operation: &str) -> i64 {
    let mut result: i64 = 0;

    if operation.starts_with("+") {
        result = value_a + value_b;
    } else if operation.starts_with("*") {
        result = value_a * value_b;
    } else if operation.starts_with("<<") {
        result = value_a << value_b;
    } else if operation.starts_with("-") {
        result = value_a - value_b;
    } else if operation.starts_with("/") {
        result = value_a / value_b;
    } else if operation.starts_with(">>") {
        result = value_a >> value_b;
    } else if operation.starts_with("AND") {
        result = value_a & value_b;
    } else if operation.starts_with("OR") {
        result = value_a | value_b;
    } else if operation.starts_with("%") {
        result = value_a % value_b;
    }
    return result;
}
#[wasm_bindgen]
pub fn rust_calc_unary(value: i64, operation: &str) -> i64 {
    let mut result: i64 = 0;
    if operation.starts_with("NOT") {
        result = !value;
    } else if operation.starts_with("NEG") {
        result = 0 - value;
    }
    return result;
}
#[wasm_bindgen]
pub fn rust_calc_float(value_a: f64, value_b: f64, operation: &str) -> f64 {
    let mut f_result: f64 = 1.1;
    if operation.starts_with("+") {
        f_result = value_a + value_b;
    } else if operation.starts_with("*") {
        f_result = value_a * value_b;
    } else if operation.starts_with("-") {
        f_result = value_a - value_b;
    } else if operation.starts_with("/") {
        f_result = value_a / value_b;
    }
    return f_result;
}
#[wasm_bindgen]
pub fn int_value(x: Option<f64>) -> i64 {
    match x {
        Some(x) => x as i64,
        None => -1,
    }
}
#[wasm_bindgen]
pub fn float_value(x: Option<f64>) -> f64 {
    match x {
        Some(x) => x,
        None => -1.0,
    }
}

#[wasm_bindgen]
pub fn main() {
    let int_ops: Vec<&str> = vec![
        "+<2>", "*<2>", "-<2>", "/<2>", ">><2>", "AND<2>", "OR<2>", "NOT<1>", "NEG<1>", "%<2>",
    ];
    let float_ops: Vec<&str> = vec!["+<2>", "*<2>", "-<2>", "/<2>"];
    // User Documentation
    println!("********************************");
    println!("*       Program Support        *");
    println!("--------------------------------");
    println!("DataType | Operation<n-Operands>");
    println!("--------------------------------");
    for i in int_ops {
        println!("INT      | {}", i)
    }
    for f in float_ops {
        println!("FLOAT    | {}", f)
    }
    println!("--------------------------------");

    // Retrieve values
    println!("Enter the Expression (Post-Fix): ");
    let mut expr = String::new();
    let mut int_a: i64 = 0;
    let mut int_b: i64 = 0;
    let mut int_result: i64 = 0;
    let mut float_a: f64 = 0.0;
    let mut float_b: f64 = 0.0;
    let mut float_result: f64 = 0.0;
    let mut op: &str;

    io::stdin()
        .read_line(&mut expr)
        .expect("Invalid String Entry");

    let line_split = expr.trim().split(" ");

    let mut stack: Vec<&str> = line_split.collect();

    // let mut count: usize = 0;
    let mut parsed_vec: Vec<f64> = Vec::new();

    for item in stack {
        /*
        // Error Checking
        println!(
            "is int({0}){1}, is float({0}){2}, is op({0}){3}",
            item,
            is_int(item.to_string()),
            is_float(item.to_string()),
            is_op_unary(item.to_string())
        );
        */

        if is_int(item.to_string()) {
            // Populate Stack
            parsed_vec.push(item.trim().parse().expect("Invalid Integer"));
        } else if is_float(item.to_string()) {
            // Populate Stack
            parsed_vec.push(item.trim().parse().expect("Invalid Float"));
        } else if is_op_unary(item.to_string()) {
            // Get operand & operator
            op = item.trim();
            int_a = int_value(parsed_vec.pop());
            // Calculate & push result to stack
            int_result = rust_calc_unary(int_a, op);
            parsed_vec.push(int_result as f64);
        } else if is_op_float(item.to_string()) {
            // Get operands & operator
            op = item.trim();
            float_a = float_value(parsed_vec.pop());
            float_b = float_value(parsed_vec.pop());
            // Calculate & push result to stack
            float_result = rust_calc_float(float_a, float_b, op);
            parsed_vec.push(float_result);
        } else if is_op_int(item.to_string()) {
            // Get operands & operator
            op = item.trim();
            int_a = int_value(parsed_vec.pop());
            int_b = int_value(parsed_vec.pop());
            // Calculate & push ressult to stack
            int_result = rust_calc_int(int_a, int_b, op);
            parsed_vec.push(int_result as f64);
        } else {
            panic!("Error: Invalid syntax!");
        }
        /*
        // Error Checking
        println!("{:?}", parsed_vec);
        */
    }

    // Print Final Result
    let final_result = float_value(parsed_vec.pop());
    if final_result.trunc() == final_result {
        println!("Result = {}", final_result.trunc());
    } else {
        println!("Result = {}", final_result);
    }
}

