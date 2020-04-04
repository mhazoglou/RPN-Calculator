use std::io;
use std::{f64::consts, f64::NAN};
use std::cell::RefCell;
use std::collections::HashMap;

macro_rules! get_sess{
    ($sess_man:ident, $sess:ident) => (
        let s = &*$sess_man.current_session.borrow(); // borrow ref
        let map = $sess_man.map.borrow();
        let $sess = &map[s];
    );
}

macro_rules! get_sess_mut{
    ($sess_man:ident, $sess:ident) => (
        let s = &*$sess_man.current_session.borrow(); // borrow ref
        let map = $sess_man.map.borrow_mut();
        let $sess = &map[s];
    );
}

pub struct SessionManager {
    map: RefCell<HashMap<String, Session>>,
    current_session: RefCell<String>
}

impl SessionManager {
    pub fn new() -> SessionManager {
        let mut map = HashMap::new();
        
        let current_session = String::from("default");

        map.insert(current_session.clone(), 
                   Session::new());
        
        SessionManager {
            map: RefCell::new(map),
            current_session: RefCell::new(current_session)
        }
    }
    
    fn add_new_session(&self, s: String) {
        let new_session = Session::new();
        self.map.borrow_mut().entry(s).or_insert(new_session);
    }
    
    fn change_current_session(&self, s: String) {
        if self.map.borrow().contains_key(&s) {
            let mut current_session = self.current_session.borrow_mut();
            *current_session = s;
        } else {
            print!("Session {} was not created", s);
            println!(" please create by entering new:{}", s);
        }
    }
    
    fn print_stack(&self) {
        get_sess!(self, sess);
        sess.print_stack();
    }
    
    fn print_history(&self) {
        get_sess!(self, sess);
        sess.print_history();
    }

    fn clear_history(&self) {
        get_sess_mut!(self, sess);
        sess.clear_history();
    }

    pub fn run_manager(&self) {
        println!("Type \"exit\" or \"quit\" to quit");
        let mut running = true;
        while running {
            self.print_stack();
            running = self.process_input();

        }
    }

    fn process_input(&self) -> bool {
        let mut running = true;
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read line");

        for tk in s.split_whitespace() {
            running = self.match_token(tk);
            if !running {
                break;
            }

        }

        self.push_to_history(s);

        running
    }

    fn push_to_stack(&self, num: &f64) {
        get_sess_mut!(self, sess);
        sess.push_to_stack(num);
    }

    fn push_to_history(&self, string: String) {
        get_sess_mut!(self, sess);
        sess.push_to_history(string);
    }

    fn op_binary(&self, bin_closure: &dyn Fn(f64, f64) -> f64) {
        get_sess_mut!(self, sess);
        sess.op_binary(bin_closure);
    }

    fn op_unary(&self, un_closure: &dyn Fn(f64) -> f64) {
        get_sess_mut!(self, sess);
        sess.op_unary(un_closure);
    }

    fn swap(&self) {
        get_sess_mut!(self, sess);
        sess.swap();
    }

    fn del(&self) {
        get_sess_mut!(self, sess);
        sess.del();
    }

    fn clear_stack(&self) {
        get_sess_mut!(self, sess);
        sess.clear_stack();
    }
    
    fn match_token(&self, tk: &str) -> bool {
        let mut running = true;
        let x = Token::new(&tk[..]);
        match x {
            Token::Number(num) => self.push_to_stack(&num),
            Token::OpBinary(bin_closure) => self.op_binary(bin_closure),
            Token::OpUnary(un_closure) => self.op_unary(un_closure),
            Token::Swap => self.swap(),
            Token::Del => self.del(),
            Token::ClearStack => self.clear_stack(),
            Token::NewSession(s) => self.add_new_session(s.to_string()),
            Token::ChangeSession(s) => self.change_current_session(s.to_string()),
            Token::History => self.print_history(),
            Token::ClearHistory => self.clear_history(),
            Token::Quit => running = false,
            Token::Invalid => println!("{} is an invalid input.", tk),
            _ => println!("What a beautiful Duwang!"),
        }
        
        running
    }
}


struct Session {
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
    
    pub fn push_to_stack(&self, num: &f64) {
        self.stack.borrow_mut().push(*num);
    }
    
    pub fn push_to_history(&self, s: String) {
        self.history.borrow_mut().push(s);
    }

    fn op_binary(&self, bin_closure: &dyn Fn(f64, f64) -> f64) {
        let mut stk = self.stack.borrow_mut();
        if stk.len() > 1 {
            let (y, x) = (stk.pop().unwrap(), stk.pop().unwrap());
            stk.push(bin_closure(x, y));
        } else {
            print!("You need at least two numbers in ");
            println!("the stack to perform binary operations.");
        }
    }

    fn op_unary(&self, un_closure: &dyn Fn(f64) -> f64) {
        let mut stk = self.stack.borrow_mut();
        if stk.len() > 0 {
            let x = stk.pop().unwrap();
            stk.push(un_closure(x));
        } else {
            print!("You need at least one number in ");
            println!("the stack to perform unary operations.");
        }
    }

    fn swap(&self) {
        let mut stk = self.stack.borrow_mut();
        if stk.len() > 1 {
            let (y, x) = (stk.pop().unwrap(), stk.pop().unwrap());
            stk.push(y);
            stk.push(x);
        } else {
            print!("You need at least two numbers in ");
            println!("the stack to perform swap operation.");
        }
    }

    fn del(&self) {
        let mut stk = self.stack.borrow_mut();
        if stk.len() > 0 {
            stk.pop();
        } else {
            print!("You need at least one number ");
            println!("in stack to perform delete operation.");
        }
    }

    fn clear_stack(&self) {
        let mut stk = self.stack.borrow_mut();
        stk.clear();
    }
    
    // fn run_session(&self) -> Vec<Output> {
        // print_stack_contents(&self.stack.borrow());
        // self.update()
    // }
    
    fn print_history(&self) {
        println!("History:");
        for s in self.history.borrow().iter() {
            println!("{}", *s);
        }
    }
    
    fn clear_history(&self) {
        self.history.borrow_mut().clear();
    }
    
    fn print_stack(&self) {
        println!("\nStack:");
        for el in self.stack.borrow().iter() {
            println!("{:e}", el);
        }
        //println!("\x1b[{}F", stk.len() as u32 + 3);
    }
    
    /*
    pub fn save(&self) {
        
    }*/
}


enum Token<'a> {
    Number(f64),
    OpBinary(&'a dyn Fn(f64, f64) -> f64),
    OpUnary(&'a dyn Fn(f64) -> f64),
    Del,
    ClearStack,
    Swap,
    Invalid,
    NewSession(&'a str),
    ChangeSession(&'a str),
    History,
    ClearHistory,
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
                // clear all elements in current stack
                ["clear"] => Token::ClearStack,
                // swap
                ["swap"] => Token::Swap,
                // new session
                ["new", s] => Token::NewSession(s),
                // change session
                ["change_to", s] => Token::ChangeSession(s),
                ["go_to", s] => Token::ChangeSession(s),
                // print history for current session
                ["hist"] => Token::History,
                // clear history of current session
                ["hist_clear"] => Token::ClearHistory,
                // everything else
                _ => Token::Invalid,
            };
        } else {
            return Token::Number(x);
        }
    }
}