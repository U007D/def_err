macro_rules! def_err {
    ( $err:ident ) => {
        let variant = env!("CARGO_PKG_NAME").to_string() + "::Error::" + &stringify!($err);
        println!("{}", variant);
    };
//    ( $( $err:ident),+ , ) => { def_err! { $( $err),* } };
}

pub fn main() {
    def_err!(MyVariant);
}
