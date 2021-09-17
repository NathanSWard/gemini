use crate::time::Duration;

pub trait Period: Default + Clone + Copy + std::fmt::Debug {
    fn to_duration() -> Duration;
}

macro_rules! duration_impl {
    ($t:ident, $dur:expr) => {
        #[derive(Default, Debug, Clone, Copy)]
        pub struct $t;

        impl Period for $t {
            fn to_duration() -> Duration {
                $dur
            }
        }

        impl From<$t> for Duration {
            fn from(_: $t) -> Duration {
                <$t>::to_duration()
            }
        }
    };
}

duration_impl!(Second, Duration::seconds(1));
duration_impl!(Minute, Duration::minutes(1));
duration_impl!(Hour, Duration::hours(1));
duration_impl!(Day, Duration::days(1));
