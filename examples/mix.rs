fn main() {
    use gen_passphrase::dictionary::{EFF_LARGE, EFF_SHORT_1, EFF_SHORT_2};
    use gen_passphrase::generate;

    let passphrase = generate(&[EFF_SHORT_2, EFF_LARGE, EFF_SHORT_1], 2, Some("_"));
    println!("{passphrase}");
}
