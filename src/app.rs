use calculator::calculate::{ Calculate, Enum };

#[derive(Default)]
pub struct MyApp {
    num: f64,
    result: f64,
    operation: Enum,
    str_num: String,
    str_res: String,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        calculator::setup_custom_fonts(&_cc.egui_ctx);
        Self::default()
    }
    
    // type conversion
    pub fn type_conversion(&mut self) {
        let num1: f64 = match self.str_num.trim().parse() {
            Ok(num1) => num1,
            Err(_) => return,
        };
        self.num = num1;
    }

}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {

        eframe::egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("Calculator");
            ui.label(&self.str_num);
            
            // =
            if ui.button("=").clicked() {
                self.type_conversion();
                self.result = Calculate::calculate(&mut Calculate { num: self.num, result: self.result }, &self.operation);
                self.str_res = self.result.to_string();

                self.result = 0.0;
                self.operation = Enum::NoOperation;
                self.num = 0.0;
            }
            ui.label(&self.str_res);
            
            eframe::egui::Grid::new("buttons").show(ui, |ui| {
               
                // input number
                if ui.button("7").clicked() {
                    if self.str_num != String::from("0") {
                        let mut s1 = self.str_num.to_string();
                        let s2 = "7".to_string();
                        s1 += &s2;
                        self.str_num = String::from(s1);
                    } else {
                        self.str_num = String::from("7");
                    }
                }
                if ui.button("8").clicked() {
                    if self.str_num != String::from("0") {
                        let mut s1 = self.str_num.to_string();
                        let s2 = "8".to_string();
                        s1 += &s2;
                        self.str_num = String::from(s1);
                    } else {
                        self.str_num = String::from("8");
                    }
                }
                if ui.button("9").clicked() {
                    if self.str_num != String::from("0") {
                        let mut s1 = self.str_num.to_string();
                        let s2 = "9".to_string();
                        s1 += &s2;
                        self.str_num = String::from(s1);
                    } else {
                        self.str_num = String::from("9");
                    }
                }
                ui.end_row();

                if ui.button("4").clicked() {
                    if self.str_num != String::from("0") {
                        let mut s1 = self.str_num.to_string();
                        let s2 = "4".to_string();
                        s1 += &s2;
                        self.str_num = String::from(s1);
                    } else {
                        self.str_num = String::from("4");
                    }
                }
                if ui.button("5").clicked() {
                    if self.str_num != String::from("0") {
                        let mut s1 = self.str_num.to_string();
                        let s2 = "5".to_string();
                        s1 += &s2;
                        self.str_num = String::from(s1);
                    } else {
                        self.str_num = String::from("5");
                    }
                }
                if ui.button("6").clicked() {
                    if self.str_num != String::from("0") {
                        let mut s1 = self.str_num.to_string();
                        let s2 = "6".to_string();
                        s1 += &s2;
                        self.str_num = String::from(s1);
                    } else {
                        self.str_num = String::from("6");
                    }
                }
                ui.end_row();
                
                if ui.button("1").clicked() {
                    if self.str_num != String::from("0") {
                        let mut s1 = self.str_num.to_string();
                        let s2 = "1".to_string();
                        s1 += &s2;
                        self.str_num = String::from(s1);
                    } else {
                        self.str_num = String::from("1");
                    }
                }
                if ui.button("2").clicked() {
                    if self.str_num != String::from("0") {
                        let mut s1 = self.str_num.to_string();
                        let s2 = "2".to_string();
                        s1 += &s2;
                        self.str_num = String::from(s1);
                    } else {
                        self.str_num = String::from("2");
                    }
                }
                if ui.button("3").clicked() {
                    if self.str_num != String::from("0") {
                        let mut s1 = self.str_num.to_string();
                        let s2 = "3".to_string();
                        s1 += &s2;
                        self.str_num = String::from(s1);
                    } else {
                        self.str_num = String::from("3");
                    }
                }
                ui.end_row();

                if ui.button(".").clicked() {
                    let mut can_be_used = true;
                    let bytes = self.str_num.as_bytes();
                    // A number can only have one decimal point
                    // So if the entered number does not have a decimal point, it can be used
                    for (_i, &item) in bytes.iter().enumerate() {
                        if item == b'.' {
                            can_be_used = false;
                        }
                    }

                    // The decimal point cannot be at the beginning
                    if self.str_num.len() == 0 {
                        can_be_used = false;
                    }

                    if can_be_used {
                        let mut s1 = self.str_num.to_string();
                        let s2 = ".".to_string();
                        s1 += &s2;
                        self.str_num = String::from(s1);
                    }
                }
                if ui.button("0").clicked() {
                    if self.str_num != String::from("0") {
                        let mut s1 = self.str_num.to_string();
                        let s2 = "0".to_string();
                        s1 += &s2;
                        self.str_num = String::from(s1);
                    } else {
                        self.str_num = String::from("0");
                    }
                }
                if ui.button("<").clicked() {
                    if self.str_num.len() != 0 {
                        let s1 = &self.str_num[0..self.str_num.len() - 1];
                        self.str_num = String::from(s1);
                    }
                }
                if ui.button("C").clicked() {
                    self.num = 0.0;
                    self.str_num = String::from("0");
                    self.result = 0.0;
                    self.str_res = String::from("0");
                    self.operation = Enum::NoOperation;
                }
                ui.end_row();

                // operation
                if ui.button("+").clicked() {
                    self.type_conversion();
                    self.result = Calculate::calculate(&mut Calculate { num: self.num, result: self.result }, &self.operation);
                    self.operation = Enum::Addition;
                    self.str_num = String::from("");
                }
                if ui.button("-").clicked() {
                    self.type_conversion();
                    self.result = Calculate::calculate(&mut Calculate { num: self.num, result: self.result }, &self.operation);
                    self.operation = Enum::Subtraction;
                    self.str_num = String::from("");
                }
                if ui.button("*").clicked() {
                    self.type_conversion();
                    self.result = Calculate::calculate(&mut Calculate { num: self.num, result: self.result }, &self.operation);
                    self.operation = Enum::Multiplication;
                    self.str_num = String::from("");
                }
                if ui.button("/").clicked() {
                    self.type_conversion();
                    self.result = Calculate::calculate(&mut Calculate { num: self.num, result: self.result }, &self.operation);
                    self.operation = Enum::Division;
                    self.str_num = String::from("");
                }
                ui.end_row();
            });
        });
    }
}
