use serde::{Serialize, Deserialize};

#[cfg(feature = "sides")]
macro_rules! enum_str {
    (enum $name:ident {
        $($variant:ident = $val:expr),*,
    }) => {
        #[allow(non_camel_case_types)]
        pub enum $name {
            $($variant = $val),*
        }

        impl $name {
            pub fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };
}

#[cfg(feature = "sides")]
enum_str! {
    enum Side{
        up = 0,
        down = 1,
        north = 2,
        east = 3,
        south = 4,
        west = 5,
    }
}

#[cfg(feature = "item")]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Item{
    id: String,
    #[allow(non_snake_case)]
    Count: usize,
}
