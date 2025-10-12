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
