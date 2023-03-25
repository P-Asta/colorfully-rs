pub mod traits;
pub use traits::*;



impl Property for String{
    /// **make** original **text**
    /// ```rs
    /// let text = "original".to_string
    /// println!("this is {} text",  text.reset.bold());
    /// ```
    fn reset(&self) -> String {
        format!("\x1b[0m{}", self.clone())
    }
    
    /// make **bold** text
    /// ```rs
    /// let text = "bold".to_string
    /// println!("this is {} text",  text.bold());
    /// ```
    fn bold(&self) -> String {
        format!("\x1b[1m{}\x1b[0m", self.clone())
    }

    /// make light text
    /// ```rs
    /// let text = "light".to_string
    /// println!("this is {} text",  text.light());
    /// ```
    fn light(&self) -> String {
        format!("\x1b[2m{}\x1b[0m", self.clone())
    }

    /// make *italic* text
    /// ```rs
    /// let text = "italic".to_string
    /// println!("this is {} text",  text.italic());
    /// ```
    fn italic(&self) -> String {
        format!("\x1b[3m{}\x1b[0m", self.clone())
    }

    /// make __under line__ text
    /// ```rs
    /// let text = "under line".to_string
    /// println!("this is {} text",  text.under_line());
    /// ```
    fn under_line(&self) -> String {
        format!("\x1b[4m{}\x1b[0m", self.clone())
    }
}

impl Colors for String{
    /// make black text
    /// ```rs
    /// let text = "black.to_string()"
    /// println!("{} color",  text.black());
    /// ```
    fn black(&self) -> String {
        format!("\x1b[30m{}\x1b[0m", self.clone())
    }

    /// make red text
    /// ```rs
    /// let text = "red".to_string();
    /// println!("{} color",  text.red());
    /// ```
    fn red(&self) -> String {
        format!("\x1b[31m{}\x1b[0m", self.clone())
    }

    /// make green text
    /// ```rs
    /// let text = "green.to_string()"
    /// println!("{} color",  text.green());
    /// ```
    fn green(&self) -> String {
        format!("\x1b[32m{}\x1b[0m", self.clone())
    }

    /// make yellow text
    /// ```rs
    /// let text = "yellow".to_string();
    /// println!("{} color",  text.yellow());
    /// ```
    fn yellow(&self) -> String {
        format!("\x1b[33m{}\x1b[0m", self.clone())
    }

    /// make blue text
    /// ```rs
    /// let text = "blue".to_string();
    /// println!("{} color",  text.blue());
    /// ```
    fn blue(&self) -> String {
        format!("\x1b[34m{}\x1b[0m", self.clone())
    }

    /// make purple text
    /// ```rs
    /// let text = "purple".to_string();
    /// println!("{} color",  text.purple());
    /// ```
    fn purple(&self) -> String {
        format!("\x1b[35m{}\x1b[0m", self.clone())
    }

    /// make cyan text
    /// ```rs
    /// let text = "cyan".to_string();
    /// println!("{} color",  text.cyan());
    /// ```
    fn cyan(&self) -> String {
        format!("\x1b[36m{}\x1b[0m", self.clone())
    }

    /// make white text
    /// ```rs
    /// let text = "white".to_string();
    /// println!("{} color",  text.white());
    /// ```
    fn white(&self) -> String {
        format!("\x1b[37m{}\x1b[0m", self.clone())
    }

    /// make gray text
    /// ```rs
    /// let text = "gray".to_string();
    /// println!("{} color",  text.gray());
    /// ```
    fn gray(&self) -> String {
        format!("\x1b[38m{}\x1b[0m", self.clone())
    }

    /// make black background text
    /// ```rs
    /// let text = "black background".to_string();
    /// println!("{}",  text.bg_black());
    /// ```
    fn bg_black(&self) -> String {
        format!("\x1b[40m{}\x1b[0m", self.clone())
    }

    /// make red background text
    /// ```rs
    /// let text = "red background".to_string();
    /// println!("{}",  text.bg_red());
    /// ```
    fn bg_red(&self) -> String {
        format!("\x1b[41m{}\x1b[0m", self.clone())
    }

    /// make green background text
    /// ```rs
    /// let text = "green background".to_string();
    /// println!("{}",  text.bg_green());
    /// ```
    fn bg_green(&self) -> String {
        format!("\x1b[42m{}\x1b[0m", self.clone())
    }

