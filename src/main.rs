use std::io;
use std::{f64::NAN, f64::consts};

fn main() {
    println!("Type \"exit\" or \"quit\" to quit");
    let mut stk: Vec<f64> = vec!();
    let mut running = true;
    while running {
        println!("\nStack:");
        for el in stk.iter() {
            println!("{:e}", el);
        }

        let mut s = String::new();
		io::stdin().read_line(&mut s)
			.expect("Failed to read line");

        running = process_input(&mut stk, &mut s);

    }
}

fn process_input(stk: &mut Vec<f64>, s: &mut String) -> bool {
    let mut continue_processing = true;

    for tk in s.split_whitespace() {
        let x = Token::new(&tk[..]);
        continue_processing = match_token(stk, tk, x);
        if !continue_processing {break}
    }
    return continue_processing;
}

fn match_token(stk: &mut Vec<f64>, tk: &str, x: Token) -> bool {
    let mut process_next_token = true;
    match x {
        Token::Number(num) => stk.push(num),
        Token::OpBinary(bin_closure) => {
            if stk.len() > 1 {
                let (y, x) = (stk.pop().unwrap(),
                              stk.pop().unwrap());
                stk.push(bin_closure(x, y));
            } else {
                print!("You need at least two numbers in ");
                println!("the stack to perform binary operations.");
            }
        },
        Token::OpUnary(un_closure) => {
            if stk.len() > 0 {
                let x = stk.pop().unwrap();
                stk.push(un_closure(x));
            } else {
                print!("You need at least one number in ");
                println!("the stack to perform unary operations.");
            }
        }
        Token::Swap => {
            if stk.len() > 1 {
                let (y, x) = (stk.pop().unwrap(),
                              stk.pop().unwrap());
                stk.push(y);
                stk.push(x);
            } else {
                print!("You need at least two numbers in ");
                println!("the stack to perform swap operation.");
            }
        },
        Token::Del => {
            if stk.len() > 0 {
                stk.pop();
            } else {
                print!("You need at least one number ");
                println!("in stack to perform delete operation.");
            }
        },
        Token::Clear => {
            stk.clear();
        },
        Token::Quit => {
            process_next_token = false;
        },
        Token::Invalid => println!("{} is an invalid input.", tk.trim()),
        _ => println!("What a beautiful Duwang!")
    }
    return process_next_token;
}

pub enum Token<'a>
{
    Number(f64),
    OpBinary(&'a dyn Fn(f64, f64) -> f64),
    OpUnary(&'a dyn Fn(f64) -> f64),
    Del,
    Clear,
    Swap,
    Invalid,
    Quit
}

impl<'a> Token<'a> {
    pub fn new(s: &str)-> Token {
        let x: f64 = match s.trim().parse() {
			Ok(num) => num,
			Err(_) => NAN,
		};

        if x.is_nan() {
            return match &s.trim()[..] {
                // Binary operations
                "+"   => Token::OpBinary(&|x, y| {x + y}),
                "-"   => Token::OpBinary(&|x, y| {x - y}),
                "*"   => Token::OpBinary(&|x, y| {x * y}),
                "/"   => Token::OpBinary(&|x, y| {x / y}),
                "%"   => Token::OpBinary(&|x, y| {x % y}),
                "pow" => Token::OpBinary(&|x, y| {x.powf(y)}),
                "^"   => Token::OpBinary(&|x, y| {x.powf(y)}),
                // Unary operations
                "neg"    => Token::OpUnary(&|x| {-x}),
                "inv"    => Token::OpUnary(&|x| {1. / x}),
                "abs"    => Token::OpUnary(&|x| {x.abs()}),
                "sq"     => Token::OpUnary(&|x| {x * x}),
                "square" => Token::OpUnary(&|x| {x * x}),
                "sqrt"   => Token::OpUnary(&|x| {x.sqrt()}),
                "cub"    => Token::OpUnary(&|x| {x * x * x}),
                "cube"   => Token::OpUnary(&|x| {x * x * x}),
                "cubrt"  => Token::OpUnary(&|x| {x.powf(1. / 3.)}),
                "cubert" => Token::OpUnary(&|x| {x.powf(1. / 3.)}),
                "exp"    => Token::OpUnary(&|x| {x.exp()}),
                "ln"     => Token::OpUnary(&|x| {x.ln()}),
                "log2"   => Token::OpUnary(&|x| {x.log2()}),
                "log10"  => Token::OpUnary(&|x| {x.log2()}),
                "sin"    => Token::OpUnary(&|x| {x.sin()}),
                "asin"   => Token::OpUnary(&|x| {x.asin()}),
                "cos"    => Token::OpUnary(&|x| {x.cos()}),
                "acos"   => Token::OpUnary(&|x| {x.acos()}),
                "tan"    => Token::OpUnary(&|x| {x.tan()}),
                "atan"   => Token::OpUnary(&|x| {x.atan()}),
                "sinh"   => Token::OpUnary(&|x| {x.sinh()}),
                "asinh"  => Token::OpUnary(&|x| {x.asinh()}),
                "cosh"   => Token::OpUnary(&|x| {x.cosh()}),
                "acosh"  => Token::OpUnary(&|x| {x.acosh()}),
                "tanh"   => Token::OpUnary(&|x| {x.tanh()}),
                "atanh"  => Token::OpUnary(&|x| {x.atanh()}),
                // constants
                "pi"    => Token::Number(consts::PI),
                "e"     => Token::Number(consts::E),
                "c"     => Token::Number(299792458.), // m/s speed of light
                "h"     => Token::Number(6.6207015e-34), // Js
                "h_bar" => Token::Number(6.6207004e-34 /
                    (2. * consts::PI)
                ), // Js
                // quiting
                "quit" => Token::Quit,
                "exit" => Token::Quit,
                // delete
                "delete" => Token::Del,
                "del"    => Token::Del,
                // clear all
                "clear" => Token::Clear,
                // swap
                "swap" => Token::Swap,
                // everything else
                _     => Token::Invalid
            }
        } else {
            return Token::Number(x)
        }
    }
}