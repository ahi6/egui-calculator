// Shunting yard algorithm
// https://en.wikipedia.org/wiki/Shunting-yard_algorithm
// https://brilliant.org/wiki/shunting-yard-algorithm/
// https://www.geeksforgeeks.org/java-program-to-implement-shunting-yard-algorithm/
fn equation_to_rpn(equation: &str) -> Result<Vec<String>, String> {
    let mut result: Vec<String> = Vec::new();
    let mut operators: Vec<char> = Vec::new();
    let mut number_buffer = String::new(); // Buffer for numbers with multiple digits
    let mut prev_was_operator = true;

    for c in equation.chars() {
        if c.is_ascii_digit() || c == '.' {
            number_buffer.push(c);
            prev_was_operator = false;
            continue;
        } else if !number_buffer.is_empty() {
            result.push(number_buffer.clone());
            number_buffer.clear();
        }

        match c {
            '(' => {
                operators.push(c);
                prev_was_operator = true;
            }
            ')' => {
                while let Some(op) = operators.pop() {
                    if op == '(' {
                        break;
                    }
                    result.push(op.to_string());
                }
                prev_was_operator = false;
            }
            '+' | '-' => {
                if prev_was_operator {
                    number_buffer.push(c); // Treat as negative sign
                } else {
                    while let Some(op) = operators.last() {
                        if *op == '(' {
                            break;
                        }
                        result.push(operators.pop().unwrap().to_string());
                    }
                    operators.push(c);
                    prev_was_operator = true;
                }
            }
            '*' | '/' => {
                while let Some(op) = operators.last() {
                    if *op == '(' || *op == '+' || *op == '-' {
                        break;
                    }
                    result.push(operators.pop().unwrap().to_string());
                }
                operators.push(c);
                prev_was_operator = true;
            }
            // '^' => {
            //     while let Some(op) = operators.last() {
            //         if *op == '(' {
            //             break;
            //         }
            //         result.push(operators.pop().unwrap().to_string());
            //     }
            //     operators.push(c);
            //     prev_was_operator = true;
            // }
            ' ' => {} // Spaces can be ignored
            _ => {
                return Err(format!("Neznámý znak v rovnici: {}", c))
            }
        }
    }

    if !number_buffer.is_empty() {
        result.push(number_buffer.clone());
    }

    while let Some(op) = operators.pop() {
        result.push(op.to_string());
    }

    Ok(result)
}
// Evaluate the RPN expression
fn evaluate_rpn(rpn: Vec<String>) -> Result<f32, String> {
    let mut stack: Vec<f32> = Vec::new();
    for token in rpn {
        if let Ok(number) = token.parse::<f32>() {
            stack.push(number);
        } else {
            let b = stack.pop().ok_or("Wrong expression".to_string())?;
            let a = stack.pop().ok_or("Wrong expression".to_string())?;
            match token.as_str() {
                "+" => stack.push(a + b),
                "-" => stack.push(a - b),
                "*" => stack.push(a * b),
                "/" => {
                    if b == 0.0 {
                        return Err("Cannot divide by zero".to_string());
                    }
                    stack.push(a / b)
                },
                // "^" => stack.push(a.powf(b)),
                _ => {}
            }
        }
    }
    stack.pop().ok_or("Wrong expression".to_string())
}

pub fn get_rpn(equation: &str) -> Result<String, String> {
    match equation_to_rpn(equation) {
        Ok(vec) => Ok(vec.join(" ")),
        Err(e) => Err(e)
    }
}
pub fn calculate(equation: &str) -> Result<f32, String> {
    evaluate_rpn(equation_to_rpn(equation)?)
}

