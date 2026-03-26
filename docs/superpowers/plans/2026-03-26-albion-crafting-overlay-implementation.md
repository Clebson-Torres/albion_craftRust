# Albion Crafting Overlay Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Rust desktop overlay that shows the top bag and cape crafting opportunities by comparing craft cost against city markets and the Black Market.

**Architecture:** The app is split into a calculation core and a compact overlay UI. The core owns item modeling, Albion API access, normalization, craft-cost and ranking logic; the UI owns hotkey registration, always-on-top window behavior, ranking display, and selected-item details.

**Tech Stack:** Rust, Cargo, `tokio`, `reqwest`, `serde`, `chrono`, `anyhow`, `tracing`, `egui`/`eframe`, `global-hotkey`, `wiremock` or equivalent HTTP mocking crate

---

## File Structure

- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\Cargo.toml`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\main.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\app.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\config.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\domain\mod.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\domain\item.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\domain\recipe.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\domain\market.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\domain\opportunity.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\api\mod.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\api\client.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\api\dto.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\api\mapper.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\services\mod.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\services\catalog.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\services\crafting.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\services\ranking.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\services\refresh.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\tests\catalog_tests.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\tests\api_mapper_tests.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\tests\crafting_tests.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\tests\ranking_tests.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\tests\refresh_tests.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\assets\app_icon.png`
- Modify later if needed: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\docs\superpowers\specs\2026-03-26-albion-crafting-overlay-design.md`

## Chunk 1: Project Bootstrap and Domain Skeleton

### Task 1: Bootstrap the Rust application

**Files:**
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\Cargo.toml`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\main.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\app.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\config.rs`

- [ ] **Step 1: Write the failing compile target**

Create `src/main.rs` with module imports that do not exist yet so the first `cargo test` fails on missing files.

```rust
mod app;
mod config;
mod api;
mod domain;
mod services;

fn main() {
    println!("albion crafting overlay bootstrap");
}
```

- [ ] **Step 2: Run build to verify it fails**

Run: `cargo test`
Expected: FAIL with module file not found errors

- [ ] **Step 3: Add minimal project manifest and empty modules**

Create `Cargo.toml` with:

```toml
[package]
name = "albion-crafting-overlay"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1"
chrono = { version = "0.4", features = ["serde"] }
eframe = "0.33"
egui = "0.33"
global-hotkey = "0.6"
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["rt-multi-thread", "macros", "time"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["fmt"] }

[dev-dependencies]
wiremock = "0.6"
```

Also create minimal `app.rs`, `config.rs`, and placeholder module trees so the crate compiles.

- [ ] **Step 4: Run test suite to verify bootstrap passes**

Run: `cargo test`
Expected: PASS with `0 passed`

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml src
git commit -m "chore: bootstrap albion overlay project"
```

### Task 2: Define domain types for items, recipes, markets, and opportunities

**Files:**
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\domain\mod.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\domain\item.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\domain\recipe.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\domain\market.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\domain\opportunity.rs`

- [ ] **Step 1: Write the failing domain test**

Create `tests/catalog_tests.rs`:

```rust
use albion_crafting_overlay::domain::item::{ItemFamily, ItemKey};

