//! Account ID handling.

pub use id::Builder;

/// Helper macro for generating `fmt` trait implementations for integer wrapper types.
macro_rules! impl_fmt_traits {
    ($type:ty) => {
        impl ::std::fmt::Display for $type {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::Binary for $type {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Binary::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::LowerExp for $type {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::LowerExp::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::LowerHex for $type {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::LowerHex::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::Octal for $type {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Octal::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::UpperExp for $type {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::UpperExp::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::UpperHex for $type {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::UpperHex::fmt(&self.0, f)
            }
        }
    };
}

pub mod error;
pub mod id;
pub mod index;
pub mod raw;

pub use error::AccountIdError;
pub use id::AccountId;
pub use index::*;
pub use raw::*;
