extern crate prost_build;

fn main() {
    prost_build::compile_protos(
        &["src/proto/net.proto", "src/proto/onnx.proto"],
        &["src/proto/"],
    )
    .unwrap();
}
