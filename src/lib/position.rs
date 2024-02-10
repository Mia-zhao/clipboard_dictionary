use mouse_position::mouse_position::Mouse;

pub fn get_position() -> (i32, i32) {
    let position = Mouse::get_mouse_position();
        match position {
            Mouse::Position { x, y } => {
                println!("x: {}, y: {}", x, y);
                (x, y)
            },
            Mouse::Error => {
                println!("Error getting mouse position");
                (500, 500)
            }
        }
}