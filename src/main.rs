macro_rules! def_err {
    ( $err:ident ) => { stringify!($err); };
//    ( $( $err:ident),+ , ) => { def_err! { $( $err),* } };
}

pub fn main() {
    println!("hi");

    def_err!(MyVariant);
}
