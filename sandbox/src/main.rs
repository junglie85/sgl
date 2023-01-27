use meh::{MehError, Screen, Vk};

fn main() -> Result<(), MehError> {
    let mut screen = Screen::new(320, 240, "Sandbox")?;
    while !screen.closed() && !screen.key_down(Vk::Escape) {
        screen.update();
    }

    Ok(())
}
