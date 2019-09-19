extern crate protoc_grpcio;

fn main() {
    let proto = "src/protos";

    protoc_grpcio::compile_grpc_protos(&["message.proto"], &[proto], &proto, None).expect("failed.");
}
