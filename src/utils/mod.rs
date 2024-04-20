pub mod terminal {

    pub enum Color {
        Black,
        Red,
        Green,
        Yellow,
        Blue,
        Magenta,
        Cyan,
        White,
    }
    
    pub enum SGR {
        Bold,
        Dim,
        Italic,
        Underline,
        Rerverse,
        Strike,
    }

    pub trait Colorize {
        // Color
        fn black(&self) -> String;
        fn red(&self) -> String;
        fn green(&self) -> String;
        fn yellow(&self) -> String;
        fn blue(&self) -> String;
        fn magenta(&self) -> String;
        fn cyan(&self) -> String;
        fn white(&self) -> String;
    
        // Background
        fn bg_black(&self) -> String;
        fn bg_red(&self) -> String;
        fn bg_green(&self) -> String;
        fn bg_yellow(&self) -> String;
        fn bg_blue(&self) -> String;
        fn bg_magenta(&self) -> String;
        fn bg_cyan(&self) -> String;
        fn bg_white(&self) -> String;
    
        // SGR
        fn bold(&self) -> String;
        fn dim(&self) -> String;
        fn italic(&self) -> String;
        fn underline(&self) -> String;
        fn strike(&self) -> String;
    
        // Formatted
        fn format_color(&self, color_code: i32) -> String;
    }
    
    pub struct Set;
    pub trait ColorCode {
        fn color(&self, code: Color) -> i32;
        fn bg(&self, code: Color) -> i32;
        fn text(&self, code: SGR) -> i32;
    }
    
    impl ColorCode for Set {
        fn color(&self, code: Color) -> i32 {
            match code {
                Color::Black => 90,
                Color::Red => 91,
                Color::Green => 92,
                Color::Yellow => 93,
                Color::Blue => 94,
                Color::Magenta => 95,
                Color::Cyan => 96,
                Color::White => 97,
            }
        }
    
        fn bg(&self, code: Color) -> i32 {
            match code {
                Color::Black => 100,
                Color::Red => 101,
                Color::Green => 102,
                Color::Yellow => 103,
                Color::Blue => 104,
                Color::Magenta => 105,
                Color::Cyan => 106,
                Color::White => 107,
            }
        }
    
        fn text(&self, code: SGR) -> i32 {
            match code {
                SGR::Bold => 1,
                SGR::Dim => 2,
                SGR::Italic => 3,
                SGR::Underline => 4,
                SGR::Rerverse => 7,
                SGR::Strike => 9,
            }
        }
    }
    
    impl Colorize for str {
        fn black(&self) -> String {
            self.format_color(Set.color(Color::Black))
        }
    
        fn red(&self) -> String {
            self.format_color(Set.color(Color::Red))
        }
    
        fn green(&self) -> String {
            self.format_color(Set.color(Color::Green))
        }
    
        fn yellow(&self) -> String {
            self.format_color(Set.color(Color::Yellow))
        }
    
        fn blue(&self) -> String {
            self.format_color(Set.color(Color::Blue))
        }
    
        fn magenta(&self) -> String {
            self.format_color(Set.color(Color::Magenta))
        }
    
        fn cyan(&self) -> String {
            self.format_color(Set.color(Color::Cyan))
        }
    
        fn white(&self) -> String {
            self.format_color(Set.color(Color::White))
        }
    
        fn bg_black(&self) -> String {
            self.format_color(Set.bg(Color::Black))
        }
    
        fn bg_red(&self) -> String {
            self.format_color(Set.bg(Color::Red))
        }
    
        fn bg_green(&self) -> String {
            self.format_color(Set.bg(Color::Green))
        }
    
        fn bg_yellow(&self) -> String {
            self.format_color(Set.bg(Color::Yellow))
        }
    
        fn bg_blue(&self) -> String {
            self.format_color(Set.bg(Color::Blue))
        }
    
        fn bg_magenta(&self) -> String {
            self.format_color(Set.bg(Color::Magenta))
        }
    
        fn bg_cyan(&self) -> String {
            self.format_color(Set.bg(Color::Cyan))
        }
    
        fn bg_white(&self) -> String {
            self.format_color(Set.bg(Color::White))
        }
    
        fn bold(&self) -> String {
            self.format_color(Set.text(SGR::Bold))
        }
    
        fn dim(&self) -> String {
            self.format_color(Set.text(SGR::Dim))
        }
    
        fn italic(&self) -> String {
            self.format_color(Set.text(SGR::Italic))
        }
    
        fn underline(&self) -> String {
            self.format_color(Set.text(SGR::Underline))
        }
    
        fn strike(&self) -> String {
            self.format_color(Set.text(SGR::Strike))
        }
    
        fn format_color(&self, color_code: i32) -> String {
            format!("\x1b[{}m{}\x1b[0m", color_code, self)
        }
    }
    
}