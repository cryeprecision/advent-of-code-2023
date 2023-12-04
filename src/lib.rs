#[macro_export]
macro_rules! load_input {
    ($filename:literal) => {
        include_str!(concat!("../../input/", $filename)).trim_end()
    };
}
