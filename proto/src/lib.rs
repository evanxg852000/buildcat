mod protobuf;

pub use protobuf::build::bazel::{remote, semver};
pub const SERVICE_DESCRIPTOR: &[u8] = include_bytes!("protobuf/services_descriptor.bin");
