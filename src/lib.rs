mod commands;
mod drawing_command;
mod font;

pub mod prelude {
    pub use crate::commands::Commands;
    pub use crate::drawing_command::{Canvas, DrawingCommand, Metrics};
    pub use crate::font::Font;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
