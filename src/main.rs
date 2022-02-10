use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();

    let input = args.get(1).ok_or("missing argument")?;

    let (result, isogram) = wordle::search(input.as_str())?;

    print(result, isogram);

    Ok(())
}

fn print(set: Vec<String>, isogram: bool) {
    println!("isogram: {}", isogram);
    println!("results: {}", set.len());
    set.iter().enumerate().for_each(|(i, item)| {
        if i % 9 == 0 && i != 0 {
            print!("\n");
        }
        print!("{}  ", item);
    })
}
