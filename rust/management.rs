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
    pub meta_root_node: ::core::option::Option<super::beegfs::EntityIdSet>,
}
/// Nested message and enum types in `GetNodesResponse`.
pub mod get_nodes_response {
    /// BeeGFS node related data
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Node {
        /// The node identifiers
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<super::super::beegfs::EntityIdSet>,
        #[prost(enumeration = "super::super::beegfs::NodeType", tag = "2")]
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
            #[prost(enumeration = "super::super::super::beegfs::NicType", tag = "3")]
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
        pub id: ::core::option::Option<super::super::beegfs::EntityIdSet>,
        /// Node type the target is on
        #[prost(enumeration = "super::super::beegfs::NodeType", tag = "2")]
        pub node_type: i32,
        /// The targets reachability state as reported by management
        #[prost(enumeration = "super::super::beegfs::ReachabilityState", tag = "3")]
        pub reachability_state: i32,
        /// The targets reachability state as reported by management
        #[prost(enumeration = "super::super::beegfs::ConsistencyState", tag = "4")]
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
        #[prost(enumeration = "super::super::beegfs::CapacityPool", tag = "10")]
        pub cap_pool: i32,
        /// The targets owner node identifiers
        #[prost(message, optional, tag = "11")]
        pub node: ::core::option::Option<super::super::beegfs::EntityIdSet>,
        /// The targets storage pool identifiers. Explicitly optional since meta targets don't have a storage pool.
        #[prost(message, optional, tag = "12")]
        pub storage_pool: ::core::option::Option<super::super::beegfs::EntityIdSet>,
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
        pub id: ::core::option::Option<super::super::beegfs::EntityIdSet>,
        /// Node type the buddy group belongs to
        #[prost(enumeration = "super::super::beegfs::NodeType", tag = "2")]
        pub node_type: i32,
        /// The buddy groups current primary target identifiers
        #[prost(message, optional, tag = "3")]
        pub primary_target: ::core::option::Option<super::super::beegfs::EntityIdSet>,
        /// The buddy groups current secondary target identifiers
        #[prost(message, optional, tag = "4")]
        pub secondary_target: ::core::option::Option<super::super::beegfs::EntityIdSet>,
        /// The buddy groups primary target consistency state
        #[prost(enumeration = "super::super::beegfs::ConsistencyState", tag = "5")]
        pub primary_consistency_state: i32,
        /// The buddy groups secondary target consistency state
        #[prost(enumeration = "super::super::beegfs::ConsistencyState", tag = "6")]
        pub secondary_consistency_state: i32,
        /// The buddy groups storage pool. Explicitly optional since meta pools dont' have a storage
        /// pool.
        #[prost(message, optional, tag = "7")]
        pub storage_pool: ::core::option::Option<super::super::beegfs::EntityIdSet>,
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
        pub id: ::core::option::Option<super::super::beegfs::EntityIdSet>,
        /// The storage pools assigned targets identifiers
        #[prost(message, repeated, tag = "2")]
        pub targets: ::prost::alloc::vec::Vec<super::super::beegfs::EntityIdSet>,
        /// The storage pools assigned buddy groups identifiers
        #[prost(message, repeated, tag = "3")]
        pub buddy_groups: ::prost::alloc::vec::Vec<super::super::beegfs::EntityIdSet>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAliasRequest {
    /// The identifier to set the alias for
    #[prost(message, optional, tag = "1")]
    pub entity_id: ::core::option::Option<super::beegfs::EntityIdVariant>,
    /// The new alias
    #[prost(string, tag = "2")]
    pub new_alias: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAliasResponse {}
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
                "/management.Management/GetNodes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("management.Management", "GetNodes"));
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
                "/management.Management/GetTargets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("management.Management", "GetTargets"));
            self.inner.unary(req, path, codec).await
        }
        /// Gets the full list of BeeGFS buddbeegfsy groups
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
                "/management.Management/GetBuddyGroups",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("management.Management", "GetBuddyGroups"));
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
                "/management.Management/GetStoragePools",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("management.Management", "GetStoragePools"));
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
                "/management.Management/SetAlias",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("management.Management", "SetAlias"));
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
        /// Gets the full list of BeeGFS buddbeegfsy groups
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
                "/management.Management/GetNodes" => {
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
                "/management.Management/GetTargets" => {
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
                "/management.Management/GetBuddyGroups" => {
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
                "/management.Management/GetStoragePools" => {
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
                "/management.Management/SetAlias" => {
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
        const NAME: &'static str = "management.Management";
    }
}
