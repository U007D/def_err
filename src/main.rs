#![feature(trace_macros)]
trace_macros!(true);

// Processing starts here -- Process list of supplied error variants
#[macro_export]
macro_rules! def_err {
    // Explicit err_typename: invoke variant corrector for each variant specified
    ( $err_typename:ident { $( $err_variants:ident $err_params:tt ),+ } ) => {
        def_err3!($( def_err2!($err_variants $err_params)));
    };

    // Explicit err_typename: re-invoke def_err!, but without variant list trailing comma
    ( $err_typename:ident { $( $err_variants:ident $err_params:tt, )+ } ) => {
        def_err!($( $err_typename ),+);
    };

    // Implicit err_typename: re-invoke def_err!, supplying `Error` as default err_typename
    ( $( $err_variants:ident $err_params:tt ),+ ) => {
       def_err!(Error { $( $err_variants $err_params),+ });
    };

    // Implicit err_typename: re_invoke def_err!, supplying `Error` as default err_typename, retaining variant list
    // trailing comma
    ( $( $err_variants:ident $err_params:tt, )+ ) => {
       def_err!(Error { $( $err_variants $err_params,)+ });
    };
}

// Correct or standardize supplied variants.  For example, `MyVariantA` is both a more ergonomic and Rustic (idiomatic
// in Rust) definition style for variants than `MyVariantB()`, but the latter simplifies parsing, since variants may
// posess arguments (e.g. `MyVariantC(foo: f64)`).
// By converting the ergonomic form into the consistently parsable form automatically, we get the best of both worlds.
macro_rules! def_err2 {

}

macro_rules! def_err3 {
    // Define error variants given an explicit error typename
    ( $err_typename:ident { $( $err_variants:ident $err_params:tt ),+ } ) => {
        use ::std::error::Error as StdError;
        use ::std::fmt;
        type Result<T> = ::std::result::Result<T, fmt::Error>;

        let crate_name = String::from(env!("CARGO_PKG_NAME"));

        let error_type = stringify!($err_typename);

//        #[derive(Clone, PartialEq, PartialOrd)]
//        pub enum $err_typename {$disp:expr
            $( def_err4!($err_variants $err_params); )+;
//        }
    };

    // Provide `Error` as the default typename for the "Error" type when none is specified
    ( $( $err_variants:ident $err_params:tt ),+ ) => {
        def_err!(Error { $( $err_variants $err_params),+ });
    };

    // Explicit error typename: Ensure final error declarations does not have trailing comma (eliminates macro_rules! ambiguity)
    ( $err_typename:ident { $( $err_variants:ident $err_params:tt, )+ } ) => {
        def_err!($err_typename { $( $err_variants $err_params ),+ });
    };

    // Implicit error typename: Ensure final error declarations does not have trailing comma (eliminates macro_rules! ambiguity)
    ( $( $err_variants:ident $err_params:tt, )+ ) => {
        def_err!($( $err_variants $err_params),+);
    };
}

macro_rules! def_err3 {
    ( $err_variant:ident () ) => {
        println!("No params: {}", stringify!($err_variant)); //DEBUG
    };

    ( $err_decl:ident ($( $params:ident: $param_types:ty ),+) ) => {
        println!("Args: {}", stringify!($err_decl)); //DEBUG
    };

    ( $err_decl:ident ({ $cause:expr }) ) => {
        println!("Cause: {}", stringify!($err_decl)); //DEBUG
    };

    ( $err_decl:ident ($( $params:ident: $param_types:tt ),+ ; { $cause:expr }) ) => {
        println!("Args, Cause: {}", stringify!($err_decl)); //DEBUG
    };

    ( $err_decl:ident ($( $params:ident: $param_types:tt ),+ ; { $cause:expr }, $( $disp:tt ),+) ) => {
        println!("Args, Cause, Display: {}", stringify!($err_decl)); //DEBUG
    };

    ( $err_decl:ident ($( $disp:tt ),+) ) => {
        println!("Display: {}", stringify!($err_decl)); //DEBUG
    };

    ( $err_decl:ident ($( $params:ident: $param_types:tt ),+ ; $( $disp:tt ),+) ) => {
        println!("Args, Display: {}", stringify!($err_decl)); //DEBUG
    };
}

fn main() {
    def_err! {
        SampleErr1(),
        SampleErr2(foo: u64, bar: f32),
        SampleErr3("my message"),
        SampleErr4(foo: u32, bar: f64; "another message: {}, {}", foo, bar),
//        SampleErr1b,
        SampleErr5({std::fmt::Error}),
        SampleErr6(foo: u8, bar: String; {std::fmt::Error}),
        SampleErr7(baz: i64; {std::fmt::Error}, "and another message (baz: {})", baz),
//        SampleErr1c,
    };
}
