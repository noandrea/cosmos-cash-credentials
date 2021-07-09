fn main() {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .type_attribute(
            "allinbits.cosmoscash.identifier.DidDocument",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "allinbits.cosmoscash.identifier.Service",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "allinbits.cosmoscash.identifier.DidDocument.VerificationRelationships",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "allinbits.cosmoscash.identifier.VerificationMethod",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .out_dir("src/pb")
        .compile(
            &[
                "proto/identifier/identifier.proto",
                "proto/identifier/query.proto",
                "proto/identifier/tx.proto",
            ],
            &["proto"],
        )
        .unwrap();
}