    /// make yellow background text
    /// ```rs
    /// let text = "yellow background".to_string();
    /// println!("{}",  text.bg_yellow());
    /// ```
    fn bg_yellow(&self) -> String {
        format!("\x1b[43m{}\x1b[0m", self.clone())
    }

    /// make blue background text
    /// ```rs
    /// let text = "blue background".to_string();
    /// println!("{}",  text.bg_blue());
    /// ```
    fn bg_blue(&self) -> String {
        format!("\x1b[44m{}\x1b[0m", self.clone())
    }

    /// make purple background text
    /// ```rs
    /// let text = "purple background".to_string();
    /// println!("{}",  text.bg_purple());
    /// ```
    fn bg_purple(&self) -> String {
        format!("\x1b[45m{}\x1b[0m", self.clone())
    }

    /// make cyan background text
    /// ```rs
    /// let text = "cyan background".to_string();
    /// println!("{}",  text.bg_cyan());
    /// ```
    fn bg_cyan(&self) -> String {
        format!("\x1b[46m{}\x1b[0m", self.clone())
    }

    /// make white background text
    /// ```rs
    /// let text = "white background".to_string();
    /// println!("{}",  text.bg_white());
    /// ```
    fn bg_white(&self) -> String {
        format!("\x1b[47m{}\x1b[0m", self.clone())
    }

    /// make gray background text
    /// ```rs
    /// let text = "gray background".to_string();
    /// println!("{}",  text.bg_gray());
    /// ```
    fn bg_gray(&self) -> String {
        format!("\x1b[48m{}\x1b[0m", self.clone())
    }

    /// make light black text
    /// ```rs
    /// let text = "black".to_string();
    /// println!("{} color",  text.lighter_black());
    /// ```
    fn lighter_black(&self) -> String {
        format!("\x1b[90m{}\x1b[0m", self.clone())
    }
    /// make light red text
    /// ```rs
    /// let text = "red".to_string();
    /// println!("{} color",  text.lighter_red());
    /// ```
    fn lighter_red(&self) -> String {
        format!("\x1b[91m{}\x1b[0m", self.clone())
    }
    /// make light green text
    /// ```rs
    /// let text = "green".to_string();
    /// println!("{} color",  text.lighter_green());
    /// ```
    fn lighter_green(&self) -> String {
        format!("\x1b[92m{}\x1b[0m", self.clone())
    }
    /// make light yellow text
    /// ```rs
    /// let text = "yellow".to_string();
    /// println!("{} color",  text.lighter_yellow());
    /// ```
    fn lighter_yellow(&self) -> String {
        format!("\x1b[93m{}\x1b[0m", self.clone())
    }
    /// make light blue text
    /// ```rs
    /// let text = "blue".to_string();
    /// println!("{} color",  text.lighter_blue());
    /// ```
    fn lighter_blue(&self) -> String {
        format!("\x1b[94m{}\x1b[0m", self.clone())
    }
    /// make light purple text
    /// ```rs
    /// let text = "purple".to_string();
    /// println!("{} color",  text.lighter_purple());
    /// ```
    fn lighter_purple(&self) -> String {
        format!("\x1b[95m{}\x1b[0m", self.clone())
    }
    /// make light cyan text
    /// ```rs
    /// let text = "cyan".to_string();
    /// println!("{} color",  text.lighter_cyan());
    /// ```
    fn lighter_cyan(&self) -> String {
        format!("\x1b[96m{}\x1b[0m", self.clone())
    }
    /// make light white text
    /// ```rs
    /// let text = "white".to_string();
    /// println!("{} color",  text.lighter_white());
    /// ```
    fn lighter_white(&self) -> String {
        format!("\x1b[97m{}\x1b[0m", self.clone())
    }
    /// make light gray text
    /// ```rs
    /// let text = "gray".to_string();
    /// println!("{} color",  text.lighter_gray());
    /// ```
    fn lighter_gray(&self) -> String {
        format!("\x1b[98m{}\x1b[0m", self.clone())
    }

