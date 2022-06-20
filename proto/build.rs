fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(
        &["services/agent_orchestration.proto"],
        &["."],
    )?;
    Ok(())
}
