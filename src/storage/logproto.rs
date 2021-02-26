#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushRequest {
    #[prost(message, repeated, tag = "1")]
    pub streams: ::prost::alloc::vec::Vec<StreamAdapter>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequest {
    #[prost(string, tag = "1")]
    pub selector: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub limit: u32,
    #[prost(message, optional, tag = "3")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "4")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(enumeration = "Direction", tag = "5")]
    pub direction: i32,
    #[prost(string, repeated, tag = "7")]
    pub shards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SampleQueryRequest {
    #[prost(string, tag = "1")]
    pub selector: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "3")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, repeated, tag = "4")]
    pub shards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SampleQueryResponse {
    #[prost(message, repeated, tag = "1")]
    pub series: ::prost::alloc::vec::Vec<Series>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResponse {
    #[prost(message, repeated, tag = "1")]
    pub streams: ::prost::alloc::vec::Vec<StreamAdapter>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// True to fetch label values, false for fetch labels names.
    #[prost(bool, tag = "2")]
    pub values: bool,
    #[prost(message, optional, tag = "3")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "4")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelResponse {
    #[prost(string, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamAdapter {
    #[prost(string, tag = "1")]
    pub labels: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub entries: ::prost::alloc::vec::Vec<EntryAdapter>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntryAdapter {
    #[prost(message, optional, tag = "1")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "2")]
    pub line: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sample {
    #[prost(int64, tag = "1")]
    pub timestamp: i64,
    #[prost(double, tag = "2")]
    pub value: f64,
    #[prost(uint64, tag = "3")]
    pub hash: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Series {
    #[prost(string, tag = "1")]
    pub labels: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub samples: ::prost::alloc::vec::Vec<Sample>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TailRequest {
    #[prost(string, tag = "1")]
    pub query: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub delay_for: u32,
    #[prost(uint32, tag = "4")]
    pub limit: u32,
    #[prost(message, optional, tag = "5")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TailResponse {
    #[prost(message, optional, tag = "1")]
    pub stream: ::core::option::Option<StreamAdapter>,
    #[prost(message, repeated, tag = "2")]
    pub dropped_streams: ::prost::alloc::vec::Vec<DroppedStream>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeriesRequest {
    #[prost(message, optional, tag = "1")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "2")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, repeated, tag = "3")]
    pub groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeriesResponse {
    #[prost(message, repeated, tag = "1")]
    pub series: ::prost::alloc::vec::Vec<SeriesIdentifier>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeriesIdentifier {
    #[prost(map = "string, string", tag = "1")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DroppedStream {
    #[prost(message, optional, tag = "1")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "2")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "3")]
    pub labels: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSeriesChunk {
    #[prost(string, tag = "1")]
    pub from_ingester_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub labels: ::prost::alloc::vec::Vec<LabelPair>,
    #[prost(message, repeated, tag = "4")]
    pub chunks: ::prost::alloc::vec::Vec<Chunk>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelPair {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chunk {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferChunksResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TailersCountRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TailersCountResponse {
    #[prost(uint32, tag = "1")]
    pub count: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChunkIDsRequest {
    #[prost(string, tag = "1")]
    pub matchers: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "3")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChunkIDsResponse {
    #[prost(string, repeated, tag = "1")]
    pub chunk_i_ds: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Direction {
    Forward = 0,
    Backward = 1,
}
#[doc = r" Generated client implementations."]
pub mod pusher_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct PusherClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PusherClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PusherClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn push(
            &mut self,
            request: impl tonic::IntoRequest<super::PushRequest>,
        ) -> Result<tonic::Response<super::PushResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/logproto.Pusher/Push");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for PusherClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for PusherClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PusherClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod querier_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct QuerierClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QuerierClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QuerierClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn query(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::QueryResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/logproto.Querier/Query");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn query_sample(
            &mut self,
            request: impl tonic::IntoRequest<super::SampleQueryRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::SampleQueryResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/logproto.Querier/QuerySample");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn label(
            &mut self,
            request: impl tonic::IntoRequest<super::LabelRequest>,
        ) -> Result<tonic::Response<super::LabelResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/logproto.Querier/Label");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn tail(
            &mut self,
            request: impl tonic::IntoRequest<super::TailRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::TailResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/logproto.Querier/Tail");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn series(
            &mut self,
            request: impl tonic::IntoRequest<super::SeriesRequest>,
        ) -> Result<tonic::Response<super::SeriesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/logproto.Querier/Series");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn tailers_count(
            &mut self,
            request: impl tonic::IntoRequest<super::TailersCountRequest>,
        ) -> Result<tonic::Response<super::TailersCountResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/logproto.Querier/TailersCount");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_chunk_i_ds(
            &mut self,
            request: impl tonic::IntoRequest<super::GetChunkIDsRequest>,
        ) -> Result<tonic::Response<super::GetChunkIDsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/logproto.Querier/GetChunkIDs");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for QuerierClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for QuerierClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "QuerierClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod ingester_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct IngesterClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IngesterClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> IngesterClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn transfer_chunks(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::TimeSeriesChunk>,
        ) -> Result<tonic::Response<super::TransferChunksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/logproto.Ingester/TransferChunks");
            self.inner
                .client_streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for IngesterClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for IngesterClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "IngesterClient {{ ... }}")
        }
    }
}