    /// make lighter black background text
    /// ```rs
    /// let text = "black background".to_string();
    /// println!("{}",  text.lighter_bg_black());
    /// ```
    fn lighter_bg_black(&self) -> String {
        format!("\x1b[100m{}\x1b[0m", self.clone())
    }
    /// maklighter e red background text
    /// ```rs
    /// let text = "red background".to_string();
    /// println!("{}",  text.lighter_bg_red());
    /// ```
    fn lighter_bg_red(&self) -> String {
        format!("\x1b[101m{}\x1b[0m", self.clone())
    }
    /// make lighter green background text
    /// ```rs
    /// let text = "green background".to_string();
    /// println!("{}",  text.lighter_bg_green());
    /// ```
    fn lighter_bg_green(&self) -> String {
        format!("\x1b[102m{}\x1b[0m", self.clone())
    }
    /// make ylighter ellow background text
    /// ```rs
    /// let text = "yellow background".to_string();
    /// println!("{}",  text.lighter_bg_yellow());
    /// ```
    fn lighter_bg_yellow(&self) -> String {
        format!("\x1b[103m{}\x1b[0m", self.clone())
    }
    /// makelighter  blue background text
    /// ```rs
    /// let text = "blue background".to_string();
    /// println!("{}",  text.lighter_bg_blue());
    /// ```
    fn lighter_bg_blue(&self) -> String {
        format!("\x1b[104m{}\x1b[0m", self.clone())
    }
    /// make plighter urple background text
    /// ```rs
    /// let text = "purple background".to_string();
    /// println!("{}",  text.lighter_bg_purple());
    /// ```
    fn lighter_bg_purple(&self) -> String {
        format!("\x1b[105m{}\x1b[0m", self.clone())
    }
    /// makelighter  cyan background text
    /// ```rs
    /// let text = "cyan background".to_string();
    /// println!("{}",  text.lighter_bg_cyan());
    /// ```
    fn lighter_bg_cyan(&self) -> String {
        format!("\x1b[106m{}\x1b[0m", self.clone())
    }
    /// make lighter white background text
    /// ```rs
    /// let text = "white background".to_string();
    /// println!("{}",  text.lighter_bg_white());
    /// ```
    fn lighter_bg_white(&self) -> String {
        format!("\x1b[107m{}\x1b[0m", self.clone())
    }
    /// makelighter  gray background text
    /// ```rs
    /// let text = "gray background".to_string();
    /// println!("{}",  text.lighter_bg_gray());
    /// ```
    fn lighter_bg_gray(&self) -> String {
        format!("\x1b[108m{}\x1b[0m", self.clone())
    }


}


impl Property for str{
    /// **make** original **text**
    /// ```rs
    /// let text = "original"
    /// println!("this is {} text",  text.reset.bold());
    /// ```
    fn reset(&self) -> String {
        format!("\x1b[0m{}", self.clone())
    }
    
    /// make **bold** text
    /// ```rs
    /// let text = "bold"
    /// println!("this is {} text",  text.bold());
    /// ```
    fn bold(&self) -> String {
        format!("\x1b[1m{}\x1b[0m", self.clone())
    }

    /// make light text
    /// ```rs
    /// let text = "light"
    /// println!("this is {} text",  text.light());
    /// ```
    fn light(&self) -> String {
        format!("\x1b[2m{}\x1b[0m", self.clone())
    }

    /// make *italic* text
    /// ```rs
    /// let text = "italic"
    /// println!("this is {} text",  text.italic());
    /// ```
    fn italic(&self) -> String {
        format!("\x1b[3m{}\x1b[0m", self.clone())
    }

    /// make __under line__ text
    /// ```rs
    /// let text = "under line"
    /// println!("this is {} text",  text.under_line());
    /// ```
    fn under_line(&self) -> String {
        format!("\x1b[4m{}\x1b[0m", self.clone())
    }
}

impl Colors for str{
    /// make black text
    /// ```rs
    /// let text = "black"
    /// println!("{} color",  text.black());
    /// ```
    fn black(&self) -> String {
        format!("\x1b[30m{}\x1b[0m", self.clone())
    }

    /// make red text
    /// ```rs
    /// let text = "red"
    /// println!("{} color",  text.red());
    /// ```
    fn red(&self) -> String {
        format!("\x1b[31m{}\x1b[0m", self.clone())
    }

    /// make green text
    /// ```rs
    /// let text = "green"
    /// println!("{} color",  text.green());
    /// ```
    fn green(&self) -> String {
        format!("\x1b[32m{}\x1b[0m", self.clone())
    }

