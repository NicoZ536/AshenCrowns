//! ASHEN CROWNS – The Age of the Fallen
//!
//! Start: `cargo run --release`
//! Screenshot ohne Fenster: `cargo run --release -- --screenshot shot.png [--scene mission] [--battle] ...`

mod art;
mod art3d;
mod audio_bridge;
mod content;
mod game;
mod hud;
mod menu;
mod mission;
mod settings;
mod ui_common;
mod view;

use ashen_engine::render::GraphicsProfile;
use ashen_engine::AppConfig;
use ashen_sim::env::Weather;
use ashen_sim::mission::{GameMode, MatchSettings};
use game::{AshenGame, AutoRun};

fn usage() -> &'static str {
    "ASHEN CROWNS – Optionen:
  --windowed | --fullscreen       Fenstermodus
  --profile <performance|balanced|quality>
  --no-audio                      Audioausgabe deaktivieren
  --screenshot <datei.png>        Ohne Fenster rendern und Bild speichern
  --size <BxH>                    Bildgröße für Screenshots (Standard 1600x900)
  --frames <n>                    Gerenderte Frames vor dem Screenshot (Standard 90)
  --scene <menu|mission|skirmish|sandbox>
  --sim-seconds <s>               Simulation vor dem Screenshot vorlaufen lassen
  --time <0..1>                   Tageszeit (0.25 Morgen, 0.5 Mittag, 0.76 Dämmerung, 0.9 Nacht)
  --weather <clear|fog|rain|storm|ash|overcast>
  --battle                        Gegnerwelle vor die Basis setzen
  --select-army                   Armee auswählen (zeigt HUD-Auswahl)
  --camera <x,y>                  Kamera auf Kachel zentrieren
  --zoom <zeilen>                 Sichtbare Bildzeilen (220–480)
  --no-ui                         Oberfläche ausblenden"
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info,wgpu_core=warn,wgpu_hal=warn,naga=warn")).init();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut settings = settings::UserSettings::load();
    let mut config = AppConfig { title: "Ashen Crowns – The Age of the Fallen".into(), fullscreen: settings.fullscreen, vsync: settings.vsync, profile: settings.profile, ..Default::default() };
    let mut screenshot: Option<std::path::PathBuf> = None;
    let mut frames = 90;
    let mut size = (1600, 900);
    let mut auto = AutoRun::default();
    let mut i = 0;
    let next = |i: &mut usize| -> String {
        *i += 1;
        args.get(*i).cloned().unwrap_or_default()
    };
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                println!("{}", usage());
                return;
            }
            "--windowed" => config.fullscreen = false,
            "--fullscreen" => config.fullscreen = true,
            "--no-audio" => config.audio_output = false,
            "--profile" => {
                config.profile = match next(&mut i).as_str() {
                    "performance" => GraphicsProfile::Performance,
                    "quality" => GraphicsProfile::Quality,
                    _ => GraphicsProfile::Balanced,
                };
                settings.profile = config.profile;
            }
            "--screenshot" => screenshot = Some(next(&mut i).into()),
            "--frames" => frames = next(&mut i).parse().unwrap_or(90),
            "--size" => {
                let s = next(&mut i);
                if let Some((w, h)) = s.split_once('x') {
                    size = (w.parse().unwrap_or(1600), h.parse().unwrap_or(900));
                }
            }
            "--scene" => {
                let mode = match next(&mut i).as_str() {
                    "mission" => Some(GameMode::Campaign),
                    "skirmish" => Some(GameMode::Skirmish),
                    "sandbox" => Some(GameMode::Sandbox),
                    _ => None,
                };
                auto.start_mission = mode.map(|m| MatchSettings { mode: m, ..Default::default() });
            }
            "--sim-seconds" => auto.sim_seconds = next(&mut i).parse().unwrap_or(0.0),
            "--time" => auto.time_of_day = next(&mut i).parse().ok(),
            "--weather" => {
                auto.weather = Some(match next(&mut i).as_str() {
                    "fog" => Weather::Fog,
                    "rain" => Weather::Rain,
                    "storm" => Weather::Storm,
                    "ash" => Weather::Ashfall,
                    "overcast" => Weather::Overcast,
                    _ => Weather::Clear,
                })
            }
            "--battle" => auto.battle = true,
            "--select-army" => auto.select_army = true,
            "--camera" => {
                let s = next(&mut i);
                if let Some((x, y)) = s.split_once(',') {
                    auto.camera_tile = Some((x.parse().unwrap_or(20.0), y.parse().unwrap_or(50.0)));
                }
            }
            "--zoom" => auto.zoom_lines = next(&mut i).parse().ok(),
            "--no-ui" => auto.hide_ui = true,
            other => {
                eprintln!("Unbekannte Option {other}\n{}", usage());
                std::process::exit(2);
            }
        }
        i += 1;
    }
    let game = AshenGame::new(settings, auto);
    let result = match screenshot {
        Some(path) => {
            config.width = size.0;
            config.height = size.1;
            ashen_engine::run_headless(config, game, frames, 1.0 / 60.0, &path).map(|_| println!("Screenshot gespeichert: {}", path.display()))
        }
        None => ashen_engine::run(config, game),
    };
    if let Err(e) = result {
        eprintln!("Fehler: {e}");
        std::process::exit(1);
    }
}
