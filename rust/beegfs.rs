/// The "old" BeeGFS numeric Id-NodeType combination that can be used to identify entities like nodes,
/// targets, ... .  Because each entity type has its own Id space (meaning a combination is NOT
/// globally unique), this also requires the entities type it is related to.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LegacyId {
    /// Old style BeeGFS numeric Id
    #[prost(uint32, tag = "1")]
    pub num_id: u32,
    /// BeeGFS node type. Despite the name, also applies to other entity types (which currently can
    /// only be on a node of their own type, so it's still correct).
    #[prost(enumeration = "NodeType", tag = "2")]
    pub node_type: i32,
    /// The referred entities type.
    #[prost(enumeration = "EntityType", tag = "3")]
    pub entity_type: i32,
}
/// Contains all existing identifiers used to uniquely identify an entity like a specific node,
/// target, ... . This is what should usually be returned by a server when referring to an entity,
/// for example when requesting a list of nodes. The requestor/client can then decide which
/// identifier to use depending on the use case.
/// Note that all fields are explicitly optional. The server should try to fill all of them, but sometimes
/// (e.g. for performance reasons), only some of them might be set. It's the requesters job to check
/// that.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityIdSet {
    /// The new style globally unique identifier. Globally unique - identifies an entity from all types
    /// without any additional context.
    #[prost(uint64, tag = "1")]
    pub uid: u64,
    /// The user definable alias of an entity. Globally unique - identifies an entity from all types
    /// without any additional context.
    #[prost(string, tag = "2")]
    pub alias: ::prost::alloc::string::String,
    /// The old style numeric Id-NodeType combination. NOT globally unique - entity type depends on
    /// the context.
    #[prost(message, optional, tag = "3")]
    pub legacy_id: ::core::option::Option<LegacyId>,
}
/// Contains one of the existing identifiers used to uniquely identify an entity like a specific
/// node, target, ... . This is meant for requests that identify one or more entities. Only one
/// unique identifier is needed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityIdVariant {
    #[prost(oneof = "entity_id_variant::Variant", tags = "1, 2, 3")]
    pub variant: ::core::option::Option<entity_id_variant::Variant>,
}
/// Nested message and enum types in `EntityIdVariant`.
pub mod entity_id_variant {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Variant {
        #[prost(uint64, tag = "1")]
        Uid(u64),
        #[prost(message, tag = "2")]
        LegacyId(super::LegacyId),
        #[prost(string, tag = "3")]
        Alias(::prost::alloc::string::String),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EntityType {
    Unspecified = 0,
    Node = 1,
    Target = 2,
    BuddyGroup = 3,
    StoragePool = 4,
}
impl EntityType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EntityType::Unspecified => "ENTITY_TYPE_UNSPECIFIED",
            EntityType::Node => "NODE",
            EntityType::Target => "TARGET",
            EntityType::BuddyGroup => "BUDDY_GROUP",
            EntityType::StoragePool => "STORAGE_POOL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ENTITY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "NODE" => Some(Self::Node),
            "TARGET" => Some(Self::Target),
            "BUDDY_GROUP" => Some(Self::BuddyGroup),
            "STORAGE_POOL" => Some(Self::StoragePool),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NodeType {
    Unspecified = 0,
    Client = 1,
    Meta = 2,
    Storage = 3,
    Management = 4,
}
impl NodeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NodeType::Unspecified => "NODE_TYPE_UNSPECIFIED",
            NodeType::Client => "CLIENT",
            NodeType::Meta => "META",
            NodeType::Storage => "STORAGE",
            NodeType::Management => "MANAGEMENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NODE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "CLIENT" => Some(Self::Client),
            "META" => Some(Self::Meta),
            "STORAGE" => Some(Self::Storage),
            "MANAGEMENT" => Some(Self::Management),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReachabilityState {
    Unspecified = 0,
    Online = 1,
    Poffline = 2,
    Offline = 3,
}
impl ReachabilityState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReachabilityState::Unspecified => "REACHABILITY_STATE_UNSPECIFIED",
            ReachabilityState::Online => "ONLINE",
            ReachabilityState::Poffline => "POFFLINE",
            ReachabilityState::Offline => "OFFLINE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REACHABILITY_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "ONLINE" => Some(Self::Online),
            "POFFLINE" => Some(Self::Poffline),
            "OFFLINE" => Some(Self::Offline),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConsistencyState {
    Unspecified = 0,
    Good = 1,
    NeedsResync = 2,
    Bad = 3,
}
impl ConsistencyState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConsistencyState::Unspecified => "CONSISTENCY_STATE_UNSPECIFIED",
            ConsistencyState::Good => "GOOD",
            ConsistencyState::NeedsResync => "NEEDS_RESYNC",
            ConsistencyState::Bad => "BAD",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONSISTENCY_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "GOOD" => Some(Self::Good),
            "NEEDS_RESYNC" => Some(Self::NeedsResync),
            "BAD" => Some(Self::Bad),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CapacityPool {
    Unspecified = 0,
    Normal = 1,
    Low = 2,
    Emergency = 3,
}
impl CapacityPool {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CapacityPool::Unspecified => "CAPACITY_POOL_UNSPECIFIED",
            CapacityPool::Normal => "NORMAL",
            CapacityPool::Low => "LOW",
            CapacityPool::Emergency => "EMERGENCY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CAPACITY_POOL_UNSPECIFIED" => Some(Self::Unspecified),
            "NORMAL" => Some(Self::Normal),
            "LOW" => Some(Self::Low),
            "EMERGENCY" => Some(Self::Emergency),
            _ => None,
        }
    }
}
/// A Nics NicType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NicType {
    Unspecified = 0,
    Ethernet = 1,
    Rdma = 2,
}
impl NicType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NicType::Unspecified => "NIC_TYPE_UNSPECIFIED",
            NicType::Ethernet => "ETHERNET",
            NicType::Rdma => "RDMA",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NIC_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ETHERNET" => Some(Self::Ethernet),
            "RDMA" => Some(Self::Rdma),
            _ => None,
        }
    }
}
