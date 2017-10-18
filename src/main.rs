#![feature(trace_macros)]
trace_macros!(true);

// Processing starts here -- Process list of supplied error variants
#[macro_export]
macro_rules! def_err {
    ($err_typename:ident { $($err_vardecls:tt)+ }) => {
        //Preamble
//        use std::error::Error as StdError;
//        use std::fmt;

        // Build enum
        //#[derive(Clone, PartialEq, PartialOrd)]
        //pub enum $err_typename {
            parse_vardecls!($( $err_vardecls )+);
        //}

        // Build display
//        println!("build display: {}", stringify!($err_typename));

        // Build debug
//        println!("build debug: {}", stringify!($err_typename));

        // Build coercions
//        println!("build coercions: {}", stringify!($err_typename));
    };

    // Re-invoke def_err! with `Error` as the default typename for the "Error" type when no typename is specified
    ($err_variant:ident $($rest:tt)*) => {
        def_err!(Error { $err_variant $($rest)* });
    };
}

// Parse variant declarations
macro_rules! parse_vardecls {
    // Terminate processing when nothing left to parse
    () => {};

    // Consume vardecl's trailing comma
    (, $($rest:tt)*) => { parse_vardecls!($($rest)*); };

    // Process one vardecl, then re-invoke parse_variants! to parse remaining vardecls
    ($ident:ident $args:tt $($rest:tt)*) => {
//        println!("parse_vardecls:\tident: {}\n\targs: {}", stringify!($ident), stringify!($args));
        parse_variant!($ident $args);
        parse_vardecls!($($rest)*);
    };
}

macro_rules! parse_variant {
    // Terminate processing when nothing left to parse
    () => {};

    // Vardecl has no args
    ($ident:ident ,) => {
        println!("parse_variant: ident: {}", stringify!($ident));
    };

    // Vardecl has no args (empty parentheses)
    ($ident:ident ()) => {
        println!("parse_variant: ident(): {}", stringify!($ident));
    };

    // Vardecl has args
    ($ident:ident ($($args:tt)*)) => {
        println!("parse_variant:\tident(args): {}", stringify!($ident));
        parse_args!($($args)*)
    };
}

macro_rules! parse_args {
    // Terminate processing when nothing left to parse
    () => {};

    // Consume arg's trailing comma
    (, $($rest:tt)*) => { parse_args!($($rest)*); };

    // Process one arg, then re-invoke parse_args! to parse remaining args
    ($ident:ident : $type:tt $($rest:tt)*) => {
        println!("\tparse_args: ident: type: {}: {}", stringify!($ident), stringify!($type));
        parse_args!($($rest)*);
    };
}

fn main() {
    def_err! {
        SampleErr1(),
        SampleErr2(foo: u64, bar: f32),
//        SampleErr3("my message"),
//        SampleErr4(foo: u32, bar: f64; "another message: {}, {}", foo, bar),
//        SampleErr1b,
//        SampleErr5({std::fmt::Error}),
//        SampleErr6(foo: u8, bar: String; {std::fmt::Error}),
//        SampleErr7(baz: i64; {std::fmt::Error}, "and another message (baz: {})", baz),
//        SampleErr1c,
    };
}
