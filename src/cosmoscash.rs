pub mod allinbits {
    pub mod cosmoscash {

        pub mod identifier {
            include!("pb/allinbits.cosmoscash.identifier.rs");
        }
    }
}

pub mod gogoproto {
    include!("pb/gogoproto.rs");
}

pub mod google {
    pub mod api {
        include!("pb/google.api.rs");
    }

    pub mod protobuf {
        include!("pb/google.protobuf.rs");
    }
}

pub mod cosmos {
    pub mod base {
        pub mod query {
            pub mod v1beta1 {
                include!("pb/cosmos.base.query.v1beta1.rs");
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
use allinbits::cosmoscash::identifier::{
    query_client::QueryClient, DidDocument, QueryIdentifierRequest,
};

#[cfg(not(target_arch = "wasm32"))]
/// queries a did identifiers
pub async fn query_did(url: &str, did: &str) -> Option<DidDocument> {
    let channel = tonic::transport::Channel::from_shared(url.to_string())
        .unwrap()
        .connect()
        .await
        .unwrap();
    let mut qc = QueryClient::new(channel);

    let qir = tonic::Request::new(QueryIdentifierRequest { id: did.into() });
    let rsp = qc.identifier(qir).await;

    match rsp {
        Ok(rsp) => rsp.get_ref().to_owned().did_document,
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_query_identifier() {
        let url = "https://grpc.cosmos-cash.app.beta.starport.cloud:443";
        let id = "did:cash:cosmos1qxyh99gmtlmjuac9ygzn8kexx4gfwy9dh89wkf";
        let f = super::query_did(url, id);
        let o = tokio_test::block_on(f);
        assert_eq!(o.unwrap().id, id);
    }

    #[test]
    fn test_query_local_identifier() {
        let url = "http://localhost:9090";
        let id = "did:cash:cosmos1uam3kpjdx3wksx46lzq6y628wwyzv0guuren75";
        let f = super::query_did(url, id);
        let o = tokio_test::block_on(f);
        assert_eq!(o.unwrap().id, id);
    }
}
