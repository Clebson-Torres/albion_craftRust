#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use albion_crafting_overlay::config::{AlbionServer, AppConfig};
use albion_crafting_overlay::services::crafting::SourcingStrategy;
use albion_crafting_overlay::services::ranking::PremiumStatus;
use albion_crafting_overlay::services::refresh::{AlbionApiSource, RefreshResult, RefreshService};
use serde::{Deserialize, Serialize};
use tauri::{Manager, State};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

const DEFAULT_CURRENT_CITY: &str = "Caerleon";
const DEFAULT_TRANSPORT_COST_PER_UNIT: u64 = 0;
const DEFAULT_BUDGET: u64 = 150_000;

#[derive(Clone)]
struct AppState {
    refresh_service: Arc<RefreshService>,
    config: AppConfig,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct UiConfig {
    locations: Vec<String>,
    server: AlbionServer,
    current_city: String,
    sourcing_strategy: SourcingStrategy,
    transport_cost_per_unit: u64,
    premium_status: PremiumStatus,
    budget: u64,
    top_limit: usize,
    hotkey: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PreferencesPayload {
    server: AlbionServer,
    current_city: String,
    sourcing_strategy: SourcingStrategy,
    transport_cost_per_unit: u64,
    premium_status: PremiumStatus,
    budget: u64,
}

#[tauri::command]
fn get_ui_config(state: State<'_, AppState>) -> UiConfig {
    UiConfig {
        locations: state.config.locations.clone(),
        server: state.config.server,
        current_city: DEFAULT_CURRENT_CITY.to_owned(),
        sourcing_strategy: SourcingStrategy::CurrentCityOnly,
        transport_cost_per_unit: DEFAULT_TRANSPORT_COST_PER_UNIT,
        premium_status: PremiumStatus::Premium,
        budget: DEFAULT_BUDGET,
        top_limit: state.config.top_limit,
        hotkey: "Ctrl+Shift+A".to_owned(),
    }
}

#[tauri::command]
fn update_preferences(preferences: PreferencesPayload, state: State<'_, AppState>) {
    state.refresh_service.update_preferences(
        preferences.server,
        preferences.current_city,
        preferences.sourcing_strategy,
        preferences.transport_cost_per_unit,
        preferences.premium_status,
        preferences.budget,
    );
}

#[tauri::command]
async fn refresh_opportunities(state: State<'_, AppState>) -> Result<RefreshResult, String> {
    state
        .refresh_service
        .refresh_top(state.config.top_limit)
        .await
        .map_err(|error| error.to_string())
}

fn main() {
    let config = AppConfig::default();
    let source = Arc::new(AlbionApiSource::new(
        config.server,
        config.locations.clone(),
        config.qualities.clone(),
    ));
    let refresh_service = Arc::new(RefreshService::new(
        source,
        config.server,
        DEFAULT_CURRENT_CITY.to_owned(),
        SourcingStrategy::CurrentCityOnly,
        DEFAULT_TRANSPORT_COST_PER_UNIT,
        PremiumStatus::Premium,
        DEFAULT_BUDGET,
    ));

    tauri::Builder::default()
        .manage(AppState {
            refresh_service,
            config,
        })
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let main_window = app.get_webview_window("main").expect("main window");
            let handle = app.handle().clone();
            app.global_shortcut()
                .on_shortcut("Ctrl+Shift+A", move |_app, _shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        if let Some(window) = handle.get_webview_window("main") {
                            let visible = window.is_visible().unwrap_or(true);
                            if visible {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .expect("register global shortcut");

            let _ = main_window.set_always_on_top(true);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_ui_config,
            update_preferences,
            refresh_opportunities
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
