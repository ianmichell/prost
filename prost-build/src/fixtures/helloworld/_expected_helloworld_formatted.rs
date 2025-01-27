#[derive(derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(string, tag = "1")]
    pub say: ::prost::alloc::string::String,
}
#[derive(derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(string, tag = "1")]
    pub say: ::prost::alloc::string::String,
}
#[some_enum_attr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServingStatus {
    Unknown = 0,
    Serving = 1,
    NotServing = 2,
}
impl ServingStatus {
    /// String value of the enum field names used in the `ProtoBuf` definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the `ProtoBuf` definition does not change) and safe for programmatic use.
    #[must_use]
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ServingStatus::Unknown => "UNKNOWN",
            ServingStatus::Serving => "SERVING",
            ServingStatus::NotServing => "NOT_SERVING",
        }
    }
    /// Creates an enum from field names used in the `ProtoBuf` definition.
    #[must_use]
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "SERVING" => Some(Self::Serving),
            "NOT_SERVING" => Some(Self::NotServing),
            _ => None,
        }
    }
}
