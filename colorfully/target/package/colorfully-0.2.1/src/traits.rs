pub trait Property{
    fn reset (&self) -> String;
    fn bold (&self) -> String;
    fn light (&self) -> String;
    fn italic (&self) -> String;
    fn under_line (&self) -> String;
}


pub trait Colors{
    fn black(&self) -> String;
    fn red(&self) -> String;
    fn green(&self) -> String;
    fn yellow(&self) -> String;
    fn blue(&self) -> String;
    fn cyan(&self) -> String;
    fn purple(&self) -> String;
    fn white(&self) -> String;
    fn gray(&self) -> String;

    fn bg_black(&self) -> String;
    fn bg_red(&self) -> String;
    fn bg_green(&self) -> String;
    fn bg_yellow(&self) -> String;
    fn bg_blue(&self) -> String;
    fn bg_cyan(&self) -> String;
    fn bg_purple(&self) -> String;
    fn bg_white(&self) -> String;
    fn bg_gray(&self) -> String;


    fn lighter_black(&self) -> String;
    fn lighter_red(&self) -> String;
    fn lighter_green(&self) -> String;
    fn lighter_yellow(&self) -> String;
    fn lighter_blue(&self) -> String;
    fn lighter_cyan(&self) -> String;
    fn lighter_purple(&self) -> String;
    fn lighter_white(&self) -> String;
    fn lighter_gray(&self) -> String;

    fn lighter_bg_black(&self) -> String;
    fn lighter_bg_red(&self) -> String;
    fn lighter_bg_green(&self) -> String;
    fn lighter_bg_yellow(&self) -> String;
    fn lighter_bg_blue(&self) -> String;
    fn lighter_bg_cyan(&self) -> String;
    fn lighter_bg_purple(&self) -> String;
    fn lighter_bg_white(&self) -> String;
    fn lighter_bg_gray(&self) -> String;
}


