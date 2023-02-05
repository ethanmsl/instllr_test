//! main.rs
//!
#![allow(clippy::uninlined_format_args)]

use instllr_tst::RunnerInfo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host_info = RunnerInfo::new();
    println!("host_info: {}", host_info);

    // instllr_tst::saved_junk::old_main()?;
    Ok(())
}
