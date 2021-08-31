#[macro_export]
macro_rules! any_some {
    ($($o:expr),+) => {
        $($o.is_some())||+
    };
}

#[macro_export]
macro_rules! string_field_impl {
    ($t:ident, $str:literal) => {
        #[derive(serde::Deserialize, serde::Serialize, std::fmt::Debug, Copy, Clone, Default)]
        #[serde(try_from = "&str", into = "&str")]
        pub struct $t;

        impl<'a> std::convert::TryFrom<&'a str> for $t {
            // TODO: make this a legit type
            type Error = &'static str;

            fn try_from(s: &'a str) -> Result<Self, Self::Error> {
                if s == $str {
                    Ok(Self)
                } else {
                    Err(concat!($str, "_error"))
                }
            }
        }

        impl std::convert::From<$t> for &'static str {
            fn from(_: $t) -> &'static str {
                $str
            }
        }
    };
}
