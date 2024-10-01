pub mod gen;
pub mod gen_dep {
    pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("./gen/proto_descriptor.bin");
}