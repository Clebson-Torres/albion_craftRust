use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::thread;

use eframe::egui;

use crate::config::AlbionServer;
use crate::domain::opportunity::Opportunity;
use crate::services::crafting::SourcingStrategy;
use crate::services::ranking::PremiumStatus;
use crate::services::refresh::RefreshResult;
use crate::services::refresh::RefreshService;

const UI_POLL_INTERVAL: std::time::Duration = std::time::Duration::from_millis(250);

#[derive(Debug, Clone)]
pub enum UiLoadState {
    Loading,
    Ready(RefreshResult),
    Error(String),
}

pub struct OverlayAppInit {
    pub receiver: Receiver<UiLoadState>,
    pub sender: Sender<UiLoadState>,
    pub refresh_service: Arc<RefreshService>,
    pub hotkey_toggle: Arc<AtomicBool>,
    pub available_cities: Vec<String>,
    pub server: AlbionServer,
    pub current_city: String,
    pub sourcing_strategy: SourcingStrategy,
    pub transport_cost_per_unit: u64,
    pub premium_status: PremiumStatus,
    pub budget: u64,
    pub top_limit: usize,
}

pub struct OverlayApp {
    receiver: Receiver<UiLoadState>,
    sender: Sender<UiLoadState>,
    refresh_service: Arc<RefreshService>,
    top_limit: usize,
    available_cities: Vec<String>,
    server: AlbionServer,
    current_city: String,
    sourcing_strategy: SourcingStrategy,
    transport_cost_per_unit: u64,
    premium_status: PremiumStatus,
    budget: u64,
    hotkey_toggle: Arc<AtomicBool>,
    visible: bool,
    state: UiLoadState,
    selected_item_id: Option<String>,
}

impl OverlayApp {
    pub fn new(init: OverlayAppInit) -> Self {
        Self {
            receiver: init.receiver,
            sender: init.sender,
            refresh_service: init.refresh_service,
            top_limit: init.top_limit,
            available_cities: init.available_cities,
            server: init.server,
            current_city: init.current_city,
            sourcing_strategy: init.sourcing_strategy,
            transport_cost_per_unit: init.transport_cost_per_unit,
            premium_status: init.premium_status,
            budget: init.budget,
            hotkey_toggle: init.hotkey_toggle,
            visible: true,
            state: UiLoadState::Loading,
            selected_item_id: None,
        }
    }

    fn selected_opportunity<'a>(&'a self, items: &'a [Opportunity]) -> Option<&'a Opportunity> {
        let selected = self.selected_item_id.as_deref()?;
        items.iter().find(|item| item.item_id == selected)
    }

    fn should_render_contents(&self) -> bool {
        self.visible
    }

    fn trigger_refresh(&self) {
        let sender = self.sender.clone();
        let refresh_service = self.refresh_service.clone();
        let top_limit = self.top_limit;
        thread::spawn(move || {
            let runtime = tokio::runtime::Runtime::new().expect("tokio runtime");
            let next_state = match runtime.block_on(refresh_service.refresh_top(top_limit)) {
                Ok(data) => UiLoadState::Ready(data),
                Err(error) => UiLoadState::Error(error.to_string()),
            };
            let _ = sender.send(next_state);
        });
    }
}