    /// make yellow text
    /// ```rs
    /// let text = "yellow"
    /// println!("{} color",  text.yellow());
    /// ```
    fn yellow(&self) -> String {
        format!("\x1b[33m{}\x1b[0m", self.clone())
    }

    /// make blue text
    /// ```rs
    /// let text = "blue"
    /// println!("{} color",  text.blue());
    /// ```
    fn blue(&self) -> String {
        format!("\x1b[34m{}\x1b[0m", self.clone())
    }

    /// make purple text
    /// ```rs
    /// let text = "purple"
    /// println!("{} color",  text.purple());
    /// ```
    fn purple(&self) -> String {
        format!("\x1b[35m{}\x1b[0m", self.clone())
    }

    /// make cyan text
    /// ```rs
    /// let text = "cyan"
    /// println!("{} color",  text.cyan());
    /// ```
    fn cyan(&self) -> String {
        format!("\x1b[36m{}\x1b[0m", self.clone())
    }

    /// make white text
    /// ```rs
    /// let text = "white"
    /// println!("{} color",  text.white());
    /// ```
    fn white(&self) -> String {
        format!("\x1b[37m{}\x1b[0m", self.clone())
    }

    /// make gray text
    /// ```rs
    /// let text = "gray"
    /// println!("{} color",  text.gray());
    /// ```
    fn gray(&self) -> String {
        format!("\x1b[38m{}\x1b[0m", self.clone())
    }

    /// make black background text
    /// ```rs
    /// let text = "black background"
    /// println!("{}",  text.bg_black());
    /// ```
    fn bg_black(&self) -> String {
        format!("\x1b[40m{}\x1b[0m", self.clone())
    }

    /// make red background text
    /// ```rs
    /// let text = "red background"
    /// println!("{}",  text.bg_red());
    /// ```
    fn bg_red(&self) -> String {
        format!("\x1b[41m{}\x1b[0m", self.clone())
    }

    /// make green background text
    /// ```rs
    /// let text = "green background"
    /// println!("{}",  text.bg_green());
    /// ```
    fn bg_green(&self) -> String {
        format!("\x1b[42m{}\x1b[0m", self.clone())
    }

    /// make yellow background text
    /// ```rs
    /// let text = "yellow background"
    /// println!("{}",  text.bg_yellow());
    /// ```
    fn bg_yellow(&self) -> String {
        format!("\x1b[43m{}\x1b[0m", self.clone())
    }

    /// make blue background text
    /// ```rs
    /// let text = "blue background"
    /// println!("{}",  text.bg_blue());
    /// ```
    fn bg_blue(&self) -> String {
        format!("\x1b[44m{}\x1b[0m", self.clone())
    }

    /// make purple background text
    /// ```rs
    /// let text = "purple background"
    /// println!("{}",  text.bg_purple());
    /// ```
    fn bg_purple(&self) -> String {
        format!("\x1b[45m{}\x1b[0m", self.clone())
    }

    /// make cyan background text
    /// ```rs
    /// let text = "cyan background"
    /// println!("{}",  text.bg_cyan());
    /// ```
    fn bg_cyan(&self) -> String {
        format!("\x1b[46m{}\x1b[0m", self.clone())
    }

    /// make white background text
    /// ```rs
    /// let text = "white background"
    /// println!("{}",  text.bg_white());
    /// ```
    fn bg_white(&self) -> String {
        format!("\x1b[47m{}\x1b[0m", self.clone())
    }

    /// make gray background text
    /// ```rs
    /// let text = "gray background"
    /// println!("{}",  text.bg_gray());
    /// ```
    fn bg_gray(&self) -> String {
        format!("\x1b[48m{}\x1b[0m", self.clone())
    }

