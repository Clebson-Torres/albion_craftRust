import { invoke } from "@tauri-apps/api/core";

const statusEl = document.querySelector("#status");
const versionBadge = document.querySelector("#version-badge");
const hotkeyBadge = document.querySelector("#hotkey-badge");
const serverSelect = document.querySelector("#server-select");
const citySelect = document.querySelector("#city-select");
const strategySelect = document.querySelector("#strategy-select");
const transportInput = document.querySelector("#transport-input");
const premiumSelect = document.querySelector("#premium-select");
const budgetInput = document.querySelector("#budget-input");
const refreshButton = document.querySelector("#refresh-button");
const rankingList = document.querySelector("#ranking-list");
const detailView = document.querySelector("#detail-view");
const metaText = document.querySelector("#meta-text");

const serverOptions = [
  { value: "West", label: "West" },
  { value: "East", label: "East" },
  { value: "Europe", label: "Europe" }
];

const strategyOptions = [
  { value: "CurrentCityOnly", label: "Compra local" },
  { value: "SingleCity", label: "Cidade unica" },
  { value: "CheapestPerIngredient", label: "Mais barato por material" }
];

const premiumOptions = [
  { value: "Premium", label: "Premium" },
  { value: "Standard", label: "Sem premium" }
];

const state = {
  config: null,
  selectedItemId: null,
  result: null
};

function formatNumber(value) {
  return Number(value ?? 0).toLocaleString("pt-BR");
}

function itemIconUrl(itemId) {
  return `https://render.albiononline.com/v1/item/${encodeURIComponent(itemId)}.png?quality=1`;
}

function setStatus(text, kind = "default") {
  statusEl.textContent = text;
  statusEl.className = `status ${kind}`;
}

function renderControls(config) {
  versionBadge.textContent = `v${config.version}`;
  hotkeyBadge.textContent = config.hotkey;

  serverSelect.innerHTML = "";
  for (const server of serverOptions) {
    const option = document.createElement("option");
    option.value = server.value;
    option.textContent = server.label;
    if (server.value === config.server) option.selected = true;
    serverSelect.append(option);
  }

  citySelect.innerHTML = "";
  for (const city of config.locations) {
    const option = document.createElement("option");
    option.value = city;
    option.textContent = city;
    if (city === config.currentCity) option.selected = true;
    citySelect.append(option);
  }

  strategySelect.innerHTML = "";
  for (const strategy of strategyOptions) {
    const option = document.createElement("option");
    option.value = strategy.value;
    option.textContent = strategy.label;
    if (strategy.value === config.sourcingStrategy) option.selected = true;
    strategySelect.append(option);
  }

  premiumSelect.innerHTML = "";
  for (const premium of premiumOptions) {
    const option = document.createElement("option");
    option.value = premium.value;
    option.textContent = premium.label;
    if (premium.value === config.premiumStatus) option.selected = true;
    premiumSelect.append(option);
  }

  transportInput.value = String(config.transportCostPerUnit);
  budgetInput.value = String(config.budget);
}

function renderRanking(result) {
  rankingList.innerHTML = "";
  metaText.textContent = result.stale
    ? "Mostrando o ultimo refresh valido"
    : "Dados recentes para decisao rapida";

  if (!result.items.length) {
    rankingList.textContent = "Nenhuma oportunidade encontrada.";
    detailView.className = "detail-view empty";
    detailView.textContent = "Nenhuma oportunidade encontrada para essa configuracao.";
    return;
  }

  if (!state.selectedItemId || !result.items.some((item) => item.itemId === state.selectedItemId)) {
    state.selectedItemId = result.items[0].itemId;
  }

  for (const item of result.items) {
    const selected = item.itemId === state.selectedItemId;
    const button = document.createElement("button");
    button.type = "button";
    button.className = `ranking-item ${selected ? "active" : ""}`;
    button.innerHTML = `
      <img class="item-icon" src="${itemIconUrl(item.itemId)}" alt="${item.itemId}">
      <div class="item-main">
        <div class="item-row">
          <strong>${item.itemId}</strong>
          <span class="item-sub">${item.sellTarget}</span>
        </div>
        <p class="item-sub">Lucro/u ${formatNumber(item.netProfit)} | Total ${formatNumber(item.totalNetProfit)}</p>
        <p class="item-sub">Craft ${formatNumber(item.craftCost)} | Qtd ${formatNumber(item.maxQuantity)}</p>
      </div>
      <div class="item-profit">
        <strong>${formatNumber(item.totalNetProfit)}</strong>
        <span class="item-sub">${item.confidence.label}</span>
      </div>
    `;
    button.addEventListener("click", () => {
      state.selectedItemId = item.itemId;
      renderRanking(state.result);
      renderDetail();
    });
    rankingList.append(button);
  }
}

function buildSummaryCard(label, value, extraClass = "") {
  return `
    <div class="detail-card">
      <span>${label}</span>
      <strong class="${extraClass}">${value}</strong>
    </div>
  `;
}

