mod ricracroe;

use std::error;
use std::fmt;
use std::io;

use std::num;

extern crate azul;
use azul::prelude::*;

#[derive(Debug)]
enum RicracroeError {
    Io(io::Error),
    BadParse(num::ParseIntError),
}

impl fmt::Display for RicracroeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RicracroeError::Io(ref err) => write!(f, "IO error: {}", err),
            RicracroeError::BadParse(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl error::Error for RicracroeError {
    fn description(&self) -> &str {
        match *self {
            RicracroeError::Io(ref err) => err.description(),
            RicracroeError::BadParse(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            RicracroeError::Io(ref err) => Some(err),
            RicracroeError::BadParse(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for RicracroeError {
    fn from(err: io::Error) -> RicracroeError {
        RicracroeError::Io(err)
    }
}

impl From<num::ParseIntError> for RicracroeError {
    fn from(err: num::ParseIntError) -> RicracroeError {
        RicracroeError::BadParse(err)
    }
}

fn main() {
    let app = App::new(ricracroe::RRRGame::new_anysize(3), AppConfig::default());

    macro_rules! CSS_PATH {
        () => {
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/ricracroe.css")
        };
    }
    let css = Css::new_from_str(include_str!(CSS_PATH!()))
        .expect("Failed while parsing interface styling information.");

    let mut options = WindowCreateOptions::default();
    options.state.title = "ricracroe".to_owned();
    options.state.size.dimensions = LogicalSize::new(400.0 as f64, 400.0 as f64);
    options.state.size.min_dimensions = Some(LogicalSize::new(150.0 as f64, 150.0 as f64));
    app.run(Window::new(options, css).unwrap()).unwrap();
}
