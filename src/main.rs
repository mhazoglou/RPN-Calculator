use rpn;
use std::collections::HashMap;

fn main() {
    println!("Type \"exit\" or \"quit\" to quit");
    let mut map: HashMap<String, rpn::Session> = HashMap::new();
    let mut current_session_name = String::from("default");
    map.insert(String::from("default"), 
               rpn::Session::new()
    );
    // let mut output = rpn::Output::new();
    
    let mut running = true;
    while running {
        let outputs = map[&current_session_name[..]].run_session();
        
        for output in outputs {
            running = output.running;
            if !running {
                break;
            }
            
            if output.new_session {
                let new_session = rpn::Session::new();
                map.entry(output.session_name).or_insert(new_session);
            } else if output.change_session {
                if map.contains_key(&output.session_name) {
                    current_session_name = output.session_name.clone();
                } else {
                    print!("Session {} was not created", output.session_name);
                    println!(" please create by entering new:{}", output.session_name);
                }
            } else if output.print_hist {
                map[&current_session_name[..]].print_history();
            }
        }
    }
}


