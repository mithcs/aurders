use std::fs;
use std::process::Command;

pub fn srcinfo() {
    let output = Command::new("makepkg")
        .arg("--printsrcinfo")
        .output()
        .expect("Failed to execute `makepkg --printsrcinfo`");

    if !output.status.success() {
        eprintln!(
            "makepkg --printsrcinfo failed with status: {}",
            output.status
        );
        panic!("Error message: {}", String::from_utf8_lossy(&output.stderr));
    }

    let output_str = String::from_utf8(output.stdout)
        .expect("Failed to convert `makepkg --printsrcinfo` output to UTF-8");

    if let Err(e) = fs::write(".SRCINFO", output_str) {
        panic!("Failed to save .SRCINFO: {e}");
    } else {
        println!("Saved .SRCINFO successfully");
    }
}
