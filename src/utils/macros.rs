#[macro_export]
macro_rules! read_input {
    ($day:literal) => {{
        read_input!("../../input", $day)
    }};
    ($path:literal, $day:literal) => {{
        include_str!(
            concat!(
                $path,
                "/input",
                stringify!($day),
                ".txt"
            )
        )
    }};
}