    /// make light black text
    /// ```rs
    /// let text = "black"
    /// println!("{} color",  text.lighter_black());
    /// ```
    fn lighter_black(&self) -> String {
        format!("\x1b[90m{}\x1b[0m", self.clone())
    }
    /// make light red text
    /// ```rs
    /// let text = "red"
    /// println!("{} color",  text.lighter_red());
    /// ```
    fn lighter_red(&self) -> String {
        format!("\x1b[91m{}\x1b[0m", self.clone())
    }
    /// make light green text
    /// ```rs
    /// let text = "green"
    /// println!("{} color",  text.lighter_green());
    /// ```
    fn lighter_green(&self) -> String {
        format!("\x1b[92m{}\x1b[0m", self.clone())
    }
    /// make light yellow text
    /// ```rs
    /// let text = "yellow"
    /// println!("{} color",  text.lighter_yellow());
    /// ```
    fn lighter_yellow(&self) -> String {
        format!("\x1b[93m{}\x1b[0m", self.clone())
    }
    /// make light blue text
    /// ```rs
    /// let text = "blue"
    /// println!("{} color",  text.lighter_blue());
    /// ```
    fn lighter_blue(&self) -> String {
        format!("\x1b[94m{}\x1b[0m", self.clone())
    }
    /// make light purple text
    /// ```rs
    /// let text = "purple"
    /// println!("{} color",  text.lighter_purple());
    /// ```
    fn lighter_purple(&self) -> String {
        format!("\x1b[95m{}\x1b[0m", self.clone())
    }
    /// make light cyan text
    /// ```rs
    /// let text = "cyan"
    /// println!("{} color",  text.lighter_cyan());
    /// ```
    fn lighter_cyan(&self) -> String {
        format!("\x1b[96m{}\x1b[0m", self.clone())
    }
    /// make light white text
    /// ```rs
    /// let text = "white"
    /// println!("{} color",  text.lighter_white());
    /// ```
    fn lighter_white(&self) -> String {
        format!("\x1b[97m{}\x1b[0m", self.clone())
    }
    /// make light gray text
    /// ```rs
    /// let text = "gray"
    /// println!("{} color",  text.lighter_gray());
    /// ```
    fn lighter_gray(&self) -> String {
        format!("\x1b[98m{}\x1b[0m", self.clone())
    }

    /// make lighter black background text
    /// ```rs
    /// let text = "black background"
    /// println!("{}",  text.lighter_bg_black());
    /// ```
    fn lighter_bg_black(&self) -> String {
        format!("\x1b[100m{}\x1b[0m", self.clone())
    }
    /// maklighter e red background text
    /// ```rs
    /// let text = "red background"
    /// println!("{}",  text.lighter_bg_red());
    /// ```
    fn lighter_bg_red(&self) -> String {
        format!("\x1b[101m{}\x1b[0m", self.clone())
    }
    /// make lighter green background text
    /// ```rs
    /// let text = "green background"
    /// println!("{}",  text.lighter_bg_green());
    /// ```
    fn lighter_bg_green(&self) -> String {
        format!("\x1b[102m{}\x1b[0m", self.clone())
    }
    /// make ylighter ellow background text
    /// ```rs
    /// let text = "yellow background"
    /// println!("{}",  text.lighter_bg_yellow());
    /// ```
    fn lighter_bg_yellow(&self) -> String {
        format!("\x1b[103m{}\x1b[0m", self.clone())
    }
    /// makelighter  blue background text
    /// ```rs
    /// let text = "blue background"
    /// println!("{}",  text.lighter_bg_blue());
    /// ```
    fn lighter_bg_blue(&self) -> String {
        format!("\x1b[104m{}\x1b[0m", self.clone())
    }
    /// make plighter urple background text
    /// ```rs
    /// let text = "purple background"
    /// println!("{}",  text.lighter_bg_purple());
    /// ```
    fn lighter_bg_purple(&self) -> String {
        format!("\x1b[105m{}\x1b[0m", self.clone())
    }
    /// makelighter  cyan background text
    /// ```rs
    /// let text = "cyan background"
    /// println!("{}",  text.lighter_bg_cyan());
    /// ```
    fn lighter_bg_cyan(&self) -> String {
        format!("\x1b[106m{}\x1b[0m", self.clone())
    }
    /// make lighter white background text
    /// ```rs
    /// let text = "white background"
    /// println!("{}",  text.lighter_bg_white());
    /// ```
    fn lighter_bg_white(&self) -> String {
        format!("\x1b[107m{}\x1b[0m", self.clone())
    }
    /// makelighter  gray background text
    /// ```rs
    /// let text = "gray background"
    /// println!("{}",  text.lighter_bg_gray());
    /// ```
    fn lighter_bg_gray(&self) -> String {
        format!("\x1b[108m{}\x1b[0m", self.clone())
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = "Colors";
        assert_eq!("\x1b[1mColors\x1b[0m".to_string(), result.bold());
    }
}
