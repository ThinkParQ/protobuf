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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodesRequest {
    /// Query the nic list for each node and include it in the response
    #[prost(bool, tag = "1")]
    pub include_nics: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodesResponse {
    /// The list of nodes
    #[prost(message, repeated, tag = "1")]
    pub nodes: ::prost::alloc::vec::Vec<get_nodes_response::Node>,
    /// The node containing the root inode. Will be missing on a fresh system without any meta
    /// targets/nodes.
    #[prost(message, optional, tag = "2")]
    pub meta_root_node: ::core::option::Option<EntityIdSet>,
}
/// Nested message and enum types in `GetNodesResponse`.
pub mod get_nodes_response {
    /// BeeGFS node related data
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Node {
        /// The node identifiers
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::EntityIdSet>,
        #[prost(enumeration = "super::NodeType", tag = "2")]
        pub node_type: i32,
        /// The nodes TCP and UDP port.
        #[prost(uint32, tag = "3")]
        pub port: u32,
        /// The nodes Nics
        #[prost(message, repeated, tag = "4")]
        pub nics: ::prost::alloc::vec::Vec<node::Nic>,
    }
    /// Nested message and enum types in `Node`.
    pub mod node {
        /// BeeGFS nic related data
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Nic {
            /// The nics IPv4 address in the form aaa.bbb.ccc.ddd:port
            #[prost(string, tag = "1")]
            pub addr: ::prost::alloc::string::String,
            /// The nics name (note that this is NOT an alias as a Nic is not considered an entity)
            #[prost(string, tag = "2")]
            pub name: ::prost::alloc::string::String,
            /// The nics type
            #[prost(enumeration = "super::super::NicType", tag = "3")]
            pub nic_type: i32,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTargetsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTargetsResponse {
    /// The list of targets
    #[prost(message, repeated, tag = "1")]
    pub targets: ::prost::alloc::vec::Vec<get_targets_response::Target>,
}
/// Nested message and enum types in `GetTargetsResponse`.
pub mod get_targets_response {
    /// A BeeGFS target
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Target {
        /// The targets identifiers
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::EntityIdSet>,
        /// Node type the target is on
        #[prost(enumeration = "super::NodeType", tag = "2")]
        pub node_type: i32,
        /// The targets reachability state as reported by management
        #[prost(enumeration = "super::ReachabilityState", tag = "3")]
        pub reachability_state: i32,
        /// The targets reachability state as reported by management
        #[prost(enumeration = "super::ConsistencyState", tag = "4")]
        pub consistency_state: i32,
        /// Duration since last contact to the target. Currently slightly inaccurate as it updates
        /// not on each received message.
        #[prost(uint64, optional, tag = "5")]
        pub last_contact_s: ::core::option::Option<u64>,
        /// Total space on the target as reported by management
        #[prost(uint64, optional, tag = "6")]
        pub total_space_bytes: ::core::option::Option<u64>,
        /// Free space on the target as reported by management
        #[prost(uint64, optional, tag = "7")]
        pub free_space_bytes: ::core::option::Option<u64>,
        /// Total inodes on the target as reported by management
        #[prost(uint64, optional, tag = "8")]
        pub total_inodes: ::core::option::Option<u64>,
        /// Free inodes on the target as reported by management
        #[prost(uint64, optional, tag = "9")]
        pub free_inodes: ::core::option::Option<u64>,
        /// The targets capacity pool as reported by the management
        #[prost(enumeration = "super::CapacityPool", tag = "10")]
        pub cap_pool: i32,
        /// The targets owner node identifiers
        #[prost(message, optional, tag = "11")]
        pub node: ::core::option::Option<super::EntityIdSet>,
        /// The targets storage pool identifiers. Explicitly optional since meta targets don't have a storage pool.
        #[prost(message, optional, tag = "12")]
        pub storage_pool: ::core::option::Option<super::EntityIdSet>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBuddyGroupsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBuddyGroupsResponse {
    /// The list of buddy groups
    #[prost(message, repeated, tag = "1")]
    pub buddy_groups: ::prost::alloc::vec::Vec<get_buddy_groups_response::BuddyGroup>,
}
/// Nested message and enum types in `GetBuddyGroupsResponse`.
pub mod get_buddy_groups_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BuddyGroup {
        /// The buddy groups identifiers
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::EntityIdSet>,
        /// Node type the buddy group belongs to
        #[prost(enumeration = "super::NodeType", tag = "2")]
        pub node_type: i32,
        /// The buddy groups current primary target identifiers
        #[prost(message, optional, tag = "3")]
        pub primary_target: ::core::option::Option<super::EntityIdSet>,
        /// The buddy groups current secondary target identifiers
        #[prost(message, optional, tag = "4")]
        pub secondary_target: ::core::option::Option<super::EntityIdSet>,
        /// The buddy groups primary target consistency state
        #[prost(enumeration = "super::ConsistencyState", tag = "5")]
        pub primary_consistency_state: i32,
        /// The buddy groups secondary target consistency state
        #[prost(enumeration = "super::ConsistencyState", tag = "6")]
        pub secondary_consistency_state: i32,
        /// The buddy groups storage pool. Explicitly optional since meta pools dont' have a storage
        /// pool.
        #[prost(message, optional, tag = "7")]
        pub storage_pool: ::core::option::Option<super::EntityIdSet>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoragePoolsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoragePoolsResponse {
    /// The list of storage pools
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<get_storage_pools_response::StoragePool>,
}
/// Nested message and enum types in `GetStoragePoolsResponse`.
pub mod get_storage_pools_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StoragePool {
        /// The storage pools identifiers
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::EntityIdSet>,
        /// The storage pools assigned targets identifiers
        #[prost(message, repeated, tag = "2")]
        pub targets: ::prost::alloc::vec::Vec<super::EntityIdSet>,
        /// The storage pools assigned buddy groups identifiers
        #[prost(message, repeated, tag = "3")]
        pub buddy_groups: ::prost::alloc::vec::Vec<super::EntityIdSet>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAliasRequest {
    /// The identifier to set the alias for
    #[prost(message, optional, tag = "1")]
    pub entity_id: ::core::option::Option<EntityIdVariant>,
    /// The new alias
    #[prost(string, tag = "2")]
    pub new_alias: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAliasResponse {}
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
/// A nodes reachability state as calculated by the management
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
/// A targets consistency state as known by the management
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
/// A targets capacity pool as calculated by management
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
/// Generated client implementations.
pub mod management_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ManagementClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ManagementClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ManagementClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ManagementClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ManagementClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Gets the full list of BeeGFS nodes
        pub async fn get_nodes(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNodesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetNodesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/beegfs.Management/GetNodes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("beegfs.Management", "GetNodes"));
            self.inner.unary(req, path, codec).await
        }
        /// Gets the full list of BeeGFS targets
        pub async fn get_targets(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTargetsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTargetsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/beegfs.Management/GetTargets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("beegfs.Management", "GetTargets"));
            self.inner.unary(req, path, codec).await
        }
        /// Gets the full list of BeeGFS buddy groups
        pub async fn get_buddy_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBuddyGroupsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBuddyGroupsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/beegfs.Management/GetBuddyGroups",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("beegfs.Management", "GetBuddyGroups"));
            self.inner.unary(req, path, codec).await
        }
        /// Gets the full list of BeeGFS storage pools
        pub async fn get_storage_pools(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStoragePoolsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetStoragePoolsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/beegfs.Management/GetStoragePools",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("beegfs.Management", "GetStoragePools"));
            self.inner.unary(req, path, codec).await
        }
        /// Sets an entity alias
        pub async fn set_alias(
            &mut self,
            request: impl tonic::IntoRequest<super::SetAliasRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetAliasResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/beegfs.Management/SetAlias",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("beegfs.Management", "SetAlias"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod management_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ManagementServer.
    #[async_trait]
    pub trait Management: Send + Sync + 'static {
        /// Gets the full list of BeeGFS nodes
        async fn get_nodes(
            &self,
            request: tonic::Request<super::GetNodesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetNodesResponse>,
            tonic::Status,
        >;
        /// Gets the full list of BeeGFS targets
        async fn get_targets(
            &self,
            request: tonic::Request<super::GetTargetsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTargetsResponse>,
            tonic::Status,
        >;
        /// Gets the full list of BeeGFS buddy groups
        async fn get_buddy_groups(
            &self,
            request: tonic::Request<super::GetBuddyGroupsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBuddyGroupsResponse>,
            tonic::Status,
        >;
        /// Gets the full list of BeeGFS storage pools
        async fn get_storage_pools(
            &self,
            request: tonic::Request<super::GetStoragePoolsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetStoragePoolsResponse>,
            tonic::Status,
        >;
        /// Sets an entity alias
        async fn set_alias(
            &self,
            request: tonic::Request<super::SetAliasRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetAliasResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ManagementServer<T: Management> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Management> ManagementServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ManagementServer<T>
    where
        T: Management,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/beegfs.Management/GetNodes" => {
                    #[allow(non_camel_case_types)]
                    struct GetNodesSvc<T: Management>(pub Arc<T>);
                    impl<
                        T: Management,
                    > tonic::server::UnaryService<super::GetNodesRequest>
                    for GetNodesSvc<T> {
                        type Response = super::GetNodesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetNodesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Management>::get_nodes(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNodesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/beegfs.Management/GetTargets" => {
                    #[allow(non_camel_case_types)]
                    struct GetTargetsSvc<T: Management>(pub Arc<T>);
                    impl<
                        T: Management,
                    > tonic::server::UnaryService<super::GetTargetsRequest>
                    for GetTargetsSvc<T> {
                        type Response = super::GetTargetsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTargetsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Management>::get_targets(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTargetsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/beegfs.Management/GetBuddyGroups" => {
                    #[allow(non_camel_case_types)]
                    struct GetBuddyGroupsSvc<T: Management>(pub Arc<T>);
                    impl<
                        T: Management,
                    > tonic::server::UnaryService<super::GetBuddyGroupsRequest>
                    for GetBuddyGroupsSvc<T> {
                        type Response = super::GetBuddyGroupsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBuddyGroupsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Management>::get_buddy_groups(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBuddyGroupsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/beegfs.Management/GetStoragePools" => {
                    #[allow(non_camel_case_types)]
                    struct GetStoragePoolsSvc<T: Management>(pub Arc<T>);
                    impl<
                        T: Management,
                    > tonic::server::UnaryService<super::GetStoragePoolsRequest>
                    for GetStoragePoolsSvc<T> {
                        type Response = super::GetStoragePoolsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetStoragePoolsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Management>::get_storage_pools(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetStoragePoolsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/beegfs.Management/SetAlias" => {
                    #[allow(non_camel_case_types)]
                    struct SetAliasSvc<T: Management>(pub Arc<T>);
                    impl<
                        T: Management,
                    > tonic::server::UnaryService<super::SetAliasRequest>
                    for SetAliasSvc<T> {
                        type Response = super::SetAliasResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetAliasRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Management>::set_alias(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetAliasSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Management> Clone for ManagementServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Management> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Management> tonic::server::NamedService for ManagementServer<T> {
        const NAME: &'static str = "beegfs.Management";
    }
}
