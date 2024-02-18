extern crate rust_terminal_colors;

fn main() {
    let colors = rust_terminal_colors::Colors::new(true);

    let message = "Hello, world!";

    println!("{}", colors.bold(message));
    println!("{}", colors.italic(message));
    println!("{}", colors.underline(message));
    println!("{}", colors.inverse(message));
    println!("{}", colors.hidden(message));
    println!("{}", colors.strikethrough(message));
    println!("{}", colors.black(message));
    println!("{}", colors.red(message));
    println!("{}", colors.green(message));
    println!("{}", colors.yellow(message));
    println!("{}", colors.blue(message));
    println!("{}", colors.magenta(message));
    println!("{}", colors.cyan(message));
    println!("{}", colors.white(message));
    println!("{}", colors.gray(message));
    println!("{}", colors.bg_black(message));
    println!("{}", colors.bg_red(message));
    println!("{}", colors.bg_green(message));
    println!("{}", colors.bg_yellow(message));
    println!("{}", colors.bg_blue(message));
    println!("{}", colors.bg_magenta(message));
    println!("{}", colors.bg_cyan(message));
    println!("{}", colors.bg_white(message));
    println!("{}", colors.bg_gray(message));
    println!("{}", colors.bg_reset(message));
    println!("{}", colors.reset(message));
}
