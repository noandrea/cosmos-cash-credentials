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
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DidDocument {
    /// @context is spec for did document.
    #[prost(string, repeated, tag="1")]
    pub context: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// id represents the id for the did document.
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    /// A DID controller is an entity that is authorized to make changes to a DID document.
    /// cfr. https://www.w3.org/TR/did-core/#did-controller
    #[prost(string, repeated, tag="3")]
    pub controller: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A DID document can express verification methods, 
    /// such as cryptographic public keys, which can be used 
    /// to authenticate or authorize interactions with the DID subject or associated parties.
    /// https://www.w3.org/TR/did-core/#verification-methods
    #[prost(message, repeated, tag="4")]
    pub verification_methods: ::prost::alloc::vec::Vec<VerificationMethod>,
    /// Services are used in DID documents to express ways of communicating 
    /// with the DID subject or associated entities.
    /// https://www.w3.org/TR/did-core/#services
    #[prost(message, repeated, tag="5")]
    pub services: ::prost::alloc::vec::Vec<Service>,
    /// A verification relationship expresses the relationship between the DID subject and a verification method.
    /// This enum is used to 
    /// cfr. https://www.w3.org/TR/did-core/#verification-relationships
    #[prost(map="string, message", tag="6")]
    pub verification_relationships: ::std::collections::HashMap<::prost::alloc::string::String, did_document::VerificationRelationships>,
}
/// Nested message and enum types in `DidDocument`.
pub mod did_document {
    /// VerificationRelationships - support structure for proto3 repeated string in map
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VerificationRelationships {
        #[prost(string, repeated, tag="1")]
        pub labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// A DID document can express verification methods, 
/// such as cryptographic public keys, which can be used 
/// to authenticate or authorize interactions 
/// with the DID subject or associated parties. 
/// https://www.w3.org/TR/did-core/#verification-methods
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerificationMethod {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub controller: ::prost::alloc::string::String,
    /// by default we support only blockchainAccountID
    #[prost(string, tag="4")]
    pub blockchain_account_id: ::prost::alloc::string::String,
}
/// Service defines how to find data associated with a identifer
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub service_endpoint: ::prost::alloc::string::String,
}
/// QueryIdentifiersRequest is request type for Query/Identifiers RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIdentifiersRequest {
    /// status enables to query for validators matching a given status.
    #[prost(string, tag="1")]
    pub status: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryIdentifiersResponse is response type for the Query/Identifiers RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIdentifiersResponse {
    /// validators contains all the queried validators.
    #[prost(message, repeated, tag="1")]
    pub did_documents: ::prost::alloc::vec::Vec<DidDocument>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryIdentifiersRequest is request type for Query/Identifiers RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIdentifierRequest {
    /// status enables to query for validators matching a given status.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
/// QueryIdentifiersResponse is response type for the Query/Identifiers RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIdentifierResponse {
    /// validators contains all the queried validators.
    #[prost(message, optional, tag="1")]
    pub did_document: ::core::option::Option<DidDocument>,
}
# [doc = r" Generated client implementations."] pub mod query_client { # ! [allow (unused_variables , dead_code , missing_docs)] use tonic :: codegen :: * ; # [doc = " Query defines the gRPC querier service."] pub struct QueryClient < T > { inner : tonic :: client :: Grpc < T > , } impl < T > QueryClient < T > where T : tonic :: client :: GrpcService < tonic :: body :: BoxBody > , T :: ResponseBody : Body + HttpBody + Send + 'static , T :: Error : Into < StdError > , < T :: ResponseBody as HttpBody > :: Error : Into < StdError > + Send , { pub fn new (inner : T) -> Self { let inner = tonic :: client :: Grpc :: new (inner) ; Self { inner } } pub fn with_interceptor (inner : T , interceptor : impl Into < tonic :: Interceptor >) -> Self { let inner = tonic :: client :: Grpc :: with_interceptor (inner , interceptor) ; Self { inner } } # [doc = " Identifiers queries all validators that match the given status."] pub async fn identifiers (& mut self , request : impl tonic :: IntoRequest < super :: QueryIdentifiersRequest > ,) -> Result < tonic :: Response < super :: QueryIdentifiersResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/allinbits.cosmoscash.identifier.Query/Identifiers") ; self . inner . unary (request . into_request () , path , codec) . await } pub async fn identifier (& mut self , request : impl tonic :: IntoRequest < super :: QueryIdentifierRequest > ,) -> Result < tonic :: Response < super :: QueryIdentifierResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/allinbits.cosmoscash.identifier.Query/Identifier") ; self . inner . unary (request . into_request () , path , codec) . await } } impl < T : Clone > Clone for QueryClient < T > { fn clone (& self) -> Self { Self { inner : self . inner . clone () , } } } impl < T > std :: fmt :: Debug for QueryClient < T > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "QueryClient {{ ... }}") } } }/// Verification is a message that allows to assign a verification method
/// to one or more verification relationships
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Verification {
    /// verificationRelationships defines which relationships
    /// are allowed to use the verification method
    ///
    /// relationships that the method is allowed into.
    #[prost(string, repeated, tag="1")]
    pub relationships: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// public key associated with the did document.
    #[prost(message, optional, tag="2")]
    pub method: ::core::option::Option<VerificationMethod>,
    /// additional contexts (json ld schemas)
    #[prost(string, repeated, tag="3")]
    pub context: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
// -------------------------------
// DID
// -------------------------------

/// MsgCreateIdentifier defines a SDK message for creating a new identifier.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateIdentifier {
    /// the did 
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// the controller did
    #[prost(string, tag="2")]
    pub controller: ::prost::alloc::string::String,
    /// the list of verification methods and relationships
    #[prost(message, repeated, tag="3")]
    pub verifications: ::prost::alloc::vec::Vec<Verification>,
    /// the list of services
    #[prost(message, repeated, tag="4")]
    pub services: ::prost::alloc::vec::Vec<Service>,
    /// address of the account signing the message 
    #[prost(string, tag="5")]
    pub signer: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateIdentifierResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateIdentifier {
    /// the did
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// update controllers
    #[prost(string, repeated, tag="2")]
    pub controller: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// address of the account signing the message
    #[prost(string, tag="5")]
    pub signer: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateIdentifierResponse {
}
// -------------------------------
// Verification methods / relations
// -------------------------------

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddVerification {
    /// the did
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// the verification to add
    #[prost(message, optional, tag="2")]
    pub verification: ::core::option::Option<Verification>,
    /// address of the account signing the message
    #[prost(string, tag="3")]
    pub signer: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddVerificationResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetVerificationRelationships {
    /// the did
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// the verification method id
    #[prost(string, tag="2")]
    pub method_id: ::prost::alloc::string::String,
    /// the list of relationships to set
    #[prost(string, repeated, tag="3")]
    pub relationships: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// address of the account signing the message
    #[prost(string, tag="4")]
    pub signer: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetVerificationRelationshipsResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRevokeVerification {
    /// the did
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// the verification method id
    #[prost(string, tag="2")]
    pub method_id: ::prost::alloc::string::String,
    /// address of the account signing the message
    #[prost(string, tag="3")]
    pub signer: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRevokeVerificationResponse {
}
// -------------------------------
// Services
// -------------------------------

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddService {
    /// the did
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// the service data to add
    #[prost(message, optional, tag="2")]
    pub service_data: ::core::option::Option<Service>,
    /// address of the account signing the message 
    #[prost(string, tag="3")]
    pub signer: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddServiceResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteService {
    /// the did
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// the service id
    #[prost(string, tag="2")]
    pub service_id: ::prost::alloc::string::String,
    /// address of the account signing the message 
    #[prost(string, tag="3")]
    pub signer: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteServiceResponse {
}
# [doc = r" Generated client implementations."] pub mod msg_client { # ! [allow (unused_variables , dead_code , missing_docs)] use tonic :: codegen :: * ; # [doc = " Msg defines the identity Msg service."] pub struct MsgClient < T > { inner : tonic :: client :: Grpc < T > , } impl < T > MsgClient < T > where T : tonic :: client :: GrpcService < tonic :: body :: BoxBody > , T :: ResponseBody : Body + HttpBody + Send + 'static , T :: Error : Into < StdError > , < T :: ResponseBody as HttpBody > :: Error : Into < StdError > + Send , { pub fn new (inner : T) -> Self { let inner = tonic :: client :: Grpc :: new (inner) ; Self { inner } } pub fn with_interceptor (inner : T , interceptor : impl Into < tonic :: Interceptor >) -> Self { let inner = tonic :: client :: Grpc :: with_interceptor (inner , interceptor) ; Self { inner } } # [doc = " CreateIdentifier defines a method for creating a new identity."] pub async fn create_identifier (& mut self , request : impl tonic :: IntoRequest < super :: MsgCreateIdentifier > ,) -> Result < tonic :: Response < super :: MsgCreateIdentifierResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/allinbits.cosmoscash.identifier.Msg/CreateIdentifier") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " UpdateIdentifier defines a method for updating an identity."] pub async fn update_identifier (& mut self , request : impl tonic :: IntoRequest < super :: MsgUpdateIdentifier > ,) -> Result < tonic :: Response < super :: MsgUpdateIdentifierResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/allinbits.cosmoscash.identifier.Msg/UpdateIdentifier") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " AddVerificationMethod adds a new verification method "] pub async fn add_verification (& mut self , request : impl tonic :: IntoRequest < super :: MsgAddVerification > ,) -> Result < tonic :: Response < super :: MsgAddVerificationResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/allinbits.cosmoscash.identifier.Msg/AddVerification") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " RevokeVerification remove the verification method and all associated verification Relations"] pub async fn revoke_verification (& mut self , request : impl tonic :: IntoRequest < super :: MsgRevokeVerification > ,) -> Result < tonic :: Response < super :: MsgRevokeVerificationResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/allinbits.cosmoscash.identifier.Msg/RevokeVerification") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " SetVerificationRelationships overwrite current verification relationships"] pub async fn set_verification_relationships (& mut self , request : impl tonic :: IntoRequest < super :: MsgSetVerificationRelationships > ,) -> Result < tonic :: Response < super :: MsgSetVerificationRelationshipsResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/allinbits.cosmoscash.identifier.Msg/SetVerificationRelationships") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " AddService add a new service"] pub async fn add_service (& mut self , request : impl tonic :: IntoRequest < super :: MsgAddService > ,) -> Result < tonic :: Response < super :: MsgAddServiceResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/allinbits.cosmoscash.identifier.Msg/AddService") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " DeleteService delete an existing service"] pub async fn delete_service (& mut self , request : impl tonic :: IntoRequest < super :: MsgDeleteService > ,) -> Result < tonic :: Response < super :: MsgDeleteServiceResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/allinbits.cosmoscash.identifier.Msg/DeleteService") ; self . inner . unary (request . into_request () , path , codec) . await } } impl < T : Clone > Clone for MsgClient < T > { fn clone (& self) -> Self { Self { inner : self . inner . clone () , } } } impl < T > std :: fmt :: Debug for MsgClient < T > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "MsgClient {{ ... }}") } } }