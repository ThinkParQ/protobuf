#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// The BeeGFS packet format defines these as a uint16 but protobuf does not support uint16 types (<https://protobuf.dev/programming-guides/proto3/#scalar>).
    #[prost(uint32, tag = "1")]
    pub format_version_major: u32,
    #[prost(uint32, tag = "2")]
    pub format_version_minor: u32,
    /// TODO (<https://github.com/ThinkParQ/bee-watch/issues/15>):
    /// This is not implemented yet in the meta service, however for now we'll have BeeWatch generate sequence IDs.
    /// Update this as needed based on final meta implementation and remove current approach in socket.go.
    #[prost(uint64, tag = "3")]
    pub seq_id: u64,
    #[prost(uint32, tag = "4")]
    pub size: u32,
    #[prost(uint64, tag = "5")]
    pub dropped_seq: u64,
    #[prost(uint64, tag = "6")]
    pub missed_seq: u64,
    #[prost(enumeration = "event::Type", tag = "7")]
    pub r#type: i32,
    #[prost(string, tag = "8")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub entry_id: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub parent_entry_id: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub target_path: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub target_parent_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Event`.
pub mod event {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        Invalid = 0,
        Flush = 1,
        Truncate = 2,
        Setattr = 3,
        CloseWrite = 4,
        Create = 5,
        Mkdir = 6,
        Mknod = 7,
        Symlink = 8,
        Rmdir = 9,
        Unlink = 10,
        Hardlink = 11,
        Rename = 12,
        OpenRead = 13,
        OpenWrite = 14,
        OpenReadWrite = 15,
        LastWriterClosed = 16,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Invalid => "INVALID",
                Type::Flush => "FLUSH",
                Type::Truncate => "TRUNCATE",
                Type::Setattr => "SETATTR",
                Type::CloseWrite => "CLOSE_WRITE",
                Type::Create => "CREATE",
                Type::Mkdir => "MKDIR",
                Type::Mknod => "MKNOD",
                Type::Symlink => "SYMLINK",
                Type::Rmdir => "RMDIR",
                Type::Unlink => "UNLINK",
                Type::Hardlink => "HARDLINK",
                Type::Rename => "RENAME",
                Type::OpenRead => "OPEN_READ",
                Type::OpenWrite => "OPEN_WRITE",
                Type::OpenReadWrite => "OPEN_READ_WRITE",
                Type::LastWriterClosed => "LAST_WRITER_CLOSED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INVALID" => Some(Self::Invalid),
                "FLUSH" => Some(Self::Flush),
                "TRUNCATE" => Some(Self::Truncate),
                "SETATTR" => Some(Self::Setattr),
                "CLOSE_WRITE" => Some(Self::CloseWrite),
                "CREATE" => Some(Self::Create),
                "MKDIR" => Some(Self::Mkdir),
                "MKNOD" => Some(Self::Mknod),
                "SYMLINK" => Some(Self::Symlink),
                "RMDIR" => Some(Self::Rmdir),
                "UNLINK" => Some(Self::Unlink),
                "HARDLINK" => Some(Self::Hardlink),
                "RENAME" => Some(Self::Rename),
                "OPEN_READ" => Some(Self::OpenRead),
                "OPEN_WRITE" => Some(Self::OpenWrite),
                "OPEN_READ_WRITE" => Some(Self::OpenReadWrite),
                "LAST_WRITER_CLOSED" => Some(Self::LastWriterClosed),
                _ => None,
            }
        }
    }
}
/// Response messages allow the subscribers to acknowledge events they have processed and request a graceful shutdown.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(uint64, tag = "1")]
    pub completed_seq: u64,
    #[prost(bool, tag = "2")]
    pub shutting_down: bool,
}
/// Generated client implementations.
pub mod subscriber_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SubscriberClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SubscriberClient<tonic::transport::Channel> {
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
    impl<T> SubscriberClient<T>
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
        ) -> SubscriberClient<InterceptedService<T, F>>
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
            SubscriberClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn receive_events(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::Event>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::Response>>,
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
                "/beewatch.Subscriber/ReceiveEvents",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("beewatch.Subscriber", "ReceiveEvents"));
            self.inner.streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod subscriber_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SubscriberServer.
    #[async_trait]
    pub trait Subscriber: Send + Sync + 'static {
        /// Server streaming response type for the ReceiveEvents method.
        type ReceiveEventsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::Response, tonic::Status>,
            >
            + Send
            + 'static;
        async fn receive_events(
            &self,
            request: tonic::Request<tonic::Streaming<super::Event>>,
        ) -> std::result::Result<
            tonic::Response<Self::ReceiveEventsStream>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct SubscriberServer<T: Subscriber> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Subscriber> SubscriberServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SubscriberServer<T>
    where
        T: Subscriber,
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
                "/beewatch.Subscriber/ReceiveEvents" => {
                    #[allow(non_camel_case_types)]
                    struct ReceiveEventsSvc<T: Subscriber>(pub Arc<T>);
                    impl<T: Subscriber> tonic::server::StreamingService<super::Event>
                    for ReceiveEventsSvc<T> {
                        type Response = super::Response;
                        type ResponseStream = T::ReceiveEventsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::Event>>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Subscriber>::receive_events(&inner, request).await
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
                        let method = ReceiveEventsSvc(inner);
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
                        let res = grpc.streaming(method, req).await;
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
    impl<T: Subscriber> Clone for SubscriberServer<T> {
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
    impl<T: Subscriber> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Subscriber> tonic::server::NamedService for SubscriberServer<T> {
        const NAME: &'static str = "beewatch.Subscriber";
    }
}
