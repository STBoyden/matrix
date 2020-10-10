#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("./tests/correct_args.rs");
    t.compile_fail("./tests/3d_array.rs");
    t.compile_fail("./tests/diff_len_arrs.rs");

    t.pass("./tests/correct_output.rs");
}
