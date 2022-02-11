use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    // let args = std::env::args().collect::<Vec<String>>();
    //
    // let input = args.get(1).ok_or("missing argument")?;
    //
    // let result = wordle::search(input.as_str())?;
    //
    // print(result);
    //

    wordle::cli::run()?;

    Ok(())
}

// fn print(set: Vec<String>) {
//     println!("results: {}", set.len());
//     set.iter().enumerate().for_each(|(i, item)| {
//         if i % 9 == 0 && i != 0 {
//             print!("\n");
//         }
//         print!("{}  ", item);
//     })
// }

// fn main() {
//     use std::io::{stdin, stdout, Write};
//
//
//     let mut s = String::new();
//     print!("Please enter some text: ");
//     let _ = stdout().flush();
//     stdin()
//         .read_line(&mut s)
//         .expect("Did not enter a correct string");
//     if let Some('\n') = s.chars().next_back() {
//         s.pop();
//     }
//     if let Some('\r') = s.chars().next_back() {
//         s.pop();
//     }
//     println!("You typed: {}", s);
// }
//
//
// fn clear_screen() {
//     print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
// }
