fn main() {
    if let Err(error) = run() {
        eprintln!("failed to start overlay: {error}");
    }
}

fn run() -> anyhow::Result<()> {
    use std::sync::atomic::AtomicBool;
    use std::sync::mpsc;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    use eframe::egui;
    use global_hotkey::hotkey::{Code, HotKey, Modifiers};
    use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};

    use albion_crafting_overlay::app::{OverlayApp, OverlayAppInit, UiLoadState};
    use albion_crafting_overlay::config::AlbionServer;
    use albion_crafting_overlay::config::AppConfig;
    use albion_crafting_overlay::services::crafting::SourcingStrategy;
    use albion_crafting_overlay::services::ranking::PremiumStatus;
    use albion_crafting_overlay::services::refresh::{AlbionApiSource, RefreshService};

    let config = AppConfig::default();
    let source = Arc::new(AlbionApiSource::new(
        config.server,
        config.locations.clone(),
        config.qualities.clone(),
    ));
    let refresh_service = Arc::new(RefreshService::new(
        source,
        config.server,
        "Caerleon".to_owned(),
        SourcingStrategy::CurrentCityOnly,
        0,
        PremiumStatus::Premium,
        150_000,
    ));

    let (sender, receiver) = mpsc::channel();
    let refresh_loop = refresh_service.clone();
    let refresh_sender = sender.clone();
    let refresh_interval = config.refresh_interval_seconds;
    let top_limit = config.top_limit;
    thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().expect("tokio runtime");
        loop {
            let result = runtime.block_on(refresh_loop.refresh_top(top_limit));
            let ui_state = match result {
                Ok(data) => UiLoadState::Ready(data),
                Err(error) => UiLoadState::Error(error.to_string()),
            };
            if refresh_sender.send(ui_state).is_err() {
                break;
            }
            thread::sleep(Duration::from_secs(refresh_interval));
        }
    });

    let hotkey_toggle = Arc::new(AtomicBool::new(false));
    let hotkey_state = hotkey_toggle.clone();
    let manager = GlobalHotKeyManager::new()?;
    manager.register(HotKey::new(
        Some(Modifiers::CONTROL | Modifiers::SHIFT),
        Code::KeyA,
    ))?;
    thread::spawn(move || {
        let hotkey_receiver = GlobalHotKeyEvent::receiver();
        while hotkey_receiver.recv().is_ok() {
            hotkey_state.store(true, std::sync::atomic::Ordering::SeqCst);
        }
    });

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_always_on_top()
            .with_inner_size([720.0, 420.0])
            .with_transparent(false)
            .with_title("Albion Crafting Overlay"),
        ..Default::default()
    };

    eframe::run_native(
        "Albion Crafting Overlay",
        native_options,
        Box::new(move |_cc| {
            Ok(Box::new(OverlayApp::new(OverlayAppInit {
                receiver,
                sender,
                refresh_service,
                hotkey_toggle,
                available_cities: config.locations.clone(),
                server: AlbionServer::West,
                current_city: "Caerleon".to_owned(),
                sourcing_strategy: SourcingStrategy::CurrentCityOnly,
                transport_cost_per_unit: 0,
                premium_status: PremiumStatus::Premium,
                budget: 150_000,
                top_limit,
            })))
        }),
    )
    .map_err(|error| anyhow::anyhow!(error.to_string()))?;

    Ok(())
}
