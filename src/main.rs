use std::io;
mod token_matcher;
fn main() {
    println!("Type \"exit\" or \"quit\" to quit");
    let mut stk: Vec<f64> = vec![];
    let mut running = true;
    while running {
        running = run_calculator(&mut stk);
    }
}

fn run_calculator(mut stk: &mut Vec<f64>) -> bool {
    print_stack_contents(&mut stk);
    return process_input(&mut stk, &mut get_input());
}

fn print_stack_contents(stk: &mut Vec<f64>) {
    println!("\nStack:");
    for el in stk.iter() {
        println!("{:e}", el);
    }
}

fn get_input() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    return s;
}

fn process_input(stk: &mut Vec<f64>, s: &mut String) -> bool {
    let mut continue_processing = true;

    for tk in s.split_whitespace() {
        continue_processing = token_matcher::match_token(stk, tk);
        if !continue_processing {
            break;
        }
    }
    return continue_processing;
}
