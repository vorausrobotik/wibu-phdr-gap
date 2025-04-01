use elf::ElfBytes;
use elf::endian::AnyEndian;
use elf::section::SectionHeader;
use std::process::Command;

#[test]
fn sample_project_runs_in_debug() {
    let output = Command::new("cargo")
        .args(["run", "--example", "sample"])
        .output()
        .expect("Failed to execute sample project");

    assert!(output.status.success(), "Sample project failed to run");
}

#[test]
fn sample_project_runs_in_release() {
    let output = Command::new("cargo")
        .args(["run", "--release", "--example", "sample"])
        .output()
        .expect("Failed to execute sample project");

    assert!(output.status.success(), "Sample project failed to run");
}

#[test]
fn sample_project_contains_wibu_phdr_gap_in_debug() {
    let output = Command::new("cargo")
        .args(["build", "--example", "sample"])
        .output()
        .expect("Failed to build sample project");

    assert!(output.status.success(), "Sample project failed to build");

    let path = std::path::PathBuf::from("target/debug/examples/sample");
    let file_data = std::fs::read(path).expect("Could not read sample file.");
    let slice = file_data.as_slice();
    let file = ElfBytes::<AnyEndian>::minimal_parse(slice).expect("Could not parse debug sample.");

    let text_hdr: SectionHeader = file
        .section_header_by_name(".text")
        .expect("section table should be parseable")
        .expect("file should have a .text section");

    let text_offset = text_hdr.sh_offset as usize;
    let text_size = text_hdr.sh_size as usize;

    assert!(text_offset + text_size <= file_data.len());

    let text_section = &file_data[text_offset..text_offset + text_size];
    assert!(text_section.windows(8).any(|w| w == b"WIBUPHDR"));
}

#[test]
fn sample_project_contains_wibu_phdr_gap_in_release() {
    let output = Command::new("cargo")
        .args(["build", "--release", "--example", "sample"])
        .output()
        .expect("Failed to build sample project");

    assert!(output.status.success(), "Sample project failed to build");

    let path = std::path::PathBuf::from("target/debug/examples/sample");
    let file_data = std::fs::read(path).expect("Could not read sample file.");
    let slice = file_data.as_slice();
    let file =
        ElfBytes::<AnyEndian>::minimal_parse(slice).expect("Could not parse release sample.");

    let text_hdr: SectionHeader = file
        .section_header_by_name(".text")
        .expect("section table should be parseable")
        .expect("file should have a .text section");

    let text_offset = text_hdr.sh_offset as usize;
    let text_size = text_hdr.sh_size as usize;

    assert!(text_offset + text_size <= file_data.len());

    let text_section = &file_data[text_offset..text_offset + text_size];
    assert!(text_section.windows(8).any(|w| w == b"WIBUPHDR"));
}
