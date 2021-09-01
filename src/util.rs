use std::error::Error;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn stderr(message: &str) -> Result<(), Box<dyn Error>> {
    let mut stderr = StandardStream::stderr(ColorChoice::Always);
    stderr
        .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
        .unwrap();
    eprintln!("{}", message);

    Ok(())
}
