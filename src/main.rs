#![feature(trace_macros)]
trace_macros!(true);

// Processing starts here -- Process list of supplied error variants
#[macro_export]
macro_rules! def_err {
    ($err_typename:ident { $( $rest:tt )+ }) => {
        //Preamble
//        use std::error::Error as StdError;
//        use std::fmt;

        // Build enum
        //#[derive(Clone, PartialEq, PartialOrd)]
        //pub enum $err_typename {
            def_variant!($( $rest )+)
        //}

        // Build display
//        println!("build display: {}", stringify!($err_typename));

        // Build debug
//        println!("build debug: {}", stringify!($err_typename));

        // Build coercions
//        println!("build coercions: {}", stringify!($err_typename));
    };

    ($err_variant:ident $( $rest:tt )*) => {
        def_err!(Error { $err_variant $( $rest )* });
    };
}

macro_rules! def_variant {
    ($err_variant:ident $( $rest:tt )*) => {
        println!("variant: {}", stringify!($err_variant));
        def_param!($( $rest )*)
    };
}

macro_rules! def_param {
    (, $( $rest:tt )*) => {
        def_variant!($( $rest )*)
    };

    (($err_param:ident : $( $rest:tt )*),) => {
        println!("param: {}", stringify!($err_param));
        def_paramtype!($( $rest )*)
    };
}

macro_rules! def_paramtype {
    ($err_paramtype:tt $( $rest:tt )*) => {
        println!("paramtype: {}", stringify!($err_paramtype));
    }
}

macro_rules! def_err1 {
    // Explicit err_typename: invoke variant corrector for each variant specified
    ( $err_typename:ident { $( $err_variants:ident $( $err_params:tt )* ),+ } ) => {
        stringify!($( stringify!($err_variants $( $err_params )*)),+)
    };

    // Explicit err_typename: re-invoke def_err!, but without variant list trailing comma
    ( $err_typename:ident { $( $err_variants:ident $( $err_params:tt )*, )+ } ) => {
        stringify!($err_typename { $( $err_variants $( $err_params )* ),+ } );
    };

    // Implicit err_typename: re-invoke def_err!, supplying `Error` as default err_typename
    ( $( $err_variants:ident $err_params:tt ),+ ) => {
        stringify!(Error { $( $err_variants $( $err_params )* ),+ })
    };

    // Implicit err_typename: re_invoke def_err!, removing variant list trailing comma
    ( $( $err_variants:ident $( $err_params:tt )*, )+ ) => {
       stringify!($( $err_variants $( $err_params )* ),+ )
    };
}

// Convert "naked" variants to variants with empty parents (e.g. `Foo` -> `Foo()`) to simplify parsing
macro_rules! def_err2 {
    ($err_variant:ident) => {
        $err_variant()
    };

    ($err_variant:ident $err_params:tt) => {
        $err_variant $err_params
    };
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
            $( def_err4!($err_variants $err_params); )+
//        }
    };

    // Provide `Error` as the default typename for the "Error" type when none is specified
    ( $( $err_variants:ident $err_params:tt ),+ ) => {
        def_err!(Error { $( $err_variants $err_params),+ })
    };

    // Explicit error typename: Ensure final error declarations does not have trailing comma (eliminates macro_rules! ambiguity)
    ( $err_typename:ident { $( $err_variants:ident $err_params:tt, )+ } ) => {
        def_err!($err_typename { $( $err_variants $err_params ),+ })
    };

    // Implicit error typename: Ensure final error declarations does not have trailing comma (eliminates macro_rules! ambiguity)
    ( $( $err_variants:ident $err_params:tt, )+ ) => {
        def_err!($( $err_variants $err_params),+)
    };
}

macro_rules! def_err3 {
    ( $err_variant:ident () ) => {
        println!("No params: {}", stringify!($err_variant)) //DEBUG
    };

    ( $err_decl:ident ($( $params:ident: $param_types:ty ),+) ) => {
        println!("Args: {}", stringify!($err_decl)) //DEBUG
    };

    ( $err_decl:ident ({ $cause:expr }) ) => {
        println!("Cause: {}", stringify!($err_decl)) //DEBUG
    };

    ( $err_decl:ident ($( $params:ident: $param_types:tt ),+ ; { $cause:expr }) ) => {
        println!("Args, Cause: {}", stringify!($err_decl)) //DEBUG
    };

    ( $err_decl:ident ($( $params:ident: $param_types:tt ),+ ; { $cause:expr }, $( $disp:tt ),+) ) => {
        println!("Args, Cause, Display: {}", stringify!($err_decl)) //DEBUG
    };

    ( $err_decl:ident ($( $disp:tt ),+) ) => {
        println!("Display: {}", stringify!($err_decl)) //DEBUG
    };

    ( $err_decl:ident ($( $params:ident: $param_types:tt ),+ ; $( $disp:tt ),+) ) => {
        println!("Args, Display: {}", stringify!($err_decl)) //DEBUG
    };
}

fn main() {
    def_err! {
        SampleErr1,
        SampleErr2(foo: u64, bar: f32),
//        SampleErr3("my message"),
//        SampleErr4(foo: u32, bar: f64; "another message: {}, {}", foo, bar),
////        SampleErr1b,
//        SampleErr5({std::fmt::Error}),
//        SampleErr6(foo: u8, bar: String; {std::fmt::Error}),
//        SampleErr7(baz: i64; {std::fmt::Error}, "and another message (baz: {})", baz),
////        SampleErr1c,
    };
}
