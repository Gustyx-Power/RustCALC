#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use eframe::egui;
use rustcalc::app::CalculatorApp;

#[cfg(target_os = "windows")]
fn start_memory_trimmer() {
    std::thread::spawn(|| {
        // Wait 800ms for initial window and OpenGL driver initialization
        std::thread::sleep(std::time::Duration::from_millis(800));
        rustcalc::trim_memory();

        loop {
            std::thread::sleep(std::time::Duration::from_millis(1500));
            rustcalc::trim_memory();
        }
    });
}

fn generate_rustcalc_icon() -> egui::IconData {
    let size = 64;
    let mut rgba = Vec::with_capacity(size * size * 4);

    for y in 0..size {
        for x in 0..size {
            let is_border = x < 4 || x >= size - 4 || y < 4 || y >= size - 4;
            // Spine of 'R'
            let is_r_spine = (18..=25).contains(&x) && (14..=50).contains(&y);
            // Top loop of 'R'
            let is_r_top = (25..=42).contains(&x) && (14..=20).contains(&y);
            let is_r_right = (38..=45).contains(&x) && (18..=32).contains(&y);
            let is_r_mid = (25..=40).contains(&x) && (28..=34).contains(&y);
            // Diagonal leg of 'R'
            let is_r_leg = (26..=46).contains(&x) && (32..=50).contains(&y) && (x as i32 - y as i32 - 1).abs() <= 4;

            if is_border || is_r_spine || is_r_top || is_r_right || is_r_mid || is_r_leg {
                // Noir Jet
                rgba.extend_from_slice(&[10, 10, 10, 255]);
            } else {
                // Hermès Signature Orange (#F37021)
                rgba.extend_from_slice(&[243, 112, 33, 255]);
            }
        }
    }

    egui::IconData {
        rgba,
        width: size as u32,
        height: size as u32,
    }
}

fn main() -> eframe::Result<()> {
    #[cfg(target_os = "windows")]
    start_memory_trimmer();

    let icon = generate_rustcalc_icon();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([460.0, 700.0])
            .with_min_inner_size([420.0, 620.0])
            .with_title("RustCALC")
            .with_icon(icon)
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "RustCALC",
        options,
        Box::new(|cc| Ok(Box::new(CalculatorApp::new(cc)))),
    )
}
