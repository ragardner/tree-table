// Create a struct and a new() function
#[macro_export]
macro_rules! define_struct_with_new {
    ($struct_name:ident { $($field:ident: $ty:ty $(= $init:expr)?),* }) => {
        #[derive(Debug)]
        pub struct $struct_name {
            $(pub $field: $ty),*
        }

        impl $struct_name {
            pub fn new() -> Self {
                $struct_name {
                    $(
                        $field: {
                            #[allow(unused_assignments)]
                            #[allow(unused_mut)]
                            {
                                let mut tmp = <$ty>::new();
                                $(tmp = $init;)?
                                tmp
                            }
                        }
                    ),*
                }
            }
        }
    };
}

// Create a HashMap with string keys and generic values
#[macro_export]
macro_rules! hm_str_key {
    ($( $key:expr => $value:expr ),* $(,)?) => {
        {
            let mut map: hashbrown::HashMap<String, _> = hashbrown::HashMap::new();
            $(
                map.insert($key.to_string(), $value);
            )*
            map
        }
    };
}

// Create a HashMap with string keys and string values
#[macro_export]
macro_rules! hm_str {
    ($( $key:expr => $value:expr ),* $(,)?) => {
        {
            let mut map: hashbrown::HashMap<String, String> = hashbrown::HashMap::new();
            $(
                map.insert($key.to_string(), $value.to_string());
            )*
            map
        }
    };
}

// Create a HashMap with raw key-value pairs (no conversion)
#[macro_export]
macro_rules! hm_raw {
    ($( $key:expr => $value:expr ),* $(,)?) => {
        {
            let mut map: hashbrown::HashMap<_, _> = hashbrown::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

// log to console
#[macro_export]
macro_rules! log {
    // Single string literal
    ($lit:literal) => {
        #[cfg(feature = "std")]
        ::std::println!($lit);
    };
    // Single expression with Debug formatting
    ($expr:expr) => {
        #[cfg(feature = "std")]
        ::std::println!("{:?}", $expr);
    };
    // Formatted string with arguments
    ($fmt:literal, $($arg:expr),*) => {
        #[cfg(feature = "std")]
        ::std::println!($fmt, $($arg),*);
    };
    // Iterable as a Vec for printing
    (array: $expr:expr) => {
        #[cfg(feature = "std")]
        ::std::println!("{:?}", $expr.collect::<Vec<_>>());
    };
    // Flexible logging
    ($($t:tt)*) => {
        #[cfg(feature = "std")]
        ::std::println!($($t)*);
    };
}

// Properly formats a String Err. Supports literal strings and format! syntax.
#[macro_export]
macro_rules! str_err {
    ($($arg:tt)*) => {
        ::alloc::format!("Error at {}:{}: {}", file!(), line!(), ::alloc::format!($($arg)*))
    };
}
