#[path = "lib/position.rs"]
mod position;
#[path = "lib/tooltip.rs"]
mod tooltip;
#[path = "lib/dictionary.rs"]
mod dictionary;

use x11_clipboard::Clipboard;

fn main() {
    let clipboard = Clipboard::new().unwrap();
    let mut last = String::new();

    println!("Waiting for selection...");


    loop {
        if let Ok(curr) = clipboard.load_wait(
            clipboard.getter.atoms.primary,
            clipboard.getter.atoms.utf8_string,
            clipboard.getter.atoms.property
        ) {
            let curr = String::from_utf8_lossy(&curr);
            println!("raw string: {}", curr);
            let curr = curr
                .trim_matches('\u{0}')
                .trim();
            if !curr.is_empty() && last != curr {
                last = curr.to_owned();
                println!("Contents of primary selection: {}", last);
                let definition = dictionary::get_definition(&last);
                let (x, y) = position::get_position();
                println!("definition: {definition}");
                tooltip::generate_tooltip(x, y, definition);
                println!("Waiting for selection...");
            }
        }
    }
}