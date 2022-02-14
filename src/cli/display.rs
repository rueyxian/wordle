use crate::universe;

// ===============================================

pub fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

// ===============================================

pub fn print_help() {
    self::clear_screen();
    println!("{:<20} {}", ":h or :help", "list commands");
    println!("{:<20} {}", ":q or :quit", "quit program");
    println!("{:<20} {}", ":r or :revert", "go to previous pattern");
    println!("{:<20} {}", ":c or :clear", "clear patterns");
    println!("{:<20} {}", "#c !o #r #g ?i", "pattern matching");
    println!("{:<23} {}", "", "# => ⬛️ => not in the word in any spot");
    println!(
        "{:<23} {}",
        "", "? => 🟨 => in the word but in the wrong spot"
    );
    println!(
        "{:<23} {}",
        "", "! => 🟩 => in the word but in the correct spot"
    );
    println!("");
    self::press_any_key_to_continue();
}

// ===============================================

pub fn print_entropy_ranking(universe: &universe::Universe, n: usize) {
    let top = universe.entropy_ranking(n);
    println!("top {}", top.len());
    println!("word      entropy (bits)");
    println!("------------------------");
    for unit in top {
        println!("{}     {:.6}", unit.word(), unit.entropy());
    }
    println!("\n");
}

// ===============================================

pub fn print_calculating(word: &str, curr: usize, max: usize) {
    self::clear_screen();
    const BAR_LEN: usize = 20;
    let percent = curr as f32 / max as f32;
    let count = (percent * BAR_LEN as f32) as usize;

    let bar_filled = std::iter::repeat('=').take(count).collect::<String>();
    let bar_empty = std::iter::repeat(' ')
        .take(BAR_LEN - count)
        .collect::<String>();

    println!(
        "calculating {} [{}>{}] {:.2}%",
        word,
        bar_filled,
        bar_empty,
        percent * 100.0
    );
}

// ===============================================

pub fn press_any_key_to_continue() {
    use std::io::Read;
    use std::io::Write;

    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    write!(stdout, "press any key to continue...").unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}
