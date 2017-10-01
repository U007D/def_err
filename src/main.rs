macro_rules! def_err {
    ( $err_type:ty { $( $err:ident ),+ } ) => {
        let crate_name = String::from(env!("CARGO_PKG_NAME"));
        let error_type = stringify!($err_type);
        $( println!("{}::{}::{}", crate_name, error_type, stringify!($err)); ),+
    };

    ( $( $err:ident ),+ ) => {
        def_err! { Error { $( $err ),+ } };
    };
}

pub fn main() {
    def_err! {
        Foo {
            MyVariant
        }
    };

    def_err! {
        MyVariant2
    };
}
