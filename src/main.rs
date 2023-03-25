use colorfully::*;
fn main() {
    println!("{}",  "black".reset().bold());
    println!("{}",    "red".red());
    println!("{}",  "green".green().under_line());
    println!("{}", "yellow".yellow());
    println!("{}",   "blue".blue().light());
    println!("{}",   "cyan".cyan());
    println!("{}", "perple".purple().italic());
    println!("{}",   "gray".gray());

    println!("\n{}",  "back ground black".bg_black().bold());
    println!("{}",    "back ground red".bg_red());
    println!("{}",  "back ground green".bg_green().under_line());
    println!("{}", "back ground yellow".bg_yellow());
    println!("{}",   "back ground blue".bg_blue().light());
    println!("{}",   "back ground cyan".bg_cyan());
    println!("{}", "back ground perple".bg_purple().italic());
    println!("{}",   "back ground gray".bg_gray());

}
