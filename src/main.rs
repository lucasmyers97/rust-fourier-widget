#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::f64::consts::PI;
use itertools::izip;

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
    let cos_sum = a
        .iter()
        .enumerate()
        .map(|(n, a_n)| a_n * (x * n as f64).cos())
        .sum::<f64>();

    let sin_sum = b
        .iter()
        .enumerate()
        .map(|(n, b_n)| b_n * (x * (n + 1) as f64).sin())
        .sum::<f64>();

    return cos_sum + sin_sum;
}

fn expression_box(
    ui: &mut egui::Ui,
    function_text: &mut String,
    function_expr: &mut meval::Expr,
    l2_error: f64,
) {
    ui.horizontal(|ui| {
        let name_label = ui.label("f(x): ");
        let single_line = ui
            .text_edit_singleline(function_text)
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

struct SliderData {
    slider_vals: Vec<f64>,
    slider_minima_text: Vec<String>,
    slider_maxima_text: Vec<String>,
    slider_minima: Vec<f64>,
    slider_maxima: Vec<f64>
}

impl Default for SliderData {
    fn default() -> Self {
        Self {
            slider_vals: vec![],
            slider_minima_text: vec![],
            slider_maxima_text: vec![],
            slider_minima: vec![],
            slider_maxima: vec![]
        }
    }
}

fn coeff_slider_column(
    ui: &mut egui::Ui,
    slider_data: &mut SliderData,
    is_cos_coeffs: bool,
    available_width: f32
) {
    let delta = 180.;

    ui.vertical(|ui| {
        ui.set_width(available_width / 2.);
        ui.style_mut().spacing.slider_width = ui.available_width() - delta;

        ui.horizontal(|ui| {
            let plus_button = ui.button("+");
            let minus_button = ui.button("-");
            if plus_button.clicked() {
                slider_data.slider_vals.push(0.);
                slider_data.slider_minima_text.push("-10.".to_string());
                slider_data.slider_maxima_text.push("10.".to_string());
                slider_data.slider_minima.push(-10.);
                slider_data.slider_maxima.push(10.);
            }
            if minus_button.clicked() {
                slider_data.slider_vals.pop();
                slider_data.slider_minima.pop();
                slider_data.slider_maxima.pop();
            }
        });
        for (i, (coeff, min_text, max_text, min, max)) in izip!(slider_data.slider_vals.iter_mut(),
                                                                slider_data.slider_minima_text.iter_mut(),
                                                                slider_data.slider_maxima_text.iter_mut(),
                                                                slider_data.slider_minima.iter_mut(),
                                                                slider_data.slider_maxima.iter_mut()).enumerate() {
            let slider_text = if is_cos_coeffs {
                format!("A{}", i)
            } else {
                format!("B{}", i + 1)
            };

            *min = match min_text.parse::<f64>() {
                Ok(new_min) => new_min,
                Err(_) => *min,
            };
            *max = match max_text.parse::<f64>() {
                Ok(new_max) => new_max,
                Err(_) => *max,
            };


            ui.horizontal(|ui| {
                let height = ui.available_height();
                ui.label(slider_text);
                ui.add(egui::DragValue::new(coeff).speed((*max - *min) * 0.01));
                ui.add_sized([2. * height, height], 
                             egui::TextEdit::singleline(min_text));
                ui.add(egui::Slider::new(coeff, *min..=*max).show_value(false));
                ui.add_sized([2. * height, height], 
                             egui::TextEdit::singleline(max_text));
            });
        }
    });
}

fn fourier_coeff_pair(
    ui: &mut egui::Ui,
    cos_slider_data: &mut SliderData,
    sin_slider_data: &mut SliderData
) {
    let available_width = ui.available_width();

    ui.horizontal(|ui| {
        coeff_slider_column(
            ui,
            cos_slider_data,
            /* is_cos_coeffs = */ true,
            available_width
        );
        ui.separator();
        coeff_slider_column(
            ui,
            sin_slider_data,
            /* is_cos_coeffs = */ false,
            available_width
        );
    });
}

fn make_plot_points(f: impl Fn(f64) -> f64) -> egui::plot::PlotPoints {
    (-1000..1000)
        .map(|i| {
            let x = i as f64 * 0.01;
            [x, f(x)]
        })
        .collect()
}

fn l2_norm(f: impl Fn(f64) -> f64) -> f64 {
    peroxide::fuga::integrate(
        |x| {
            let f_val = f(x);
            f_val * f_val
        },
        (-PI, PI),
        peroxide::fuga::G30K61(1e-16),
    )
    .sqrt()
}

fn fourier_plot(
    ui: &mut egui::Ui, 
    fourier_points: egui::plot::PlotPoints,
    function_points: egui::plot::PlotPoints
) {
    let available_width = ui.available_width();
    let plot_height_percentage = 0.7;
    let plot_aspect = 2.0;
    let available_height = ui.available_height();
    let plot_height = if available_width > available_height * plot_aspect * plot_height_percentage { 
        plot_height_percentage * available_height
    } else { 
        available_width / plot_aspect
    };
    
    let fourier_curve = egui::plot::Line::new(fourier_points);
    let function_curve = egui::plot::Line::new(function_points);
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        egui::plot::Plot::new("my_plot")
            .view_aspect(plot_aspect)
            .height(plot_height)
            .show(ui, |plot_ui| {
                plot_ui.set_plot_bounds(egui::plot::PlotBounds::from_min_max(
                    [-std::f64::consts::PI, -5.],
                    [std::f64::consts::PI, 5.],
                ));
    
                plot_ui.line(fourier_curve);
                plot_ui.line(function_curve);
            });
    });
}

struct MyApp {
    function_text: String,
    expr: meval::Expr,
    sin_slider_data: SliderData,
    cos_slider_data: SliderData,
    l2_error: f64,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            function_text: "x^2".to_owned(),
            expr: "x^2".parse::<meval::Expr>().unwrap(),
            sin_slider_data: SliderData::default(),
            cos_slider_data: SliderData::default(),
            l2_error: 0.,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.heading("Fourier series widget");
            });

            ui.vertical_centered(|ui| {
                expression_box(ui, &mut self.function_text, &mut self.expr, self.l2_error);
            });

            let func = match self.expr.clone().bind("x") {
                Ok(func) => func,
                Err(_) => "0.".parse::<meval::Expr>().unwrap().bind("x").unwrap(),
            };

            self.l2_error =
                l2_norm(|x| func(x) - fourier_sum(x, 
                                                  &self.cos_slider_data.slider_vals, 
                                                  &self.sin_slider_data.slider_vals));

            let function_points = make_plot_points(func);
            let fourier_points =
                make_plot_points(|x| fourier_sum(x, 
                                                 &self.cos_slider_data.slider_vals, 
                                                 &self.sin_slider_data.slider_vals));

            fourier_plot(ui, fourier_points, function_points);

            egui::containers::ScrollArea::vertical().show(ui, |ui| {
                fourier_coeff_pair(ui, 
                                   &mut self.cos_slider_data, 
                                   &mut self.sin_slider_data);
            });

        });
    }
}
