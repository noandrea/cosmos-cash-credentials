fn main() {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
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
