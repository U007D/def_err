#[macro_export]
macro_rules! main {
    ($main:expr) => {
        fn main() {
            use ::std::io::Write;

            ::std::process::exit(match $main() {
                Ok(r) => $crate::ExitCode::code(r),
                Err(ref e) => {
                    write!(&mut ::std::io::stderr(), "{}", )
                        .expect("Error writing to stderr");

                    1
                }
            });
        }
    };
}

/// Represents a value that can be used as the exit status of the process.
/// See [`quick_main!`](macro.quick_main.html).
pub trait ExitCode {
    /// Returns the value to use as the exit status.
    fn code(self) -> i32;
}

impl ExitCode for i32 {
    fn code(self) -> i32 {
        self
    }
}

impl ExitCode for () {
    fn code(self) -> i32 { EXIT_SUCCESS }
}

macro_rules! def_err {
    //Unit error type
    // allow trailing comma on last error definition, given explicit typename
    ( $err_type:ty { $( $err:ident ),+ ,} ) => {
        def_err! { $err_type { $( $err ),+ } };
    };

    // Allow trailing comma on last error definition, given no typename
    ( $( $err:ident ),+ ,) => {
        def_err! { $( $err ),+ };
    };

    // Provide `Error` as the default typename for the "Error" type when none is specified
    ( $( $err:ident ),+ ) => {
        def_err! { Error { $( $err ),+ } };
    };

    // Define error variants given explicit typename
    ( $err_type:ty { $( $err:ident ),+ } ) => {
        let crate_name = String::from(env!("CARGO_PKG_NAME"));
        let error_type = stringify!($err_type);
        let errs = vec![ $( stringify!($err) ),+ ];
        println!("{}::{}::{:?}", $crate, error_type, errs);
    };
}

pub fn main() {
    def_err! {
        UnitVariant,
//        UnitVariant2(),
    };
}
