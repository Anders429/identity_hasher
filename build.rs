extern crate autocfg;

fn main() {
    let ac = autocfg::new();

    ac.emit_has_type("u128");
    ac.emit_has_type("i128");

    ac.emit_rustc_version(1, 6);
    ac.emit_rustc_version(1, 3);
}