#[test]
fn item_key_preserves_family_and_tier() {
    let item = ItemKey::new(ItemFamily::Bag, 4, 0);
    assert_eq!(item.tier, 4);
    assert_eq!(item.enchantment, 0);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test item_key_preserves_family_and_tier`
Expected: FAIL with unresolved crate items

- [ ] **Step 3: Write minimal domain implementation**

Define:

- `ItemFamily` enum with `Bag` and `Cape`
- `ItemKey { family, tier, enchantment }`
- `RecipeLine { material_id, amount }`
- `Recipe { item_id, ingredients }`
- `MarketSnapshot { item_id, city, sell_price_min, quality, observed_at }`
- `Opportunity { item_id, craft_cost, sell_target, net_profit, confidence }`

Add `src/lib.rs` if needed to expose modules to tests.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test item_key_preserves_family_and_tier`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/domain src/lib.rs tests/catalog_tests.rs
git commit -m "feat: add core domain models"
```

## Chunk 2: Static Catalog and Recipe Modeling

### Task 3: Add the supported bags and capes catalog

**Files:**
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\services\catalog.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\services\mod.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\tests\catalog_tests.rs`

- [ ] **Step 1: Write the failing catalog test**

Add:

```rust
use albion_crafting_overlay::services::catalog::supported_items;

#[test]
fn supported_items_contains_bags_and_capes() {
    let items = supported_items();
    assert!(items.iter().any(|item| item.family.is_bag()));
    assert!(items.iter().any(|item| item.family.is_cape()));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test supported_items_contains_bags_and_capes`
Expected: FAIL with missing catalog service

- [ ] **Step 3: Implement the catalog**

Add a curated internal list for V1 with:

- a small tier range first, such as T4 to T6
- normal bags
- normal capes and faction or city cape variants only if recipe and sale mapping are reliable

Expose helpers:

- `supported_items() -> &'static [ItemKey]`
- `supported_item_ids() -> Vec<String>`
- `supported_material_ids() -> Vec<String>`

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test supported_items_contains_bags_and_capes`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/services/catalog.rs src/services/mod.rs tests/catalog_tests.rs
git commit -m "feat: add supported bags and capes catalog"
```

### Task 4: Model recipes for the supported item set

**Files:**
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\domain\recipe.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\services\catalog.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\tests\crafting_tests.rs`

- [ ] **Step 1: Write the failing recipe lookup test**

```rust
use albion_crafting_overlay::services::catalog::recipe_for;

#[test]
fn recipe_lookup_returns_ingredients_for_supported_item() {
    let recipe = recipe_for("T4_BAG").expect("recipe");
    assert!(!recipe.ingredients.is_empty());
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test recipe_lookup_returns_ingredients_for_supported_item`
Expected: FAIL with missing recipe lookup

- [ ] **Step 3: Implement recipe lookup**

Add a static recipe map keyed by Albion item id string. Keep recipe definitions near the catalog to avoid unnecessary abstraction in V1.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test recipe_lookup_returns_ingredients_for_supported_item`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/domain/recipe.rs src/services/catalog.rs tests/crafting_tests.rs
git commit -m "feat: add recipe data for supported items"
```

## Chunk 3: Albion API Client and Mapping

### Task 5: Implement price endpoint client

**Files:**
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\api\mod.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\api\dto.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\api\client.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\tests\api_mapper_tests.rs`

- [ ] **Step 1: Write the failing HTTP client test**

Use `wiremock` to return a fake `/api/v2/stats/prices/...` payload and assert the client deserializes it.

```rust
#[tokio::test]
async fn client_fetches_price_rows() {
    // arrange mock server and response
    // assert the returned dto list length is 1
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test client_fetches_price_rows -- --nocapture`
Expected: FAIL with missing API client

- [ ] **Step 3: Implement minimal async client**

Add:

- `AlbionApiClient::new(base_url: String)`
- `fetch_prices(item_ids: &[String], locations: &[String]) -> Result<Vec<PriceDto>>`

Use configurable base URL so tests do not hit the real API.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test client_fetches_price_rows -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/api tests/api_mapper_tests.rs
git commit -m "feat: add albion price api client"
```

### Task 6: Map API DTOs into internal market snapshots

**Files:**
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\api\mapper.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\domain\market.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\tests\api_mapper_tests.rs`

- [ ] **Step 1: Write the failing mapper test**

```rust
#[test]
fn mapper_converts_price_dto_into_market_snapshot() {
    // given one dto with timestamp and city
    // expect internal snapshot with parsed timestamp and item id
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test mapper_converts_price_dto_into_market_snapshot`
Expected: FAIL with missing mapper

- [ ] **Step 3: Implement DTO mapping**

Map string and numeric fields into:

- normalized city enum or newtype
- parsed UTC timestamp
- optional min and max prices
- quality

Filter out rows that are unusable for sell-price comparisons.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test mapper_converts_price_dto_into_market_snapshot`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/api/mapper.rs src/domain/market.rs tests/api_mapper_tests.rs
git commit -m "feat: map albion api prices into market snapshots"
```

## Chunk 4: Craft Cost and Opportunity Ranking

### Task 7: Calculate craft cost from recipe ingredients

**Files:**
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\services\crafting.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\tests\crafting_tests.rs`

- [ ] **Step 1: Write the failing craft-cost test**

```rust
use albion_crafting_overlay::services::crafting::calculate_craft_cost;

#[test]
fn calculate_craft_cost_uses_cheapest_available_inputs() {
    // given a recipe and two city prices for ingredients
    // expect total cost from the cheapest valid source
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test calculate_craft_cost_uses_cheapest_available_inputs`
Expected: FAIL with missing crafting service

- [ ] **Step 3: Implement minimal craft-cost calculator**

The function should:

- look up recipe ingredients
- match ingredient price rows
- choose the cheapest valid buy source for each material
- sum material cost
- return both total cost and source breakdown

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test calculate_craft_cost_uses_cheapest_available_inputs`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/services/crafting.rs tests/crafting_tests.rs
git commit -m "feat: calculate crafting cost from ingredient prices"
```

### Task 8: Rank city and Black Market sale opportunities

**Files:**
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\services\ranking.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\tests\ranking_tests.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\domain\opportunity.rs`

- [ ] **Step 1: Write the failing ranking test**

```rust
use albion_crafting_overlay::services::ranking::rank_opportunities;

#[test]
fn rank_opportunities_prefers_higher_net_profit() {
    // given two supported items
    // expect the more profitable one first
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test rank_opportunities_prefers_higher_net_profit`
Expected: FAIL with missing ranking service

- [ ] **Step 3: Implement minimal ranking**

The ranking service should:

- evaluate each supported item
- compare city market vs Black Market sale price
- compute gross profit
- apply net adjustment using a fixed V1 tax assumption
- choose the best destination
- sort descending by score

Start with a fixed constant for market tax to keep V1 small.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test rank_opportunities_prefers_higher_net_profit`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/services/ranking.rs src/domain/opportunity.rs tests/ranking_tests.rs
git commit -m "feat: rank crafting opportunities across cities and black market"
```

### Task 9: Add confidence scoring and stale-data filtering

**Files:**
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\services\ranking.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\tests\ranking_tests.rs`

- [ ] **Step 1: Write the failing confidence test**

```rust
#[test]
fn stale_or_incomplete_data_reduces_confidence() {
    // expect fresher complete data to outrank stale incomplete data
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test stale_or_incomplete_data_reduces_confidence`
Expected: FAIL because ranking ignores freshness

- [ ] **Step 3: Implement confidence heuristics**

Use a small explicit formula combining:

- freshness penalty
- missing data penalty
- margin bonus

Store both a numeric score and a user-facing label such as `High`, `Medium`, `Low`.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test stale_or_incomplete_data_reduces_confidence`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/services/ranking.rs tests/ranking_tests.rs
git commit -m "feat: add confidence scoring for opportunity ranking"
```

## Chunk 5: Refresh Pipeline and Resilience

### Task 10: Build the refresh service that produces the top list

**Files:**
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\services\refresh.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\tests\refresh_tests.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\config.rs`

- [ ] **Step 1: Write the failing refresh test**

```rust
#[tokio::test]
async fn refresh_service_returns_ranked_top_items() {
    // mock API client and assert top list is non-empty
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test refresh_service_returns_ranked_top_items -- --nocapture`
Expected: FAIL with missing refresh service

- [ ] **Step 3: Implement refresh orchestration**

The service should:

- gather supported item ids and material ids
- fetch market data
- map API payloads
- compute ranked opportunities
- return the top N list

Keep network and ranking boundaries explicit so they remain testable.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test refresh_service_returns_ranked_top_items -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/services/refresh.rs src/config.rs tests/refresh_tests.rs
git commit -m "feat: add refresh pipeline for top opportunities"
```

### Task 11: Preserve last valid results on refresh failure

**Files:**
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\services\refresh.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\tests\refresh_tests.rs`

- [ ] **Step 1: Write the failing fallback test**

```rust
#[tokio::test]
async fn refresh_reuses_last_valid_result_when_api_fails() {
    // first refresh succeeds, second fails, previous data should be retained
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test refresh_reuses_last_valid_result_when_api_fails -- --nocapture`
Expected: FAIL because last valid state is not retained

- [ ] **Step 3: Implement last-known-good caching**

Store the most recent valid ranking and mark it stale when reused after failure.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test refresh_reuses_last_valid_result_when_api_fails -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/services/refresh.rs tests/refresh_tests.rs
git commit -m "feat: preserve last valid ranking on refresh failure"
```

## Chunk 6: Overlay UI and Hotkey Behavior

### Task 12: Render the compact ranking window

**Files:**
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\app.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\main.rs`

- [ ] **Step 1: Write the failing UI smoke test or manual verification checklist**

If automated UI testing is too heavy for V1, add a manual verification note in code comments and keep logic testable outside the UI. Document expected states:

- empty state
- loading state
- top list state
- stale state

- [ ] **Step 2: Run the app and verify the window is not yet correct**

Run: `cargo run`
Expected: the placeholder window opens without ranking UI

- [ ] **Step 3: Implement the compact top-list window**

Render:

- title bar or heading
- top 10 ranking list
- item name
- craft cost
- best destination
- net profit
- confidence label

Keep the layout narrow and game-friendly.

- [ ] **Step 4: Run the app to verify the ranking window**

Run: `cargo run`
Expected: window opens and shows mock or live data in compact list form

- [ ] **Step 5: Commit**

```bash
git add src/app.rs src/main.rs
git commit -m "feat: render compact opportunity overlay window"
```

### Task 13: Add selected-item details panel

**Files:**
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\app.rs`

- [ ] **Step 1: Write the failing interaction test or manual verification checklist**

Document:

- selecting a row updates details
- details include materials, cheapest source, best sell target, recency, confidence

- [ ] **Step 2: Run the app and verify detail selection is missing**

Run: `cargo run`
Expected: list is visible but no usable detail panel exists

- [ ] **Step 3: Implement the detail panel**

Use list selection state and render:

- ingredients breakdown
- total craft cost
- source city summary
- best sale destination
- data age
- confidence explanation

- [ ] **Step 4: Run the app to verify selection works**

Run: `cargo run`
Expected: clicking an opportunity updates the detail panel

- [ ] **Step 5: Commit**

```bash
git add src/app.rs
git commit -m "feat: show selected opportunity details"
```

### Task 14: Add global hotkey and overlay-friendly window behavior

**Files:**
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\main.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\app.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\config.rs`

- [ ] **Step 1: Write the failing manual verification checklist**

Document:

- hotkey toggles visibility
- window remains always on top
- app stays responsive while refresh continues

- [ ] **Step 2: Run the app and verify the hotkey is missing**

Run: `cargo run`
Expected: no global toggle behavior yet

- [ ] **Step 3: Implement hotkey and window toggling**

Add:

- configurable hotkey in config
- registration via `global-hotkey`
- visible/hidden UI state
- always-on-top window flags if supported by the chosen crate

- [ ] **Step 4: Run the app to verify overlay behavior**

Run: `cargo run`
Expected: hotkey shows and hides the compact window

- [ ] **Step 5: Commit**

```bash
git add src/main.rs src/app.rs src/config.rs
git commit -m "feat: add overlay hotkey and window behavior"
```

## Chunk 7: Polish, Documentation, and Verification

### Task 15: Add refresh loop and loading or stale states to the UI

**Files:**
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\app.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\services\refresh.rs`

- [ ] **Step 1: Write the failing state test or manual verification checklist**

Document:

- first load shows loading
- failed refresh shows stale badge with previous results
- next success clears stale state

- [ ] **Step 2: Run the app and verify state transitions are incomplete**

Run: `cargo run`
Expected: UI does not clearly represent loading or stale conditions

- [ ] **Step 3: Implement the state handling**

Add explicit app state enums and corresponding UI badges or messages.

- [ ] **Step 4: Run the app to verify the state flow**

Run: `cargo run`
Expected: loading and stale states display correctly

- [ ] **Step 5: Commit**

```bash
git add src/app.rs src/services/refresh.rs
git commit -m "feat: surface loading and stale refresh states"
```

### Task 16: Document V1 limitations and future city-selection support

**Files:**
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\README.md`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\docs\superpowers\specs\2026-03-26-albion-crafting-overlay-design.md`

- [ ] **Step 1: Write the failing docs checklist**

Document the sections that must exist:

- what the app does
- how to run it
- supported items
- known limitations
- future city-selection roadmap

- [ ] **Step 2: Verify docs are absent or incomplete**

Run: `Get-Content README.md`
Expected: file missing or incomplete

- [ ] **Step 3: Write the docs**

Include:

- quickstart
- data-source note
- note that turnover is inferred, not directly provided by the API
- future roadmap for selecting current city

- [ ] **Step 4: Verify docs are present**

Run: `Get-Content README.md`
Expected: README contains usage and limitations

- [ ] **Step 5: Commit**

```bash
git add README.md docs/superpowers/specs/2026-03-26-albion-crafting-overlay-design.md
git commit -m "docs: add v1 usage and roadmap notes"
```

### Task 17: Run final verification before implementation handoff

**Files:**
- No file changes required unless fixes are found

- [ ] **Step 1: Run the full test suite**

Run: `cargo test`
Expected: PASS

- [ ] **Step 2: Run formatting**

Run: `cargo fmt -- --check`
Expected: PASS

- [ ] **Step 3: Run linting**

Run: `cargo clippy --all-targets --all-features -- -D warnings`
Expected: PASS

- [ ] **Step 4: Run the app for manual smoke verification**

Run: `cargo run`
Expected: overlay opens, refreshes, and toggles through hotkey

- [ ] **Step 5: Commit any final fixes**

```bash
git add .
git commit -m "chore: finalize crafting overlay v1"
```

## Notes for the Implementer

- Keep V1 small. Do not add automatic city detection yet.
- Prefer fixed defaults over large configuration surfaces.
- If city cape recipes or market mappings turn out noisy, trim the supported set rather than expanding abstraction.
- If `global-hotkey` or `eframe` introduces platform-specific issues on Windows, document the limitation and keep the core engine working first.
- If UI test automation becomes costly, keep domain and service layers heavily tested and treat UI verification as a small manual pass for V1.
