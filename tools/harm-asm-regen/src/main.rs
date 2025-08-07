use std::ffi::OsStr;
use std::io::Read;
use std::io::Write;
use std::path::Path;

fn main() -> eyre::Result<()> {
    let mut asm_code = String::new();
    std::io::stdin().read_to_string(&mut asm_code)?;

    let orig_lines = strip_lines(asm_code.lines());

    let mut asm_file = tempfile::Builder::new().suffix(".s").tempfile()?;
    for line in &orig_lines {
        writeln!(asm_file, "{line}")?;
    }
    let obj_file = tempfile::Builder::new().suffix(".o").tempfile()?;

    run_cc(asm_file.path(), obj_file.path()).unwrap();
    let asm = run_disassembler(obj_file.path()).unwrap();
    let asm = clean_disassembler(asm.as_str(), &orig_lines)?;
    eprintln!("===========================================================================");
    println!("{asm}");
    Ok(())
}

fn strip_lines<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<&'a str> {
    lines
        .map(|line| {
            match line.find('\t') {
                // it should include the tab
                Some(pos) => line.split_at(pos).1,
                None => line,
            }
        })
        .collect()
}

// TODO try `llvm-mc -triple=aarch64 -show-encoding`: it emits source line and encoding without a disassembler and join.
fn run_cc(asm_path: &Path, obj_path: &Path) -> eyre::Result<()> {
    let status = std::process::Command::new("cc")
        .args([
            OsStr::new("-o"),
            OsStr::new(obj_path),
            OsStr::new("-c"),
            OsStr::new(asm_path),
        ])
        .stderr(std::process::Stdio::inherit())
        .status()?;
    eyre::ensure!(status.success(), "cc failed: {:?}", status);
    Ok(())
}

fn run_disassembler(obj_path: &Path) -> eyre::Result<String> {
    let output = std::process::Command::new("objdump")
        .args([OsStr::new("--disassemble-all"), OsStr::new(obj_path)])
        .stderr(std::process::Stdio::inherit())
        .output()?;
    eyre::ensure!(output.status.success(), "cc failed: {:?}", output.status);

    Ok(String::from_utf8(output.stdout)?)
}

fn clean_disassembler(asm: &str, asm_orig: &[&str]) -> eyre::Result<String> {
    let lines: Vec<_> = asm
        .lines()
        .filter(|line| line.starts_with("     "))
        .collect();
    eyre::ensure!(
        lines.len() == asm_orig.len(),
        "Produced asm instruction count mismatches the original one"
    );
    Ok(lines
        .into_iter()
        .zip(asm_orig)
        .map(|(line, orig)| {
            let line = line.split_at(10).1;
            let inst_code = line.split_at(8).0;
            format!("{inst_code}{orig}\n")
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_lines() {
        let lines = strip_lines("a\tb\nc\n".lines());
        assert_eq!(lines, ["\tb", "c"]);
    }
}
