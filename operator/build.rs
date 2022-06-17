fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(
        &["../proto/services/agent_orchestration.proto"],
        &["../proto"],
    )?;
    Ok(())
}
