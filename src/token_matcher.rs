#[path = "./token.rs"]
mod token;
use token::Token;

pub(crate) fn match_token(stk: &mut Vec<f64>, tk: &str) -> bool {
    let x = Token::new(&tk[..]);
    let mut process_next_token = true;
    match x {
        Token::Number(num) => handle_number(stk, num),
        Token::OpBinary(bin_closure) => handle_op_binary(stk, bin_closure),
        Token::OpUnary(un_closure) => handle_op_unary(stk, un_closure),
        Token::Swap => handle_swap(stk),
        Token::Del => handle_del(stk),
        Token::Clear => handle_clear(stk),
        Token::Quit => process_next_token = false,
        Token::Invalid => handle_invalid(tk),
        _ => handle_catch_all(),
    }
    return process_next_token;
}

fn handle_number(stk: &mut Vec<f64>, num: f64) {
    stk.push(num);
}

fn handle_op_binary(stk: &mut Vec<f64>, bin_closure: &dyn Fn(f64, f64) -> f64) {
    if stk.len() > 1 {
        let (y, x) = (stk.pop().unwrap(), stk.pop().unwrap());
        stk.push(bin_closure(x, y));
    } else {
        print!("You need at least two numbers in ");
        println!("the stack to perform binary operations.");
    }
}

fn handle_op_unary(stk: &mut Vec<f64>, un_closure: &dyn Fn(f64) -> f64) {
    if stk.len() > 0 {
        let x = stk.pop().unwrap();
        stk.push(un_closure(x));
    } else {
        print!("You need at least one number in ");
        println!("the stack to perform unary operations.");
    }
}

fn handle_swap(stk: &mut Vec<f64>) {
    if stk.len() > 1 {
        let (y, x) = (stk.pop().unwrap(), stk.pop().unwrap());
        stk.push(y);
        stk.push(x);
    } else {
        print!("You need at least two numbers in ");
        println!("the stack to perform swap operation.");
    }
}

fn handle_del(stk: &mut Vec<f64>) {
    if stk.len() > 0 {
        stk.pop();
    } else {
        print!("You need at least one number ");
        println!("in stack to perform delete operation.");
    }
}

fn handle_clear(stk: &mut Vec<f64>) {
    stk.clear();
}

fn handle_invalid(tk: &str) {
    println!("{} is an invalid input.", tk.trim());
}

fn handle_catch_all() {
    println!("What a beautiful Duwang!");
}