impl eframe::App for OverlayApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(next) = self.receiver.try_recv() {
            if let UiLoadState::Ready(result) = &next {
                if self.selected_item_id.is_none() {
                    self.selected_item_id = result.items.first().map(|item| item.item_id.clone());
                }
            }
            self.state = next;
        }

        if self.hotkey_toggle.swap(false, Ordering::SeqCst) {
            self.visible = !self.visible;
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(self.visible));
        }

        if !self.should_render_contents() {
            ctx.request_repaint_after(UI_POLL_INTERVAL);
            return;
        }

        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Albion Crafting Overlay");
                match &self.state {
                    UiLoadState::Loading => {
                        ui.label("Atualizando...");
                    }
                    UiLoadState::Ready(result) if result.stale => {
                        ui.colored_label(egui::Color32::YELLOW, "Dados antigos");
                    }
                    UiLoadState::Ready(_) => {
                        ui.colored_label(egui::Color32::LIGHT_GREEN, "Dados recentes");
                    }
                    UiLoadState::Error(message) => {
                        ui.colored_label(egui::Color32::LIGHT_RED, message);
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label("Cidade atual:");
                let mut city_changed = false;
                egui::ComboBox::from_id_salt("current-city")
                    .selected_text(&self.current_city)
                    .show_ui(ui, |ui| {
                        for city in &self.available_cities {
                            if ui
                                .selectable_value(&mut self.current_city, city.clone(), city)
                                .changed()
                            {
                                city_changed = true;
                            }
                        }
                    });

                ui.label("Compra:");
                let previous_strategy = self.sourcing_strategy;
                egui::ComboBox::from_id_salt("sourcing-strategy")
                    .selected_text(self.sourcing_strategy.label())
                    .show_ui(ui, |ui| {
                        for strategy in SourcingStrategy::ALL {
                            ui.selectable_value(
                                &mut self.sourcing_strategy,
                                strategy,
                                strategy.label(),
                            );
                        }
                    });

                ui.label("Transporte:");
                let penalty_changed = ui
                    .add(egui::DragValue::new(&mut self.transport_cost_per_unit).range(0..=500))
                    .changed();

                if city_changed || previous_strategy != self.sourcing_strategy || penalty_changed {
                    self.refresh_service.update_preferences(
                        self.server,
                        self.current_city.clone(),
                        self.sourcing_strategy,
                        self.transport_cost_per_unit,
                        self.premium_status,
                        self.budget,
                    );
                    self.state = UiLoadState::Loading;
                    self.trigger_refresh();
                }
            });
        });

        egui::SidePanel::left("ranking")
            .min_width(260.0)
            .show(ctx, |ui| match &self.state {
                UiLoadState::Loading => {
                    ui.label("Carregando oportunidades...");
                }
                UiLoadState::Error(message) => {
                    ui.label(format!("Falha ao carregar: {message}"));
                }
                UiLoadState::Ready(result) => {
                    ui.label("Top oportunidades");
                    ui.separator();
                    for item in &result.items {
                        let selected =
                            self.selected_item_id.as_deref() == Some(item.item_id.as_str());
                        if ui
                            .selectable_label(
                                selected,
                                format!(
                                    "{} | lucro {} | {}",
                                    item.item_id, item.net_profit, item.sell_target
                                ),
                            )
                            .clicked()
                        {
                            self.selected_item_id = Some(item.item_id.clone());
                        }
                    }
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| match &self.state {
            UiLoadState::Loading => {
                ui.heading("Esperando dados");
                ui.label("A primeira consulta da API pode levar alguns segundos.");
            }
            UiLoadState::Error(message) => {
                ui.heading("Erro");
                ui.label(message);
            }
            UiLoadState::Ready(result) => {
                let fallback = result.items.first();
                let Some(item) = self.selected_opportunity(&result.items).or(fallback) else {
                    ui.label("Nenhuma oportunidade encontrada.");
                    return;
                };

                ui.heading(&item.item_id);
                ui.label(format!("Melhor destino: {}", item.sell_target));
                ui.label(format!("Preco de venda: {}", item.sell_price));
                ui.label(format!("Custo de craft: {}", item.craft_cost));
                ui.label(format!("Lucro liquido: {}", item.net_profit));
                ui.label(format!(
                    "Penalidade transporte/u: {}",
                    self.transport_cost_per_unit
                ));
                ui.label(format!(
                    "Confianca: {} ({})",
                    item.confidence.label, item.confidence.score
                ));
                ui.label(format!("Observado em: {}", item.observed_at));
                ui.separator();
                ui.label("Materiais");
                for ingredient in &item.ingredients {
                    ui.label(format!(
                        "{} x{} em {} por {}",
                        ingredient.material_id,
                        ingredient.amount,
                        ingredient.source_city,
                        ingredient.unit_price
                    ));
                }
            }
        });

        ctx.request_repaint_after(UI_POLL_INTERVAL);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn hidden_overlay_still_requires_polling() {
        let (_sender, receiver) = mpsc::channel();
        let (sender, _unused_receiver) = mpsc::channel();
        let refresh_service = Arc::new(RefreshService::new(
            Arc::new(crate::services::refresh::AlbionApiSource::new(
                AlbionServer::West,
                vec!["Caerleon".to_owned()],
                vec![1],
            )),
            AlbionServer::West,
            "Caerleon".to_owned(),
            SourcingStrategy::CurrentCityOnly,
            0,
            PremiumStatus::Premium,
            150_000,
        ));
        let mut app = OverlayApp::new(OverlayAppInit {
            receiver,
            sender,
            refresh_service,
            hotkey_toggle: Arc::new(AtomicBool::new(false)),
            available_cities: vec!["Caerleon".to_owned()],
            server: AlbionServer::West,
            current_city: "Caerleon".to_owned(),
            sourcing_strategy: SourcingStrategy::CurrentCityOnly,
            transport_cost_per_unit: 0,
            premium_status: PremiumStatus::Premium,
            budget: 150_000,
            top_limit: 10,
        });

        app.visible = false;

        assert!(!app.should_render_contents());
        assert_eq!(UI_POLL_INTERVAL, std::time::Duration::from_millis(250));
    }
}
