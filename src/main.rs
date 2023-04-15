// Simple RPN Calculator in Slint and Rust
// cargo build --release

use std::{cell::RefCell, rc::Rc};

slint::slint! {
    import {Button, VerticalBox} from "std-widgets.slint";

    export global CalcLogic {
        callback button-pressed(string);
    }

    component Button {
        in property <string> text;
        Rectangle {
            background: ta.pressed ? red : ta.has-hover ? #255 : #0d3955;
            //animate background {duration: 500ms;}
            border-radius: 4px;
            border-width: 2px;
            border-color: self.background.darker(10%);
            ta := TouchArea {
                clicked => {CalcLogic.button-pressed(root.text)}
            }
        }
        Text { text: root.text; color: #f5f9f5;}
    }
    export component App inherits Window {
        title: "RPN Calculator";
        background: #605e59;
        width: 300px;
        height: 300px;

        in property <float> value: 0;

        GridLayout {
            padding: 10px;
            spacing: 10px;

            Text {
                horizontal-alignment: right;
                text: "0";
                colspan: 5;
                font-size: 24px;

            }
            Row {
                Button {text: "%T";}
                Button {text: "Δ%";}
                Button {text: " %  ";}
                Button {text: "STO";}
                Button {text: "RCL";}
            }
            Row {
                Button {text: "CHS";}
                Button {text: "7";}
                Button {text: "8";}
                Button {text: "9";}
                Button {text: "/";}
            }
            Row {
                Button {text: "CLX";}
                Button {text: "4";}
                Button {text: "5";}
                Button {text: "6";}
                Button {text: "*";}

            }
            Row {
                Button {text: "ENT"; rowspan: 2;}
                Button {text: "1";}
                Button {text: "2";}
                Button {text: "3";}
                Button {text: "-";}
            }
            Row {
                // Button {text: "";}
                Button {text: "0"; col: 1;}
                Button {text: ".";}
                Button {text: "f";}
                Button {text: "+";}
            }
        }

    }
}

#[derive(Default)]
struct CalcState {
    t: f32,
    z: f32,
    y: f32,
    x: f32,
    operator: slint::SharedString,
}

fn main() {
    let app = App::new().unwrap();
    let weak = app.as_weak();
    let state = Rc::new(RefCell::new(CalcState::default()));

    app.global::<CalcLogic>().on_button_pressed(move |value| {
        let app = weak.unwrap();
        let mut state = state.borrow_mut();
        if let Ok(val) = value.parse::<f32>() {
            state.t = state.z;
            state.z = state.y;
            state.y = state.x;
            state.x = val;
        }
        let val: f32 = value.parse().unwrap();
        app.set_value(app.get_value() * 10.0 + val);
    });
    app.run().unwrap();
    println!("Hello, world!");
}
