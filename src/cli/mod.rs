use crate::pattern;
use crate::word_source;

// ===============================================

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;

    let word_source = word_source::WordSource::new();
    let word_set = word_source.read_vault().unwrap();
    let word_set = word_set
        .iter()
        .map(|word| word.as_str())
        .collect::<Vec<&str>>();

    let mut word_set_history = Vec::<Vec<&str>>::new();

    word_set_history.push(word_set.clone());

    let mut pattern_stack = pattern::PatternStack::new(5);

    loop {
        clear_screen();
        print_word_sets(&word_set_history, 9);
        println!("{}", pattern_stack);

        let input = get_input();

        let _ = std::io::stdout().flush();

        match input.as_str() {
            ":q" => break,
            ":undo" => {
                if word_set_history.len() > 1 {
                    pattern_stack.remove_last_pattern_line();
                    word_set_history.pop();
                } else {
                    println!("can't undo anymore");
                    press_any_key_to_continue();
                }
            }
            _ => {
                //
                match pattern::PatternLine::try_from(input.as_str()) {
                    Ok(pattern_line) => {
                        //
                        pattern_stack.add_pattern_line(pattern_line).unwrap();
                        let possible_words = pattern_stack.possible_words(&word_set);
                        word_set_history.push(possible_words);
                    }
                    Err(e) => {
                        println!("{:?}", e);
                        press_any_key_to_continue();
                    }
                }
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
    std::io::stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }
    input
}

// ===============================================

fn print_word_sets(sets: &Vec<Vec<&str>>, cols: usize) {
    if sets.len() > 1 {
        for (i, word) in sets.last().unwrap().into_iter().enumerate() {
            if i % cols == 0 && i != 0 {
                print!("\n");
            }
            print!("{}  ", word);
        }
    }
    println!("\n");
}

// ===============================================

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

// ===============================================

fn press_any_key_to_continue() {
    use std::io::Read;
    use std::io::Write;

    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    write!(stdout, "press any key to continue...").unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}