// Test the shunting yard algorithm
#[test]
fn test_equation_to_rpn() {
    assert_eq!(equation_to_rpn("1+2").unwrap(), vec!["1", "2", "+"]);
    assert_eq!(equation_to_rpn("1+2*3").unwrap(), vec!["1", "2", "3", "*", "+"]);
    assert_eq!(equation_to_rpn("1+2*3-4").unwrap(), vec!["1", "2", "3", "*", "+", "4", "-"]);
    assert_eq!(equation_to_rpn("1+2*3-4/5").unwrap(), vec!["1", "2", "3", "*", "+", "4", "5", "/", "-"]);
    assert_eq!(equation_to_rpn("1+2*3-4/5+6").unwrap(), vec!["1", "2", "3", "*", "+", "4", "5", "/", "-", "6", "+"]);
    assert_eq!(equation_to_rpn("1+2*3-4/5+6*7").unwrap(), vec!["1", "2", "3", "*", "+", "4", "5", "/", "-", "6", "7", "*", "+"]);
    assert_eq!(equation_to_rpn("1+2*3-4/5+6*7-8").unwrap(), vec!["1", "2", "3", "*", "+", "4", "5", "/", "-", "6", "7", "*", "+", "8", "-"]);
    assert_eq!(equation_to_rpn("1+2*3-4/5+6*7-8/9").unwrap(), vec!["1", "2", "3", "*", "+", "4", "5", "/", "-", "6", "7", "*", "+", "8", "9", "/", "-"]);
    // assert_eq!(equation_to_rpn("2(2)").unwrap(), vec!["2", "2", "*"])
}

// Test the evaluation of RPN expressions
#[test]
fn test_evaluate_rpn() {
    assert_eq!(calculate("1+2").unwrap(), 3.0);
    assert_eq!(calculate("1+2*3").unwrap(), 7.0);
    assert_eq!(calculate("1+2*3-4").unwrap(), 3.0);
    assert_eq!(calculate("1+2*3-4/5").unwrap(), 6.2);
    assert_eq!(calculate("1+2*3-4/5+6").unwrap(), 12.2);
    assert_eq!(calculate("1+2*3-4/5+6*7").unwrap(), 48.2);
    assert_eq!(calculate("1+2*3-4/5+6*7-8").unwrap(), 40.2);
    assert_eq!(calculate("1+2*3-4/5+6*7-8/9").unwrap().round(), 47.0);
}

#[test]
fn test_invalid_inputs() {
    assert!(calculate("1+2*3-4/5+6*7-8/").is_err());
    assert!(calculate("1+2*3-4/5+6*7-8/9+").is_err());
    assert!(calculate("1+2*3-4/5+6*7-8/9+*").is_err());
    assert!(calculate("1+2*3-4/5+6*7-8/9+*(").is_err());
    assert!(calculate("1+2*3-4/5+6*7-8/9+*)").is_err());
    assert!(calculate("1+2*3-4/5+6*7-8/9+*a").is_err());
}

#[test]
fn test_negative_numbers() {
    assert_eq!(equation_to_rpn("-1+2").unwrap(), vec!["-1", "2", "+"]);
    assert_eq!(equation_to_rpn("1+-2").unwrap(), vec!["1", "-2", "+"]);
    assert_eq!(equation_to_rpn("1+(-2)").unwrap(), vec!["1", "-2", "+"]);
    assert_eq!(calculate("-1+2").unwrap(), 1.0);
    assert_eq!(calculate("1+-2").unwrap(), -1.0);
    assert_eq!(calculate("1+(-2)").unwrap(), -1.0);
}

// #[test]
// fn test_power_function() {
//     assert_eq!(calculate("2^3").unwrap(), 8.0);
//     assert_eq!(calculate("4^0.5").unwrap(), 2.0);
//     assert_eq!(calculate("2^3+1").unwrap(), 9.0);
//     assert_eq!(calculate("2^(1+1)").unwrap(), 4.0);
//     // yeah, it has bugs
//     // assert_eq!(calculate("2^3^2").unwrap(), 512.0); // 2^(3^2) = 2^9 = 512
//     // assert_eq!(calculate("2^3*2").unwrap(), 16.0); // 2^3 * 2 = 8 * 2 = 16
//     // assert_eq!(calculate("2*2^3").unwrap(), 16.0); // 2 * 2^3 = 2 * 8 = 16
// }
