fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = std::process::Command::new("rustc").arg("-V").output()?;
    // set nightly cfg if current toolchain is nightly
    if std::str::from_utf8(&output.stdout)?.contains("nightly") {
        println!("cargo:rustc-cfg=nightly");
    }
    Ok(())
}
