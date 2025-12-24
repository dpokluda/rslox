#[macro_export]
macro_rules! cprintln {
    ($color:expr, $($arg:tt)*) => {{
        use colored::Colorize;
        println!("{}", format!($($arg)*).color($color));
    }};
}

#[macro_export]
macro_rules! cprint {
    ($color:expr, $($arg:tt)*) => {{
        use colored::Colorize;
        print!("{}", format!($($arg)*).color($color));
    }};
}
