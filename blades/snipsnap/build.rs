fn main() {
    pk_build::exe();
    // rustc links with /DEBUG even when stripped, which leaves a debug directory (the PDB's name
    // and linker records) in .rdata: a 512-byte section step here.
    println!("cargo:rustc-link-arg-bins=/DEBUG:NONE");
}
