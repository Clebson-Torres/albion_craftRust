# Albion Crafting Overlay V2 Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Extend the Tauri-based Albion crafting overlay so it supports server selection, premium-aware fee calculations, budget-aware ranking, richer materials UX, and GitHub-based CI/CD for Windows and Linux artifacts.

**Architecture:** Keep the Rust domain engine as the source of truth for pricing, sourcing, budget, transport, and ranking. Extend the Tauri backend with operational preferences and richer payloads, and extend the frontend with execution-focused controls and detail rendering. Add repository scaffolding and GitHub Actions workflows as a delivery layer around the app.

**Tech Stack:** Rust, Cargo, Tokio, Serde, Tauri v2, Tauri global shortcut plugin, Vite, vanilla JavaScript, Git, GitHub Actions

---

## File Structure

- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\config.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\services\crafting.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\services\ranking.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\services\refresh.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\domain\opportunity.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\api\client.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src-tauri\src\main.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\ui\main.js`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\ui\index.html`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\ui\styles.css`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\package.json`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src-tauri\tauri.conf.json`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\README.md`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\.gitignore`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\.github\workflows\ci.yml`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\.github\workflows\release.yml`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\tests\budget_tests.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\tests\fees_tests.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\tests\server_tests.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\tests\serialization_tests.rs`

## Chunk 1: Operational Preferences and Fee Model

### Task 1: Add explicit operational preferences for server, premium, budget, and transport

