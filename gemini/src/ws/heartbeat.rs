use serde::Deserialize;

mod tag {
    crate::string_field_impl!(Heartbeat, "heartbeat");
}

#[derive(Deserialize, Clone, Debug)]
pub struct Heartbeat<D>
where
    D: Clone + std::fmt::Debug,
{
    #[serde(rename = "type")]
    ty: tag::Heartbeat,
    #[serde(flatten)]
    pub data: D,
}
