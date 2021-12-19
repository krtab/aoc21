use std::error::Error;

pub type DynResult<T> = Result<T, Box<dyn Error + 'static>>;

#[macro_export]
macro_rules! read_input {
    () => {
        std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/",
            env!("CARGO_BIN_NAME"),
            ".txt"
        ))
        .unwrap()
    };
    (parse) => {{
        let buf = read_input!();
        buf.lines().map(|l| l.parse().unwrap()).collect()
    }};
}
