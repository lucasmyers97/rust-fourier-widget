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



fn expression_box(ui: &mut egui::Ui, 
                  function_text: &mut String, 
                  function_expr: &mut meval::Expr,
                  l2_error: f64) {

    ui.horizontal(|ui| {
        let name_label = ui.label("f(x): ");
        let single_line = ui.text_edit_singleline(function_text)
                            .labelled_by(name_label.id);
    
        if single_line.lost_focus() {
            match function_text.parse::<meval::Expr>() {
                Ok(expr) => *function_expr = expr,
                Err(_) => println!("Have math error!"),
            }
        }

        ui.label(format!("L2 error: {:.8}", l2_error));
    });
}



fn coeff_slider_column(ui: &mut egui::Ui,
                       coeff_vec: &mut Vec<f64>,
                       is_cos_coeffs: bool,
                       available_width: f32,
                       delta: f32) {

    ui.vertical(|ui| {
        ui.set_width(available_width / 2.);
        ui.style_mut().spacing.slider_width
            = ui.available_width() - delta;
    
        ui.horizontal(|ui| {
            let plus_button = ui.button("+");
            let minus_button = ui.button("-");
            if plus_button.clicked() {
                coeff_vec.push(0.);
            }
            if minus_button.clicked() {
                coeff_vec.pop();
            }
        });
        for (i, coeff) in coeff_vec.iter_mut().enumerate() {
            let slider_text = if is_cos_coeffs { format!("A{}", i) }
                              else { format!("B{}", i + 1) };

            ui.add( egui::Slider::new(coeff, -10.0..=10.0).text(slider_text) );
        }
    });
}



fn make_plot_points(f: impl Fn(f64) -> f64) -> egui::plot::PlotPoints {

    (-1000..1000).map(|i| { let x = i as f64 * 0.01;
                            [x, f(x)] }).collect()
}



struct MyApp {
    function_text: String,
    expr: meval::Expr,
    sin_coeff_vec: Vec<f64>,
    cos_coeff_vec: Vec<f64>,
    l2_error: f64,
}



impl Default for MyApp {
    fn default() -> Self {
        Self {
            function_text: "x^2".to_owned(),
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

            expression_box(ui, 
                           &mut self.function_text, 
                           &mut self.expr,
                           self.l2_error);
            
            let func = self.expr.clone().bind("x").unwrap();

            self.l2_error 
                = peroxide::fuga::integrate(|x| {
                    let f_diff = func(x) - fourier_sum(x, 
                                                       &self.cos_coeff_vec, 
                                                       &self.sin_coeff_vec);
                    f_diff * f_diff
            }, (-PI, PI), peroxide::fuga::G30K61(1e-16)).sqrt();

            let available_width = ui.available_width();
            let delta = 100.;
            ui.horizontal(|ui| {
                coeff_slider_column(ui, 
                                    &mut self.cos_coeff_vec, 
                                    /* is_cos_coeffs = */ true, 
                                    available_width, 
                                    delta);
                ui.separator();
                coeff_slider_column(ui, 
                                    &mut self.sin_coeff_vec, 
                                    /* is_cos_coeffs = */ false, 
                                    available_width, 
                                    delta);
            });
            
            let fourier_points = make_plot_points(
                                    |x|fourier_sum(x, 
                                                   &self.cos_coeff_vec, 
                                                   &self.sin_coeff_vec) 
                                    );
            let fourier_curve = egui::plot::Line::new(fourier_points);

            let function_points = make_plot_points(func);
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
