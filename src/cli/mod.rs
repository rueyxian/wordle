mod display;
mod input;

pub use display::print_calculating;

// ===============================================

use crate::pattern;
use crate::word_pool;

use crate::universe;

// ===============================================

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;

    let word_source = word_pool::WordPool::new();
    let word_pool = word_source.read_pool().unwrap();

    let mut universe = universe::Universe::new(5, word_pool);
    let mut top_n = 24;

    loop {
        display::clear_screen();

        println!("possibility: {}\n", universe.posibility());
        display::print_entropy_ranking(&universe, top_n);
        println!("{}", universe.pattern_stack());

        println!("enter `:h` for help\n");
        let input = get_input();

        let _ = std::io::stdout().flush();

        match input::Input::try_from(input.as_str()) {
            Ok(input) => match input {
                input::Input::Quit => break,
                input::Input::Help => display::print_help(),
                input::Input::Revert => {
                    if universe.revert().is_err() {
                        println!("can't revert");
                        display::press_any_key_to_continue();
                    }
                }
                input::Input::Clear => while universe.revert().is_ok() {},
                input::Input::Top(n) => top_n = n,
                input::Input::Pattern(s) => {
                    let pattern_line = pattern::PatternLine::try_from(s.as_str()).unwrap();
                    universe.progress(pattern_line);
                }
            },
            Err(e) => {
                println!("{:?}\n", e);
                display::press_any_key_to_continue();
            }
        }
    }

    Ok(())
}

// ===============================================

fn get_input() -> String {
    use std::io::Write;
    print!("▶︎ ");
    let mut input = String::new();
    let _ = std::io::stdout().flush();
    std::io::stdin().read_line(&mut input).unwrap();
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }
    input
}

// ===============================================
