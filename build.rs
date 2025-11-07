fn main() {
    // báo cho Cargo biết hãy biên dịch file ui.slint
    slint_build::compile("src/ui.slint").unwrap();
}