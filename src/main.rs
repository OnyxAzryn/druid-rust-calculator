// Use RUI module
mod rui;

// External crates
use druid::{AppLauncher, LocalizedString, WindowDesc};
use rug::Float;

pub fn main() {
    let window = WindowDesc::new(rui::build_calc)
        .window_size((223., 300.))
        .resizable(false)
        .title(
            LocalizedString::new("calc-demo-window-title").with_placeholder("Simple Calculator"),
        );
    let calc_state = rui::calc_state::CalcState {
        value: "0".to_string(),
        operand: rui::druid_float::DruidFloat {
            value: Float::new(16),
        },
        operator: 'C',
        in_num: false,
    };
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(calc_state)
        .expect("launch failed");
}
