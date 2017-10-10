#[macro_export]

macro_rules! def_err {
    // Unit error type
    // allow trailing comma on last error definition, given explicit error typename
    ( $err_type:ty { $( $err:ident ),+ ,} ) => {
        def_err! { $err_type { $( $err ),+ } };
    };

    // Allow trailing comma on last error definition, given no explicit error typename
    ( $( $err:ident ),+ ,) => {
        def_err! { $( $err ),+ };
    };

    // Provide `Error` as the default error typename when an explicit error typename is not specified
    ( $( $err:ident ),+ ) => {
        def_err! { Error { $( $err ),+ } };
    };

    // Define error variants given an explicit error typename
    ( $err_type:ty { $( $err:ident ),+ } ) => {
        let crate_name = String::from(env!("CARGO_PKG_NAME"));
        let error_type = stringify!($err_type);
        let errs = vec![ $( stringify!($err) ),+ ];
        println!("{}::{}::{:?}", crate_name, error_type, errs);
    };
}

fn main() {
    def_err! {
        SampleErr1,
    }
}
