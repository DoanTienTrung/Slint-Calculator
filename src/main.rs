// nap thu vien slint

slint::include_modules!(); // tu dong nhung code bien dich tu file ui.slint
mod calc;
use calc::{CalculatorState, Op};
use std::rc::Rc;
use std::cell::RefCell;
fn main() {
    // tao cua so giao dien tu component "caculator"
    let ui = Calculator::new().unwrap(); 

    let state_handle = Rc::new(RefCell::new(CalculatorState::new()));
    // biến lưu nội dung hiện tại trên màn hình
    let ui_handle = ui.as_weak();
    ui.on_button_pressed(move |btn| {
        let ui = ui_handle.upgrade().unwrap();
        let mut state = state_handle.borrow_mut();

        match btn.as_str() {
            "C" => *state = CalculatorState::new(),
            "⌫" => state.backspace(),
            "±" => state.toggle_sign(),
            "." => state.input_dot(),
            "%" => state.percent(),
            "+" => state.set_op(Op::Add),
            "-" => state.set_op(Op::Sub),
            "×" => state.set_op(Op::Mul),
            "÷" => state.set_op(Op::Div),
            "=" =>state.evaluate(),
            d if d.chars().all(|c| c.is_ascii_digit()) => {
                state.input_digital(d.chars().next().unwrap());
            }
            _ => {}
        }
        ui.set_display(state.current.clone().into());

    });

    

    // chay ung dung hien thi ui cho nguoi dung thao tac
    ui.run().unwrap();
}
