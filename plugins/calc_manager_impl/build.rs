use idl_gen::rust::build_server;
use anyhow::Result;

fn main() -> Result<()> {
    build_server(std::path::Path::new("../idl"))?;
    Ok(())
}