use wibu_phdr_gap::ensure_header_space;

fn main() {
    // This function needs to be called in order for the optimizer to not touch it.
    // It is not necessary to call it as first function, but only to do so at all.
    ensure_header_space();

    println!("Hello voraus!");
}
