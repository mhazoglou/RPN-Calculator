use std::io;
use std::{f64::consts, f64::NAN};
use std::cell::RefCell;
use std::collections::HashMap;


pub struct Session {
    stack: RefCell<Vec<f64>>,
    history: RefCell<Vec<String>>
}

impl Session {
    pub fn new() -> Session {
        Session {
            stack: RefCell::new(vec![]),
            history: RefCell::new(vec![])
        }
    }
    
    fn update(self:&Self) -> Vec<Output> {
        let s = get_input();
        self.history.borrow_mut().push(s.clone());
        
        process_input(&mut self.stack.borrow_mut(), &s[..])
    }
    
    pub fn run_session(self:&Self) -> Vec<Output> {
        print_stack_contents(&self.stack.borrow());
        self.update()
    }
    
    pub fn print_history(self:&Self) {
        println!("History:");
        for s in self.history.borrow().iter() {
            print!("{}", *s);
        }
    }
    /*
    pub fn save(self:&Self) {
        
    }*/
}

// pub fn run_calculator(mut stk: &mut Vec<f64>) -> Output {
    // print_stack_contents(&stk);
    // return process_input(&mut stk, &get_input());
// }

fn print_stack_contents(stk: &Vec<f64>) {
    println!("\nStack:");
    for el in stk.iter() {
        println!("{:e}", el);
    }
    //println!("\x1b[{}F", stk.len() as u32 + 3);
}

fn get_input() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    
    s
}

fn process_input(stk: &mut Vec<f64>, s: &str) -> Vec<Output> {
    let mut outputs: Vec<Output> = Vec::new();

    for tk in s.split_whitespace() {
        let x = Token::new(&tk[..]);
        let output = match_token(stk, tk, x);
        let running = output.running;
        outputs.push(output);
        if !running {
            break;
        }
        
    }
    
    outputs
}

fn match_token(stk: &mut Vec<f64>, tk: &str, x: Token) -> Output {
    let mut running = true;
    let mut new_session = false;
    let mut change_session = false;
    let mut session_name = String::new();
    let mut print_hist = false;
    match x {
        Token::Number(num) => handle_number(stk, &num),
        Token::OpBinary(bin_closure) => handle_op_binary(stk, bin_closure),
        Token::OpUnary(un_closure) => handle_op_unary(stk, un_closure),
        Token::Swap => handle_swap(stk),
        Token::Del => handle_del(stk),
        Token::Clear => handle_clear(stk),
        Token::NewSession(s) => {
            new_session = true;
            session_name = String::from(s);
        },
        Token::ChangeSession(s) => {
            change_session = true;
            session_name = String::from(s);},
        Token::History => print_hist = true,
        Token::Quit => running = false,
        Token::Invalid => handle_invalid(tk),
        _ => handle_catch_all(),
    }
    
    Output {
        running,
        new_session,
        change_session,
        session_name,
        print_hist
    }
}

fn handle_number(stk: &mut Vec<f64>, num: &f64) {
    stk.push(*num);
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


pub struct Output {
    pub running: bool,
    pub new_session: bool,
    pub change_session: bool,
    pub session_name: String,
    pub print_hist: bool
}

impl Output {
    pub fn new() -> Output {
        Output {
            running: true,
            new_session: false,
            change_session: false,
            session_name: String::new(),
            print_hist: false,
        }
    }
}

pub enum Token<'a> {
    Number(f64),
    OpBinary(&'a dyn Fn(f64, f64) -> f64),
    OpUnary(&'a dyn Fn(f64) -> f64),
    Del,
    Clear,
    Swap,
    Invalid,
    NewSession(&'a str),
    ChangeSession(&'a str),
    History,
    Quit
}

impl<'a> Token<'a> {
    pub fn new(s: &str) -> Token {
        let x: f64 = match s.trim().parse() {
            Ok(num) => num,
            Err(_) => NAN,
        };

        if x.is_nan() {
            return match s.trim().split(':').collect::<Vec<&str>>()[..] {
                // Binary operations
                ["+"] => Token::OpBinary(&|x, y| x + y),
                ["-"] => Token::OpBinary(&|x, y| x - y),
                ["*"] => Token::OpBinary(&|x, y| x * y),
                ["/"] => Token::OpBinary(&|x, y| x / y),
                ["%"] => Token::OpBinary(&|x, y| x % y),
                ["pow"] => Token::OpBinary(&|x, y| x.powf(y)),
                ["^"] => Token::OpBinary(&|x, y| x.powf(y)),
                // Unary operations
                ["neg"] => Token::OpUnary(&|x| -x),
                ["inv"] => Token::OpUnary(&|x| 1. / x),
                ["abs"] => Token::OpUnary(&|x| x.abs()),
                ["sq"] => Token::OpUnary(&|x| x * x),
                ["square"] => Token::OpUnary(&|x| x * x),
                ["sqrt"] => Token::OpUnary(&|x| x.sqrt()),
                ["cub"] => Token::OpUnary(&|x| x * x * x),
                ["cube"] => Token::OpUnary(&|x| x * x * x),
                ["cubrt"] => Token::OpUnary(&|x| x.powf(1. / 3.)),
                ["cubert"] => Token::OpUnary(&|x| x.powf(1. / 3.)),
                ["exp"] => Token::OpUnary(&|x| x.exp()),
                ["ln"] => Token::OpUnary(&|x| x.ln()),
                ["log2"] => Token::OpUnary(&|x| x.log2()),
                ["log10"] => Token::OpUnary(&|x| x.log2()),
                ["sin"] => Token::OpUnary(&|x| x.sin()),
                ["asin"] => Token::OpUnary(&|x| x.asin()),
                ["cos"] => Token::OpUnary(&|x| x.cos()),
                ["acos"] => Token::OpUnary(&|x| x.acos()),
                ["tan"] => Token::OpUnary(&|x| x.tan()),
                ["atan"] => Token::OpUnary(&|x| x.atan()),
                ["sinh"] => Token::OpUnary(&|x| x.sinh()),
                ["asinh"] => Token::OpUnary(&|x| x.asinh()),
                ["cosh"] => Token::OpUnary(&|x| x.cosh()),
                ["acosh"] => Token::OpUnary(&|x| x.acosh()),
                ["tanh"] => Token::OpUnary(&|x| x.tanh()),
                ["atanh"] => Token::OpUnary(&|x| x.atanh()),
                // constants
                ["pi"] => Token::Number(consts::PI),
                ["e"] => Token::Number(consts::E),
                ["c"] => Token::Number(299792458.), // m/s speed of light
                ["h"] => Token::Number(6.6207015e-34), // Js
                ["h_bar"] => Token::Number(6.6207004e-34 / (2. * consts::PI)), // Js
                // quiting
                ["quit"] => Token::Quit,
                ["exit"] => Token::Quit,
                // delete
                ["delete"] => Token::Del,
                ["del"] => Token::Del,
                // clear all
                ["clear"] => Token::Clear,
                // swap
                ["swap"] => Token::Swap,
                // new session
                ["new", s] => Token::NewSession(s),
                // change session
                ["change_to", s] => Token::ChangeSession(s),
                //
                ["hist"] => Token::History,
                // everything else
                _ => Token::Invalid,
            };
        } else {
            return Token::Number(x);
        }
    }
}