#![feature(trace_macros)]
trace_macros!(true);

// Processing starts here -- Process list of supplied error variants
#[macro_export]
macro_rules! def_err {
    ($err_typename:ident { $($err_variants:tt)+ }) => {
        //Preamble
//        use std::error::Error as StdError;
//        use std::fmt;

        // Build enum
        //#[derive(Clone, PartialEq, PartialOrd)]
        //pub enum $err_typename {
            $(parse_variant!($err_variants)),+;
        //}

        // Build description
//        println!("build description: {}", stringify!($err_typename));
//        format!("{}::{}::", env!("CARGO_PKG_NAME"), stringify!($err_typename))

        // Build display
//        println!("build display: {}", stringify!($err_typename));

        // Build debug
//        println!("build debug: {}", stringify!($err_typename));

        // Build coercions
//        println!("build coercions: {}", stringify!($err_typename));
    };

    // Re-invoke def_err! with `Error` as the default typename for the "Error" type when no typename is specified
    ($err_variant:ident $($rest:tt)*) => { def_err!(Error { $err_variant $($rest)* }); };
}

macro_rules! parse_variant {
    // Terminate processing when nothing left to parse
//    () => {};

    // Variant has no args
    ($ident:ident ,) => { $ident };

    // Variant has no args (empty parentheses)
    ($ident:ident ()) => { $ident() };

    // Variant has args
    ($ident:ident ($($args:tt)*)) => { ( $(parse_args!($arg0)),+ ) };
}

macro_rules! parse_args {
    // Terminate processing when nothing left to parse
//    () => {};

    // Consume arg's trailing comma
//    (, $($rest:tt)*) => { , parse_args!($($rest)*); };

    // Parse extended parameters (triggered by ;)
    (; $($rest:tt)*) => { /*parse_extparams!($($rest)*);*/ };

    // Parse extended parameters (triggered by { causal_error })
    ({ $cause:ty }) => { /*parse_extparams!($cause,);*/ };

    // Parse extended parameters (triggered by Display str)
    ($($rest:expr)*) => { /*parse_extparams!($($rest)*);*/ };

    // Parse extended parameters (triggered by { causal_error }, Display str)
    ({ $cause:ty }, $($rest:tt)*) => { /*parse_extparams!($cause, $($rest)*);*/ };

    // Process one `ident: type` arg, then re-invoke parse_args! to parse remaining args
    ($ident:ident : $type:tt $($rest:tt)*) => {
        $ident: $type
//        parse_args!($($rest)*);
    };
}

macro_rules! parse_extparams {
    // Terminate processing when nothing left to parse
    () => {};

    // Consume extparam's trailing comma
    (, $($rest:tt)*) => { parse_args!($($rest)*); };

    // Parse cause extparam
    ($cause:ty, $($rest:tt)*) => {
        println!("parse_extparam: cause: {}", stringify!($cause));
        parse_extparams!($($rest)*);
    };

    // Parse Display str extended parameters (always terminal argument due to format str varargs)
    ($($rest:tt)*) => {
        println!("parse_extparam: Display {}", stringify!($($rest)*));
    };

}

fn main() {
    def_err! {
        SampleErr1,
        SampleErr2(foo: u64, bar: f32),
        SampleErr3("my message"),
        SampleErr4(foo: u32, bar: f64; "another message: {}, {}", foo, bar),
        SampleErr5({std::fmt::Error}),
        SampleErr6(foo: u8, bar: String; {std::fmt::Error}),
        SampleErr7(baz: i64; {std::fmt::Error}, "and another message (baz: {})", baz),
        SampleErr8(),
    };
}
