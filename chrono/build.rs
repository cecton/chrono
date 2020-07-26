#[cfg(feature = "locales")]
extern crate chrono_format;
#[cfg(feature = "locales")]
extern crate pure_rust_locales;

#[cfg(feature = "locales")]
fn generate_localized_datetime_formats() {
    use chrono_format::*;

    let x = StrftimeItems::new("%Y-%m-%d");
    println!("{:?}", x.collect::<Vec<_>>());
}

fn main() {
    #[cfg(feature = "locales")]
    generate_localized_datetime_formats();
}
