use std::fmt::Write;
use std::sync::OnceLock;
use std::time::Duration;

use dialoguer::console::Style;
use dialoguer::theme::{ColorfulTheme, Theme};
use indicatif::ProgressBar;
use proc_exit::Code;

pub fn theme() -> &'static ColorfulTheme {
    static THEME: OnceLock<ColorfulTheme> = OnceLock::<ColorfulTheme>::new();
    THEME.get_or_init(ColorfulTheme::default)
}

// Default style println.
pub fn println(msg: &str) {
    println!("{}", theme().inactive_item_style.apply_to(msg));
}

// Success style println.
pub fn sprintln(msg: &str) {
    let mut buf = String::new();
    write!(
        &mut buf,
        "{} {}",
        theme().success_prefix,
        theme().prompt_style.apply_to(msg)
    )
    .expect("format should be valid");
    println!("{buf}");
}

// Error style println and exit.
pub fn eprintln(msg: &str) -> ! {
    let mut buf = String::new();
    theme()
        .format_error(&mut buf, msg)
        .expect("format should be valid");
    eprintln!("{buf}");
    Code::FAILURE.process_exit()
}

pub fn get_style_for_weather(description: &str) -> Style {
    let style = Style::new().for_stderr().bold();
    match description.to_lowercase() {
        s if s.contains("clear") => style.color256(4),
        s if s.contains("cloud") => style.color256(7),
        s if s.contains("sun") => style.color256(11),
        s if s.contains("rain") => style.color256(12),
        s if s.contains("wind") => style.color256(14),
        s if s.contains("snow") => style.color256(15),
        _ => style,
    }
}

pub fn with_spinner<F, T>(f: F) -> T
where
    F: Fn() -> T,
{
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(100));
    let data = f();
    spinner.finish_and_clear();
    data
}
