use std::path::{Path, PathBuf};

use anyhow::Result;
use aobscan::PatternBuilder;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: {} <HITMAN3_PATH>", args[0]);
        return;
    }

    if let Err(e) = run(&args[1]) {
        println!("error: {}", e);
    }
}

fn get_hitman3_file_path(path: &str) -> PathBuf {
    Path::new(path).join("Retail").join("HITMAN3.exe")
}

fn run(in_path: &str) -> Result<()> {
    let hitman3_path = get_hitman3_file_path(in_path);
    anyhow::ensure!(hitman3_path.exists(), "HITMAN3.exe does not exist");

    let mut data = std::fs::read(&hitman3_path)
        .map_err(|e| anyhow::anyhow!("failed to read HITMAN3.exe: {}", e))?;

    // Store a backup of the file to patch as HITMAN3.exe.bak
    let backup_path = hitman3_path.with_extension("exe.bak");
    std::fs::write(&backup_path, &data)
        .map_err(|e| anyhow::anyhow!("failed to create HITMAN3.exe.bak: {}", e))?;

    // call CDXGIAdapter::RegisterVideoMemoryBudgetChangeNotificationEvent
    // test eax, eax
    // jns short loc_XXXXXXXXX
    let mut reg_vm_budget_change_adr = 0usize;
    if !PatternBuilder::from_ida_style("41 FF 91 ? ? ? ? 85 C0")
        .unwrap()
        .build()
        .scan(&data, |addr| {
            reg_vm_budget_change_adr = addr + 0x9;
            false
        })
    {
        anyhow::bail!("failed to find call RegisterVideoMemoryBudgetChangeNotificationEvent");
    }

    let first_byte = *data.get(reg_vm_budget_change_adr).unwrap();

    // jmp instead of jns, perhaps the patch has already been applied?
    anyhow::ensure!(
        first_byte != 0xEB,
        "it looks like the patch has already been applied"
    );

    // Wrong OPCode, we cannot use this instruction!
    anyhow::ensure!(
        first_byte == 0x79,
        "unexpected instruction found at patch address"
    );

    // Write 0xEB (jmp) to the first byte of the instruction.
    *data.get_mut(reg_vm_budget_change_adr).unwrap() = 0xEB;

    // Store the patched file to disk.
    std::fs::write(&hitman3_path, &data)
        .map_err(|e| anyhow::anyhow!("failed to write HITMAN3.exe: {}", e))?;

    println!("patch applied successfully!");
    Ok(())
}
