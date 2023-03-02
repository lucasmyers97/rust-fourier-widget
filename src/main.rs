#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
extern crate meval;

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

struct MyApp {
    function: String,
    expr: meval::Expr,
    coeff_vec: Vec<f64>,
    a1: f64,
    a2: f64,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            function: "x^2".to_owned(),
            expr: "x^2".parse::<meval::Expr>().unwrap(),
            coeff_vec: vec![], 
            a1: 0.0,
            a2: 0.0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("Fourier series widget");

            ui.horizontal(|ui| {
                let name_label = ui.label("Function to approximate: ");
                let single_line = ui.text_edit_singleline(&mut self.function)
                                    .labelled_by(name_label.id);
                if single_line.lost_focus() {
                    match self.function.parse::<meval::Expr>() {
                        Ok(expr) => self.expr = expr,
                        Err(_) => println!("Have math error!"),
                    }
                }
            });
            let func = self.expr.clone().bind("x").unwrap();

            let sin_button = ui.add(egui::Button::new("+"));
            if sin_button.clicked()
            {
                self.coeff_vec.push(0.);
            }
            for (i, coeff) in self.coeff_vec.iter_mut().enumerate() {
                ui.add(egui::Slider::new(coeff, -10.0..=10.0)
                       .text(format!("a{}", i + 1))
                       );
            }
            
            ui.add(egui::Slider::new(&mut self.a1, -10.0..=10.0).text("a1"));
            
            // Make plot
            let mut fourier_points: Vec<egui::plot::PlotPoints> = vec![];
            for (n, coeff) in self.coeff_vec.iter_mut().enumerate() {
                fourier_points.push(
                    (-1000..1000).map(|i| { 
                        let x = i as f64 * 0.01;
                        [x, *coeff * (x * n as f64).sin()] }).collect()
                    );
            }

            let sin: egui::plot::PlotPoints = (-1000..1000).map(|i| {
                let x = i as f64 * 0.01;
                [x, self.a1 * x.sin()]
            }).collect();
            let line = egui::plot::Line::new(sin);

            let mut fourier_curves: Vec<egui::plot::Line> = vec![];
            while let Some(fourier_point) = fourier_points.pop()
            {
                fourier_curves.push(egui::plot::Line::new(fourier_point));
            }

            let my_func: egui::plot::PlotPoints = (-1000..1000).map(|i| {
                let x = i as f64 * 0.01;
                [x, func(x)]
            }).collect();
            let line2 = egui::plot::Line::new(my_func);

            egui::plot::Plot::new("my_plot")
                .view_aspect(2.0)
                .show(ui, |plot_ui| {
                    plot_ui.set_plot_bounds(
                        egui::plot::PlotBounds::from_min_max(
                            [-std::f64::consts::PI, -5.], 
                            [std::f64::consts::PI, 5.]));

                    for fourier_curve in fourier_curves.iter()
                    {
                        plot_ui.line(*fourier_curve);
                    }
                    plot_ui.line(line);
                    plot_ui.line(line2)
                })
        });
    }
}