**Files:**
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\config.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\services\refresh.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src-tauri\src\main.rs`

- [ ] **Step 1: Write the failing server configuration test**

Create `tests/server_tests.rs` with a test asserting that selecting a server maps to the correct API base URL.

```rust
#[test]
fn server_selection_maps_to_expected_base_url() {
    assert_eq!(server_base_url("west"), "https://west.albion-online-data.com");
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test server_selection_maps_to_expected_base_url`
Expected: FAIL with missing server mapping support

- [ ] **Step 3: Implement operational preference types**

Add:

- server enum or string-backed validated type
- premium boolean or enum
- budget field
- helper for server host mapping
- default preferences for Tauri startup

- [ ] **Step 4: Run the test to verify it passes**

Run: `cargo test server_selection_maps_to_expected_base_url`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/config.rs src/services/refresh.rs src-tauri/src/main.rs tests/server_tests.rs
git commit -m "feat: add operational preferences and server selection"
```

### Task 2: Add premium-aware buy and sell fee presets

**Files:**
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\services\ranking.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\domain\opportunity.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\tests\fees_tests.rs`

- [ ] **Step 1: Write the failing premium fee test**

Create a test asserting that premium and non-premium produce different fee totals and net profit.

```rust
#[test]
fn premium_changes_total_fee_and_net_profit() {
    // same opportunity, different premium flag
    // expect different fee totals
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test premium_changes_total_fee_and_net_profit`
Expected: FAIL because fee presets are not modeled

- [ ] **Step 3: Implement fee presets**

Add:

- buy fee total
- sell fee total
- total fee impact
- premium preset constants
- non-premium preset constants

- [ ] **Step 4: Run the test to verify it passes**

Run: `cargo test premium_changes_total_fee_and_net_profit`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/services/ranking.rs src/domain/opportunity.rs tests/fees_tests.rs
git commit -m "feat: add premium-aware fee calculations"
```

## Chunk 2: Budget-Aware Calculations and Ranking

### Task 3: Add budget-based quantity and total-profit calculations

**Files:**
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\services\ranking.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\domain\opportunity.rs`
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\tests\budget_tests.rs`

- [ ] **Step 1: Write the failing budget test**

```rust
#[test]
fn budget_limits_quantity_and_total_profit() {
    // given a cost per craft and a budget
    // expect max quantity, budget used, leftover, and total profit
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test budget_limits_quantity_and_total_profit`
Expected: FAIL with missing budget fields

- [ ] **Step 3: Implement budget math**

Add to opportunity output:

- `max_quantity`
- `budget_used`
- `budget_remaining`
- `total_net_profit`

- [ ] **Step 4: Run the test to verify it passes**

Run: `cargo test budget_limits_quantity_and_total_profit`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/services/ranking.rs src/domain/opportunity.rs tests/budget_tests.rs
git commit -m "feat: add budget-aware opportunity metrics"
```

### Task 4: Make default ranking prioritize total profit with budget

**Files:**
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\services\ranking.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\tests\ranking_tests.rs`

- [ ] **Step 1: Write the failing ranking-order test**

Add a test where a lower unit margin wins because it fits the budget better and yields higher total net profit.

```rust
#[test]
fn ranking_prefers_higher_total_profit_with_budget() {
    // two items, one wins by total budget utilization
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test ranking_prefers_higher_total_profit_with_budget`
Expected: FAIL because ranking still prefers another metric

- [ ] **Step 3: Update ranking sort order**

Sort primarily by:

- total net profit
- net profit per unit
- confidence

- [ ] **Step 4: Run the test to verify it passes**

Run: `cargo test ranking_prefers_higher_total_profit_with_budget`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/services/ranking.rs tests/ranking_tests.rs
git commit -m "feat: rank opportunities by total budget return"
```

## Chunk 3: Server Switching and API Integration

### Task 5: Make the API client switch hosts by selected server

**Files:**
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\api\client.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src-tauri\src\main.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\tests\server_tests.rs`

- [ ] **Step 1: Write the failing refresh-level server test**

Add a test asserting that changing the selected server changes the underlying base URL used by the data source.

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test changing_server_updates_data_source_host`
Expected: FAIL because the host is fixed at startup

- [ ] **Step 3: Implement host switching**

Ensure that:

- selected server is stored in preferences
- refresh layer builds or updates the correct base URL
- Tauri preferences command updates the server

- [ ] **Step 4: Run the test to verify it passes**

Run: `cargo test changing_server_updates_data_source_host`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/api/client.rs src-tauri/src/main.rs tests/server_tests.rs
git commit -m "feat: support switching albion data server"
```

## Chunk 4: Tauri Payload and Frontend Expansion

### Task 6: Expand serialization for all V2 fields

**Files:**
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\domain\opportunity.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src\services\refresh.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\tests\serialization_tests.rs`

- [ ] **Step 1: Write the failing serialization test**

Add assertions for:

- `maxQuantity`
- `budgetUsed`
- `budgetRemaining`
- `totalNetProfit`
- `buyFeeTotal`
- `sellFeeTotal`
- `totalFeeImpact`

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test tauri_payload_uses_camel_case_keys`
Expected: FAIL because the new fields are absent

- [ ] **Step 3: Implement payload fields**

Add the new opportunity fields and serialize them in camelCase.

- [ ] **Step 4: Run the test to verify it passes**

Run: `cargo test tauri_payload_uses_camel_case_keys`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/domain/opportunity.rs src/services/refresh.rs tests/serialization_tests.rs
git commit -m "feat: expand tauri payload for budget and fee metrics"
```

### Task 7: Add server, premium, and budget controls to the Tauri UI

**Files:**
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\ui\index.html`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\ui\main.js`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\ui\styles.css`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src-tauri\src\main.rs`

- [ ] **Step 1: Write the failing UI checklist**

Document required V2 controls:

- server selector
- premium toggle
- budget input
- transport input

- [ ] **Step 2: Run the app and verify controls are missing**

Run: `npm run tauri dev`
Expected: UI lacks V2 controls

- [ ] **Step 3: Implement the controls**

Wire the frontend to:

- show server selector
- show premium toggle
- show budget input
- send all values via `update_preferences`

- [ ] **Step 4: Run the app to verify controls**

Run: `npm run tauri dev`
Expected: controls render and changing them triggers refresh

- [ ] **Step 5: Commit**

```bash
git add ui src-tauri/src/main.rs
git commit -m "feat: add v2 controls for server premium and budget"
```

### Task 8: Add material icons and richer financial detail panel

**Files:**
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\ui\main.js`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\ui\styles.css`

- [ ] **Step 1: Write the failing UI checklist**

Document required detail fields:

- item icon
- material icons
- quantity craftable
- budget used
- budget remaining
- total net profit
- fee totals

- [ ] **Step 2: Run the app and verify details are incomplete**

Run: `npm run tauri dev`
Expected: the detail panel is missing the new financial breakdown

- [ ] **Step 3: Implement the richer detail panel**

Render:

- material icons via item id URL
- fee summary
- budget summary
- total profit summary

- [ ] **Step 4: Run the app to verify rendering**

Run: `npm run tauri dev`
Expected: detail panel shows icons and operational numbers correctly

- [ ] **Step 5: Commit**

```bash
git add ui/main.js ui/styles.css
git commit -m "feat: show material icons and financial breakdown"
```

## Chunk 5: Repository Initialization and Ignore Rules

### Task 9: Add repository metadata and ignore rules

**Files:**
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\.gitignore`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\README.md`

- [ ] **Step 1: Write the failing repo checklist**

Required ignore coverage:

- `target`
- `node_modules`
- `dist`
- Tauri build output
- local brainstorm or Codex folders

- [ ] **Step 2: Verify ignore rules are absent**

Run: `Test-Path .gitignore`
Expected: missing or incomplete

- [ ] **Step 3: Add `.gitignore` and repo notes**

Document local development and artifact paths in README.

- [ ] **Step 4: Verify ignore rules**

Run: `Get-Content .gitignore`
Expected: required entries are present

- [ ] **Step 5: Commit**

```bash
git add .gitignore README.md
git commit -m "chore: add repository ignore rules and dev notes"
```

### Task 10: Initialize git and create the baseline repository state

**Files:**
- No source file change required unless docs need an update

- [ ] **Step 1: Verify repository status**

Run: `git rev-parse --show-toplevel`
Expected: fail if repo is not initialized

- [ ] **Step 2: Initialize the repository if needed**

Run: `git init`
Expected: repository created

- [ ] **Step 3: Verify git status**

Run: `git status --short`
Expected: project files are tracked as expected

- [ ] **Step 4: Create baseline commit**

```bash
git add .
git commit -m "chore: initialize albion crafting overlay repository"
```

- [ ] **Step 5: Stop and ask the user for the GitHub remote URL if it is not already known**

The remote cannot be invented safely. Ask only at this point if needed.

## Chunk 6: GitHub Actions CI and Release Delivery

### Task 11: Add GitHub Actions CI workflow

**Files:**
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\.github\workflows\ci.yml`

- [ ] **Step 1: Write the failing workflow checklist**

Required jobs:

- frontend install and build
- Rust fmt check
- Rust tests
- Rust clippy
- Tauri cargo check

- [ ] **Step 2: Verify workflow is absent**

Run: `Test-Path .github/workflows/ci.yml`
Expected: missing

- [ ] **Step 3: Add CI workflow**

Target runners:

- `windows-latest`
- `ubuntu-latest`

- [ ] **Step 4: Validate workflow syntax**

Run: `Get-Content .github/workflows/ci.yml`
Expected: workflow contains the required jobs and matrix

- [ ] **Step 5: Commit**

```bash
git add .github/workflows/ci.yml
git commit -m "ci: add github actions validation workflow"
```

### Task 12: Add GitHub Actions release workflow for Windows and Linux artifacts

**Files:**
- Create: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\.github\workflows\release.yml`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src-tauri\tauri.conf.json`

- [ ] **Step 1: Write the failing release checklist**

Required outputs:

- Windows executable or bundle
- Windows MSI
- Linux bundle artifact
- GitHub Release upload step

- [ ] **Step 2: Verify release workflow is absent**

Run: `Test-Path .github/workflows/release.yml`
Expected: missing

- [ ] **Step 3: Add release workflow**

Use:

- tag or manual dispatch trigger
- Tauri bundle build
- artifact upload and release publication

- [ ] **Step 4: Validate workflow content**

Run: `Get-Content .github/workflows/release.yml`
Expected: workflow contains release jobs and artifact upload steps

- [ ] **Step 5: Commit**

```bash
git add .github/workflows/release.yml src-tauri/tauri.conf.json
git commit -m "ci: add release workflow for windows and linux artifacts"
```

## Chunk 7: Verification and Handoff

### Task 13: Run final verification for V2 implementation readiness

**Files:**
- No file changes required unless fixes are needed

- [ ] **Step 1: Run Rust tests**

Run: `cargo test`
Expected: PASS

- [ ] **Step 2: Run frontend build**

Run: `npm run build`
Expected: PASS

- [ ] **Step 3: Run Tauri backend check**

Run: `cargo check --manifest-path src-tauri/Cargo.toml`
Expected: PASS

- [ ] **Step 4: Run lint checks**

Run: `cargo clippy --all-targets --all-features -- -D warnings`
Expected: PASS

- [ ] **Step 5: Run manual smoke verification**

Run: `npm run tauri dev`
Expected: app opens, refreshes data, shows icons, reacts to server/premium/budget inputs, and hotkey toggles visibility

## Notes for the Implementer

- Keep fee presets simple in V2. Do not build a custom fee editor yet.
- Budget should be a hard cap in the first implementation.
- If Linux packaging specifics vary by runner, prioritize a reliable downloadable artifact first over distro-perfect packaging.
- If pushing to GitHub is blocked by missing remote details, stop and ask for the repository URL rather than guessing.
- Preserve the existing Rust engine and extend it incrementally instead of rewriting the calculation core.
