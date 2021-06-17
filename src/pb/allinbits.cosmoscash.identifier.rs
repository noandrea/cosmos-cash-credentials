//
//This represents a minimal self-managed did document
//definition: https://w3c.github.io/did-core/
//
//{
//"@context": "https://www.w3.org/ns/did/v1",
//"id": "did:example:123456789abcdefghi",
//"authentication": [{
//"id": "did:example:123456789abcdefghi#keys-1",
//"type": "Ed25519VerificationKey2018",
//"controller": "did:example:123456789abcdefghi",
//"publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
//}],
//"service": [{
//"id":"did:example:123456789abcdefghi#vcs",
//"type": "VerifiableCredentialService",
//"serviceEndpoint": "https://example.com/vc/"
//}]
//}

/// DidDocument represents a dencentralised identifer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DidDocument {
    /// @context is spec for did document.
    #[prost(string, tag = "1")]
    pub context: ::prost::alloc::string::String,
    /// id represents the id for the did document.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// authentication represents public key associated with the did document.
    #[prost(message, repeated, tag = "3")]
    pub authentication: ::prost::alloc::vec::Vec<Authentication>,
    /// services represents each service associated with a did
    #[prost(message, repeated, tag = "4")]
    pub services: ::prost::alloc::vec::Vec<Service>,
}
/// Authentication defines how to authenticate a did document.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authentication {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub controller: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub public_key: ::prost::alloc::string::String,
}
/// Service defines how to find data associated with a identifer
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub service_endpoint: ::prost::alloc::string::String,
}
/// QueryIdentifersRequest is request type for Query/Identifers RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIdentifiersRequest {
    /// status enables to query for validators matching a given status.
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryIdentifersResponse is response type for the Query/Identifers RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIdentifiersResponse {
    /// validators contains all the queried validators.
    #[prost(message, repeated, tag = "1")]
    pub did_documents: ::prost::alloc::vec::Vec<DidDocument>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryIdentifersRequest is request type for Query/Identifers RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIdentifierRequest {
    /// status enables to query for validators matching a given status.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// QueryIdentifersResponse is response type for the Query/Identifers RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIdentifierResponse {
    /// validators contains all the queried validators.
    #[prost(message, optional, tag = "1")]
    pub did_document: ::core::option::Option<DidDocument>,
}
#[doc = r" Generated client implementations."]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Query defines the gRPC querier service."]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueryClient<tonic::transport::Channel> {
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
    impl<T> QueryClient<T>
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
        #[doc = " Identifers queries all validators that match the given status."]
        pub async fn identifiers(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryIdentifiersRequest>,
        ) -> Result<tonic::Response<super::QueryIdentifiersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/allinbits.cosmoscash.identifier.Query/Identifiers",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn identifier(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryIdentifierRequest>,
        ) -> Result<tonic::Response<super::QueryIdentifierResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/allinbits.cosmoscash.identifier.Query/Identifier",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for QueryClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for QueryClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "QueryClient {{ ... }}")
        }
    }
}
/// MsgCreateIdentifier defines a SDK message for creating a new identifer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateIdentifier {
    #[prost(string, tag = "1")]
    pub context: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// authentication represents public key associated with the did document.
    #[prost(message, repeated, tag = "3")]
    pub authentication: ::prost::alloc::vec::Vec<Authentication>,
    /// services represents each service associated with a did
    #[prost(message, repeated, tag = "4")]
    pub services: ::prost::alloc::vec::Vec<Service>,
    /// owner represents the user creating the message
    #[prost(string, tag = "5")]
    pub owner: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateIdentifierResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddAuthentication {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// authentication represents public key associated with the did document.
    #[prost(message, optional, tag = "2")]
    pub authentication: ::core::option::Option<Authentication>,
    /// owner is the address of the user creating the message
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddAuthenticationResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddService {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// authentication represents public key associated with the did document.
    #[prost(message, optional, tag = "2")]
    pub service_data: ::core::option::Option<Service>,
    /// owner is the address of the user creating the message
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddServiceResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteAuthentication {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    /// owner is the address of the user creating the message
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteAuthenticationResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteService {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub service_id: ::prost::alloc::string::String,
    /// owner is the address of the user creating the message
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteServiceResponse {}
#[doc = r" Generated client implementations."]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Msg defines the identity Msg service."]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MsgClient<tonic::transport::Channel> {
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
    impl<T> MsgClient<T>
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
        #[doc = " CreateDidDocument defines a method for creating a new identity."]
        pub async fn create_identifier(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateIdentifier>,
        ) -> Result<tonic::Response<super::MsgCreateIdentifierResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/allinbits.cosmoscash.identifier.Msg/CreateIdentifier",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_authentication(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAddAuthentication>,
        ) -> Result<tonic::Response<super::MsgAddAuthenticationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/allinbits.cosmoscash.identifier.Msg/AddAuthentication",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_service(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAddService>,
        ) -> Result<tonic::Response<super::MsgAddServiceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/allinbits.cosmoscash.identifier.Msg/AddService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_authentication(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeleteAuthentication>,
        ) -> Result<tonic::Response<super::MsgDeleteAuthenticationResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/allinbits.cosmoscash.identifier.Msg/DeleteAuthentication",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_service(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeleteService>,
        ) -> Result<tonic::Response<super::MsgDeleteServiceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/allinbits.cosmoscash.identifier.Msg/DeleteService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for MsgClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for MsgClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MsgClient {{ ... }}")
        }
    }
}
