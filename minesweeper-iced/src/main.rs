use iced::{window, Application, Settings};
use minesweeper_iced::AppState;
#[cfg(target_os = "macos")]
use {
    fruitbasket::{ActivationPolicy, InstallDir, Trampoline},
    std::path::PathBuf,
};

#[cfg(target_os = "macos")]
fn main() {
    let icon = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("icon.png");

    let app =
        match Trampoline::new("MineSweeper", "my-weather", "com.dr.my-weather")
            .resource(icon.to_str().unwrap())
            .icon("icon.icns")
            .build(InstallDir::Temp)
        {
            Ok(app) => app,
            Err(err) => {
                eprint!("Failed to launch {err}");
                std::process::exit(1);
            }
        };

    app.set_activation_policy(ActivationPolicy::Regular);

    match launch() {
        Ok(_) => {
            fruitbasket::FruitApp::terminate(0);
        }
        Err(err) => {
            fruitbasket::FruitApp::terminate(1);
        }
    }
}

#[cfg(not(target_os = "macos"))]
fn main() -> iced::Result {
    launch()
}

fn launch() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: (600, 700),
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    };
    AppState::run(settings)
}