function renderDetail() {
  const item = state.result?.items.find((entry) => entry.itemId === state.selectedItemId);
  if (!item) {
    detailView.className = "detail-view empty";
    detailView.textContent = "Selecione uma oportunidade para ver os detalhes.";
    return;
  }

  const premiumLabel = state.config.premiumStatus === "Premium" ? "Premium" : "Sem premium";
  const confidenceClass = `confidence-${item.confidence.label.toLowerCase()}`;
  detailView.className = "detail-view";
  detailView.innerHTML = `
    <div class="detail-head">
      <img class="item-icon" src="${itemIconUrl(item.itemId)}" alt="${item.itemId}">
      <div>
        <h2>${item.itemId}</h2>
        <p>${item.sellTarget}</p>
        <p class="detail-sub">${state.config.server} | ${premiumLabel} | Atualizado ${item.observedAt}</p>
      </div>
    </div>

    <div class="detail-summary">
      ${buildSummaryCard("Lucro liquido/u", formatNumber(item.netProfit))}
      ${buildSummaryCard("Lucro total", formatNumber(item.totalNetProfit), item.netProfit >= 0 ? "confidence-high" : "confidence-low")}
      ${buildSummaryCard("Custo de craft", formatNumber(item.craftCost))}
      ${buildSummaryCard("Venda", formatNumber(item.sellPrice))}
      ${buildSummaryCard("Qtd maxima", formatNumber(item.maxQuantity))}
      ${buildSummaryCard("Confianca", `${item.confidence.label} (${item.confidence.score})`, confidenceClass)}
    </div>

    <div class="detail-groups">
      <div>
        <div class="group-header">
          <h3>Financeiro</h3>
          <p>Capital e taxas aplicadas</p>
        </div>
        <div class="detail-summary">
          ${buildSummaryCard("Capital usado", formatNumber(item.budgetUsed))}
          ${buildSummaryCard("Capital restante", formatNumber(item.budgetRemaining))}
          ${buildSummaryCard("Taxa de compra", formatNumber(item.buyFeeTotal))}
          ${buildSummaryCard("Taxa de venda", formatNumber(item.sellFeeTotal))}
          ${buildSummaryCard("Impacto total", formatNumber(item.totalFeeImpact))}
          ${buildSummaryCard("Transporte/u", formatNumber(state.config.transportCostPerUnit))}
        </div>
      </div>

      <div>
        <div class="group-header">
          <h3>Materiais</h3>
          <p>Lista de compra do item selecionado</p>
        </div>
        <div class="materials">
          ${item.ingredients.map((ingredient) => `
            <div class="material-row">
              <div class="material-main">
                <img class="material-icon" src="${itemIconUrl(ingredient.materialId)}" alt="${ingredient.materialId}">
                <div>
                  <strong>${ingredient.materialId} x${formatNumber(ingredient.amount)}</strong>
                  <span>${ingredient.sourceCity} | ${formatNumber(ingredient.unitPrice)} cada</span>
                </div>
              </div>
              <strong>${formatNumber(ingredient.unitPrice * ingredient.amount)}</strong>
            </div>
          `).join("")}
        </div>
      </div>
    </div>
  `;
}

async function refresh() {
  setStatus("Atualizando...");
  const result = await invoke("refresh_opportunities");
  state.result = result;
  setStatus(result.stale ? "Usando cache" : "Dados recentes", result.stale ? "warn" : "ok");
  renderRanking(result);
  renderDetail();
}

async function applyPreferences() {
  state.config.server = serverSelect.value;
  state.config.currentCity = citySelect.value;
  state.config.sourcingStrategy = strategySelect.value;
  state.config.transportCostPerUnit = Number(transportInput.value || 0);
  state.config.premiumStatus = premiumSelect.value;
  state.config.budget = Number(budgetInput.value || 0);

  await invoke("update_preferences", {
    preferences: {
      server: state.config.server,
      currentCity: state.config.currentCity,
      sourcingStrategy: state.config.sourcingStrategy,
      transportCostPerUnit: state.config.transportCostPerUnit,
      premiumStatus: state.config.premiumStatus,
      budget: state.config.budget
    }
  });

  await refresh();
}

async function boot() {
  state.config = await invoke("get_ui_config");
  renderControls(state.config);

  serverSelect.addEventListener("change", applyPreferences);
  citySelect.addEventListener("change", applyPreferences);
  strategySelect.addEventListener("change", applyPreferences);
  transportInput.addEventListener("change", applyPreferences);
  premiumSelect.addEventListener("change", applyPreferences);
  budgetInput.addEventListener("change", applyPreferences);
  refreshButton.addEventListener("click", refresh);

  await refresh();
  window.setInterval(refresh, 30000);
}

boot().catch((error) => {
  console.error(error);
  setStatus("Falha ao carregar", "bad");
  detailView.className = "detail-view empty";
  detailView.textContent = String(error);
});
