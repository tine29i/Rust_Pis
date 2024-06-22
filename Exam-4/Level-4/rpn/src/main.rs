fn rpn(s: &str) {
    let mut stack = Vec::new();

    for token in s.split_whitespace() {
        match token.parse::<i64>() {
            Ok(num) => stack.push(num),
            Err(_) => {
                let right = stack.pop();
                let left = stack.pop();

                match (left, right, token) {
                    (Some(left), Some(right), "+") => stack.push(left + right),
                    (Some(left), Some(right), "-") => stack.push(left - right),
                    (Some(left), Some(right), "*") => stack.push(left * right),
                    (Some(left), Some(right), "/") => stack.push(left / right),
                    (Some(left), Some(right), "%") => stack.push(left % right),
                    _ => return println!("Error"),
                }
            }
        }
    }

        if stack.len() == 1 {
            println!("{}", stack.pop().unwrap());
        } else {
            println!("Error");
        }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    rpn(&args[1]);
}