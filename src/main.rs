// Simple RPN Calculator in Slint and Rust
// cargo build --release

slint::include_modules!();

use std::{cell::RefCell, rc::Rc};

#[derive(Default)]
struct Calc {
    t: f64,
    z: f64,
    y: f64,
    x: f64,
    temp: f64,
    integer_part: String,
    decimal_part: String,
    decimal_flag: bool,
    binary_flag: bool,
    lstx: f64,
    mem: f64, // just 1 STO & RCL memory to make it easier
}

impl Calc {
    fn stop_decimal_input(&mut self) {
        self.decimal_flag = false;
        self.integer_part.clear();
        self.decimal_part.clear();
    }

    fn push(&mut self) {
        self.t = self.z;
        self.z = self.y;
        self.y = self.x;
    }

    fn pop(&mut self) {
        self.y = self.z;
        self.z = self.t;
    }

    fn enter(&mut self) {
        self.push();
        self.x = self.temp;
        self.temp = 0.0;
        self.binary_flag = true;
    }

    fn addition(&mut self) {
        if self.binary_flag {
            self.push();
            self.x = self.temp;
            self.temp = 0.0;
            self.lstx = self.x;
            self.x += self.y;
            self.pop();
            self.binary_flag = false;
        } else {
            self.lstx = self.x;
            self.x += self.y;
            self.pop();
        }
    }

    fn subtraction(&mut self) {
        if self.binary_flag {
            self.push();
            self.x = self.temp;
            self.temp = 0.0;
            self.lstx = self.x;
            self.x = self.y - self.x;
            self.pop();
            self.binary_flag = false;
        } else {
            self.lstx = self.x;
            self.x = self.y - self.x;
            self.pop();
        }
    }

    fn division(&mut self) {
        if self.binary_flag {
            self.push();
            self.x = self.temp;
            self.temp = 0.0;
            self.lstx = self.x;
            self.x = self.y / self.x;
            self.pop();
            self.binary_flag = false;
        } else {
            self.lstx = self.x;
            self.x = self.y / self.x;
            self.pop();
        }
    }

    fn multiplication(&mut self) {
        if self.binary_flag {
            self.push();
            self.x = self.temp;
            self.temp = 0.0;
            self.lstx = self.x;
            self.x *= self.y;
            self.pop();
            self.binary_flag = false;
        } else {
            self.lstx = self.x;
            self.x *= self.y;
            self.pop();
        }
    }

    // fn showregs(&self) {
    //     println!("T: {}", self.t);
    //     println!("Z: {}", self.z);
    //     println!("Y: {}", self.y);
    //     println!("X: {}", self.x);
    //     println!("---------");
    // }

    fn clx(&mut self) {
        self.x = 0.0;
        self.temp = 0.0;
    }

    fn clr(&mut self) {
        self.clx();
        self.t = 0.0;
        self.z = 0.0;
        self.y = 0.0;
    }

    fn roll_down(&mut self) {
        self.temp = self.x;
        self.x = self.y;
        self.y = self.z;
        self.z = self.t;
        self.t = self.temp;
    }

    fn xy(&mut self) {
        self.temp = self.x;
        self.x = self.y;
        self.y = self.temp;
        self.temp = 0.0;
    }

    fn last_x(&mut self) {
        self.push();
        self.x = self.lstx;
        self.temp = 0.0;
        self.binary_flag = true;
    }

    fn sto(&mut self) {
        self.mem = self.temp;
        self.x = self.temp;
        self.temp = 0.0;
    }

    fn rcl(&mut self) {
        self.x = self.mem;
        self.temp = 0.0;
    }
}

// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // no console in release for GUI apps
#[quit::main]
fn main() {
    let app = App::new().unwrap();
    let weak = app.as_weak();
    let calc = Rc::new(RefCell::new(Calc::default()));
    app.global::<CalcLogic>().on_button_pressed(move |value| {
        let app = weak.unwrap();

        let mut calc = calc.borrow_mut();
        if let Ok(val) = value.parse::<f64>() {
            if !calc.decimal_flag {
                calc.temp *= 10.0;
                calc.temp += val;
                app.set_value(calc.temp as f32);
            } else {
                calc.decimal_part.push_str(&value);
                let mut float_num = String::new();
                float_num.push_str(&calc.integer_part);
                float_num.push_str(&calc.decimal_part);
                calc.temp = float_num.parse::<f64>().unwrap();
                app.set_value(calc.temp as f32);
            }
        }
        match value.as_str() {
            "/" => {
                calc.stop_decimal_input();
                calc.division();
                app.set_value(calc.x as f32);
                // calc.showregs();
            }
            "*" => {
                calc.stop_decimal_input();
                calc.multiplication();
                app.set_value(calc.x as f32);
                // calc.showregs();
            }
            "-" => {
                calc.stop_decimal_input();
                calc.subtraction();
                app.set_value(calc.x as f32);
                // calc.showregs();
            }
            "+" => {
                calc.stop_decimal_input();
                calc.addition();
                app.set_value(calc.x as f32);
                // calc.showregs();
            }
            "." => {
                if !calc.decimal_flag {
                    calc.decimal_flag = true;
                    calc.integer_part = calc.temp.to_string();
                    calc.decimal_part.push('.');

                    let mut float_num = String::new();
                    float_num.push_str(&calc.integer_part);
                    float_num.push_str(&calc.decimal_part);
                    calc.temp = float_num.parse::<f64>().unwrap();
                    app.set_value(calc.temp as f32);
                }
            }
            "ENT" => {
                // Enter key
                calc.stop_decimal_input();
                calc.enter();
                app.set_value(calc.x as f32);
                // calc.showregs();
            }
            "CLx" => {
                // C key
                calc.stop_decimal_input();
                calc.clx();
                app.set_value(calc.x as f32);
                // calc.showregs();
            }
            "CLR" => {
                // Esc key
                calc.stop_decimal_input();
                calc.clr();
                app.set_value(calc.x as f32);
                // calc.showregs();
            }
            "R↓" => {
                // D key
                calc.stop_decimal_input();
                calc.roll_down();
                app.set_value(calc.x as f32);
                // calc.showregs();
            }
            "x⇔y" => {
                // Z key
                calc.stop_decimal_input();
                calc.xy();
                app.set_value(calc.x as f32);
                // calc.showregs();
            }
            "LA x" => {
                // X key
                calc.stop_decimal_input();
                calc.last_x();
                app.set_value(calc.x as f32);
                // calc.showregs();
            }
            "STO" => {
                // S key
                calc.stop_decimal_input();
                calc.sto();
                app.set_value(calc.x as f32);
                // calc.showregs();
            }
            "RCL" => {
                // R key
                calc.stop_decimal_input();
                calc.rcl();
                app.set_value(calc.x as f32);
                // calc.showregs();
            }
            "q" => {
                quit::with_code(0);
            }
            _ => {}
        }
    });
    app.run().unwrap();
}
