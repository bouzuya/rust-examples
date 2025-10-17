#[macro_export]
macro_rules! my_macro {
    (
        pub enum $name:ident {
            $($variant:ident),* $(,)?
        }
    ) => {

        pub enum $name {
            $($variant),*
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$variant => stringify!($variant).fmt(f),)*
                }
            }
        }

        impl $name {
            pub fn variants() -> Vec<&'static str> {
                vec![$(stringify!($variant)),*]
            }
        }
    };
}

pub use my_macro;

#[macro_export]
macro_rules! my_macro2 {
    (
        pub enum $name:ident {
            $($variant:ident($inner:ty)),* $(,)?
        }
    ) => {

        pub enum $name {
            $($variant($inner)),*
        }

        impl $crate::MyErrorTrait for $name {
            fn into_public_error(self) -> $crate::PublicError {
                match self {
                    $(Self::$variant(e) => e.into_public_error(),)*
                }
            }
        }
    };
}

pub use my_macro2;

#[macro_export]
macro_rules! my_macro3(("a") => { "b" });

pub use my_macro3;

#[macro_export]
macro_rules! my_macro4(["a"] => [ "b" ]);

pub use my_macro4;

#[macro_export]
macro_rules! my_macro5({"a"} => [ "b" ]);

pub use my_macro5;
