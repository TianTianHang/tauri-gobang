use std::process::Command;
use std::path::Path;

fn main() {
    // Check if Rapfi binary exists
    let rapfi_path = "./binaries/rapfi-x86_64-unknown-linux-gnu";
    
    if Path::new(rapfi_path).exists() {
        println!("✓ Rapfi binary found at: {}", rapfi_path);
        
        // Test if it's executable
        let output = Command::new(rapfi_path)
            .arg("--help")
            .output();
            
        match output {
            Ok(out) => {
                println!("✓ Rapfi is executable");
                println!("Help output:\n{}", String::from_utf8_lossy(&out.stdout));
            }
            Err(e) => {
                println!("✗ Error executing Rapfi: {}", e);
            }
        }
    } else {
        println!("✗ Rapfi binary not found at: {}", rapfi_path);
    }
}
