/*!
All macros in the library should be in, or a descendant of, this module.

The exception are macros with limited or very specific use (like those in `features::cc::parse`).
*/
#[macro_use] pub mod bitflags;

macro_rules! as_item {
    ($($i:item)*) => ($($i)*);
}

/**
Used with enums with no payloads, where every variant has a value.

Generates some extra conversion `impl`s automatically.
*/
macro_rules! c_enum {
    (
        $(#[$attrs:meta])*
        pub enum $name:ident: $base_ty:ty {
            $($var_name:ident = $var_value:expr),+
            $(,)*
        }
    ) => {
        $(#[$attrs])*
        pub enum $name {
            $($var_name = $var_value),+
        }

        impl TryFrom<$base_ty> for $name {
            fn try_from(v: $base_ty) -> Option<$name> {
                match v {
                    $($var_value => Some($name::$var_name),)+
                    _ => None
                }
            }
        }

        impl From<$base_ty> for $name {
            fn from(v: $base_ty) -> $name {
                TryFrom::try_from(v).unwrap()
            }
        }

        impl From<$name> for $base_ty {
            fn from(v: $name) -> $base_ty {
                match v {
                    $($name::$var_name => $var_value,)+
                }
            }
        }
    };
}

/**
A (crappy) shortcut to defining an extension trait for exactly one type.
*/
macro_rules! ext_impl {
    (
        $impl_ty:ty as $trait_ident:ident {
            $(fn $fn_name:ident [$($gen_args:tt)*] $fn_args:tt $(-> $fn_return:ty)* {$($fn_block:tt)*})*
        }
    ) => {
        as_item! {
            pub trait $trait_ident {
                $(fn $fn_name <$($gen_args)*> $fn_args $(-> $fn_return)*;)*
            }

            impl $trait_ident for $impl_ty {
                $(fn $fn_name <$($gen_args)*> $fn_args $(-> $fn_return)* {$($fn_block)*})*
            }
        }
    };
}

/**
Shorthand for implementing `Display`.
*/
macro_rules! impl_Display {
    (for $t:ty, ($self_:ident, $fmt:ident) $body:block) => {
        impl ::std::fmt::Display for $t {
            #[allow(unused_imports)]
            fn fmt(&self, $fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
                use ::std::fmt::Display;
                let $self_ = self;
                $body
            }
        }
    };

    (<[$($gen_defs:tt)*]> for $t:ty, ($self_:ident, $fmt:ident) $body:block) => {
        as_item! {
            impl<$($gen_defs)*> ::std::fmt::Display for $t {
                #[allow(unused_imports)]
                fn fmt(&self, $fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
                    use ::std::fmt::Display;
                    let $self_ = self;
                    $body
                }
            }
        }
    };
}
