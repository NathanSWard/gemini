pub trait StrLike: AsRef<str> + std::str::FromStr {}

pub trait Price: StrLike {}

pub trait Quantity: StrLike {}

macro_rules! impl_currency {
    ($curr:ident, $repr:literal, $($tr:ident),*) => {
        #[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy)]
        pub struct $curr;

        $(impl $tr for $curr {})*

        impl StrLike for $curr {}

        impl AsRef<str> for $curr {
            fn as_ref(&self) -> &'static str {
                $repr
            }
        }

        impl std::str::FromStr for $curr {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if s.chars().map(|c| c.to_ascii_lowercase()).eq($repr.chars()) {
                    Ok(Self)
                } else {
                    Err(())
                }
            }
        }
    };
}

impl_currency!(USDollar, "usd", Quantity);
impl_currency!(Bitcoin, "btc", Quantity, Price);
impl_currency!(Ethereum, "eth", Quantity, Price);
