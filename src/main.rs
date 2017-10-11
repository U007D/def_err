#![feature(trace_macros)]
trace_macros!(true);

#[macro_export]
macro_rules! def_err {
    // Define error variants given an explicit error typename
    ( $err_typename:ident { $( $err_decls:ident $err_params:tt ),+ } ) => {
        use ::std::error::Error as StdError;
        use ::std::fmt;
        type Result<T> = ::std::result::Result<T, fmt::Error>;

        let crate_name = String::from(env!("CARGO_PKG_NAME"));
        let error_type = stringify!($err_typename);

//        pub enum $err_typename {
            $( process_err_decls!($err_decls $err_params); )+;
//        }
    };

    // Provide `Error` as the default typename for the "Error" type when none is specified
    ( $( $err_decls:ident $err_params:tt ),+ ) => {
        def_err!(Error { $( $err_decls $err_params),+ });
    };

    // Explicit error type: Ensure final error declarations does not have trailing comma (eliminates macro_rules! ambiguity)
    ( $err_type:ident { $( $err_decls:ident $err_params:tt ),+ ,} ) => {
        def_err!($err_type { $( $err_decls $err_params ),+ });
    };

    // Implicit error type: Ensure final error declarations does not have trailing comma (eliminates macro_rules! ambiguity)
    ( $( $err_decls:ident $err_params:tt ),+ ,) => {
        def_err!($( $err_decls $err_params),+);
    };

//    // Provide `Error` as the default error typename when an explicit error typename is not specified
//    ( $simple_variant:tt, ) => {
//        println!("{}::{}::{:?}", crate_name, error_type, $simple_variant); //DEBUG
//    };
}

macro_rules! process_err_decls {
    ( $err_decl:ident () ) => {
        println!("No params: {}", stringify!($err_decl)); //DEBUG
    };

    ( $err_decl:ident ($( $params:ident: $param_types:ty ),+) ) => {
        println!("Args: {}", stringify!($err_decl)); //DEBUG
    };

    ( $err_decl:ident ($( $params:ident: $param_types:tt ),* ; $desc:expr) ) => {
        println!("Args, Description: {}", stringify!($err_decl)); //DEBUG
    };

    ( $err_decl:ident ($desc:expr) ) => {
        println!("Description: {}", stringify!($err_decl)); //DEBUG
    };

    ( $err_decl:ident ($( $params:ident: $param_types:tt ),* ; { $cause:expr }, $desc:expr) ) => {
        println!("Cause: {}", stringify!($err_decl)); //DEBUG
    };

    ( $err_decl:ident ($( $params:ident: $param_types:tt ),* ; { $cause:expr }) ) => {
        println!("Cause: {}", stringify!($err_decl)); //DEBUG
    };
}

fn main() {
    def_err! {
        SampleErr1(),
        SampleErr2(foo: u64, bar: f32),
        SampleErr3("my message"),
        SampleErr4(foo: u32, bar: f64; "another message"),
        SampleErr5({std::fmt::Error}),
        SampleErr6(foo: u8, bar: String; {std::fmt::Error}),
        SampleErr7(foo: u8, bar: String, baz: i64; {std::fmt::Error}, "and another message"),
    };
}

//        SampleErr2("my message"),
//        SampleErr3(foo: u32, bar: f64, "another message"),
//        SampleErr4({ std::io::Error }),

