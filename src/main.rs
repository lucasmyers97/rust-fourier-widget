#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::f64::consts::PI;

use eframe::egui;
extern crate meval;

extern crate peroxide;

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(500.0, 400.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}


fn fourier_sum(x: f64, a: &Vec<f64>, b: &Vec<f64>) -> f64 {

    let cos_sum = a.iter().enumerate().map(|(n, a_n)| {
        a_n * (x * n as f64).cos()
    }).sum::<f64>();

    let sin_sum = b.iter().enumerate().map(|(n, b_n)| {
        b_n * (x * (n + 1) as f64).sin()
    }).sum::<f64>();

    return cos_sum + sin_sum;
}



struct MyApp {
    function: String,
    expr: meval::Expr,
    sin_coeff_vec: Vec<f64>,
    cos_coeff_vec: Vec<f64>,
    l2_error: f64,
}



impl Default for MyApp {
    fn default() -> Self {
        Self {
            function: "x^2".to_owned(),
            expr: "x^2".parse::<meval::Expr>().unwrap(),
            sin_coeff_vec: vec![], 
            cos_coeff_vec: vec![],
            l2_error: 0.,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("Fourier series widget");

            ui.horizontal(|ui| {
                let name_label = ui.label("f(x): ");
                let single_line = ui.text_edit_singleline(&mut self.function)
                                    .labelled_by(name_label.id);

                if single_line.lost_focus() {
                    match self.function.parse::<meval::Expr>() {
                        Ok(expr) => self.expr = expr,
                        Err(_) => println!("Have math error!"),
                    }
                }

                ui.label(format!("L2 error: {:.5}", self.l2_error));
            });
            
            let func = self.expr.clone().bind("x").unwrap();

            self.l2_error 
                = peroxide::prelude::integrate(|x| {
                (func(x) - fourier_sum(x, 
                                       &self.cos_coeff_vec, 
                                       &self.sin_coeff_vec)).abs()
            }, (0., PI));

            let available_width = ui.available_width();
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.set_width(available_width / 2.);

                    let cos_button = ui.button("+");
                    if cos_button.clicked()
                    {
                        self.cos_coeff_vec.push(0.);
                    }
                    for (i, coeff) in self.cos_coeff_vec.iter_mut().enumerate() {
                        ui.add( egui::Slider::new(coeff, -10.0..=10.0)
                                .text(format!("A{}", i)) );
                    }
                });
                ui.separator();
                ui.vertical(|ui| {
                    ui.set_width(available_width / 2.);

                    let sin_button = ui.button("+");
                    if sin_button.clicked()
                    {
                        self.sin_coeff_vec.push(0.);
                    }
                    for (i, coeff) in self.sin_coeff_vec.iter_mut().enumerate() {
                        ui.add( egui::Slider::new(coeff, -10.0..=10.0)
                                .text(format!("B{}", i + 1)) );
                    }
                });
            });
            
            let fourier_points: egui::plot::PlotPoints 
                = (-1000..1000).map(|i| { 
                        let x = i as f64 * 0.01;
                        [x, fourier_sum(x, 
                                        &self.cos_coeff_vec, 
                                        &self.sin_coeff_vec)]
                }).collect();
            let fourier_curve = egui::plot::Line::new(fourier_points);

            let function_points: egui::plot::PlotPoints 
                = (-1000..1000).map(|i| {
                    let x = i as f64 * 0.01;
                    [x, func(x)]
                }).collect();
            let function_curve = egui::plot::Line::new(function_points);

            egui::plot::Plot::new("my_plot")
                .view_aspect(2.0)
                .show(ui, |plot_ui| {
                    plot_ui.set_plot_bounds(
                        egui::plot::PlotBounds::from_min_max(
                            [-std::f64::consts::PI, -5.], 
                            [std::f64::consts::PI, 5.]));

                    plot_ui.line(fourier_curve);
                    plot_ui.line(function_curve);
                })
        });
    }
}
