extern crate autocfg;

fn main() {
    let ac = autocfg::new();

    ac.emit_has_type("u128");
    ac.emit_has_type("i128");

    #[cfg(test)]
    {
        if ac.probe_rustc_version(1, 25) {
            autocfg::emit("use_nested_groups");
        }
    }
}
