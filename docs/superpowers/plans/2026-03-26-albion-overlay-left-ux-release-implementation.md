# Albion Overlay Left UX + Release Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Corrigir versionamento e release, posicionar a overlay no lado esquerdo na primeira abertura e redesenhar a UI para o formato lateral equilibrado escolhido pelo usuario.

**Architecture:** Manter o motor Rust/Tauri atual, mas ajustar a configuracao de versao e bundle, adicionar persistencia simples de estado de janela via Tauri plugin/store, e reorganizar a UI vanilla para um layout vertical com ranking compacto e detalhe operacional expansivel. O trabalho fica dividido em camadas pequenas: metadados/release, comportamento da janela e redesign do frontend.

**Tech Stack:** Rust, Tauri v2, Vite, vanilla JavaScript, CSS, GitHub Actions

---

## File Structure

- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\package.json`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src-tauri\Cargo.toml`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src-tauri\tauri.conf.json`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src-tauri\src\main.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\ui\index.html`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\ui\main.js`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\ui\styles.css`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\.github\workflows\ci.yml`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\.github\workflows\release.yml`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\README.md`

## Chunk 1: Versioning and Workflow Maintenance

### Task 1: Align app version metadata to the release version

**Files:**
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\package.json`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src-tauri\Cargo.toml`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src-tauri\tauri.conf.json`

- [ ] **Step 1: Write a failing metadata checklist**

Confirm these three files still expose `0.1.0` instead of `0.1.1`.

- [ ] **Step 2: Verify current values**

Run: `Select-String -Path package.json,src-tauri\\Cargo.toml,src-tauri\\tauri.conf.json -Pattern '"version": "0.1.0"|version = "0.1.0"'`
Expected: matches found in all metadata files

- [ ] **Step 3: Update version strings**

Set the app version to `0.1.1` consistently in all three files.

- [ ] **Step 4: Verify metadata now matches**

Run: `Select-String -Path package.json,src-tauri\\Cargo.toml,src-tauri\\tauri.conf.json -Pattern '0.1.1'`
Expected: `0.1.1` present in all three files

- [ ] **Step 5: Commit**

```bash
git add package.json src-tauri/Cargo.toml src-tauri/tauri.conf.json
git commit -m "chore: align app version metadata"
```

### Task 2: Reduce future GitHub Actions runtime risk

**Files:**
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\.github\workflows\ci.yml`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\.github\workflows\release.yml`

- [ ] **Step 1: Write a failing workflow checklist**

Document that the current workflows still emit the Node 20 deprecation warning for marketplace actions.

- [ ] **Step 2: Inspect current workflow pins**

Run: `Get-Content .github\\workflows\\ci.yml; Get-Content .github\\workflows\\release.yml`
Expected: current action versions are visible for review

- [ ] **Step 3: Update workflow actions or environment for modern runner compatibility**

Prefer newer supported action versions if available. If version bumps alone are insufficient, add the appropriate workflow environment setting to opt into Node 24-compatible execution.

- [ ] **Step 4: Verify workflow structure**

Run: `Get-Content .github\\workflows\\ci.yml; Get-Content .github\\workflows\\release.yml`
Expected: updated actions or environment are present in both workflows

- [ ] **Step 5: Commit**

```bash
git add .github/workflows/ci.yml .github/workflows/release.yml
git commit -m "ci: modernize github actions runtime settings"
```

## Chunk 2: Left-Side Window Behavior

### Task 3: Open the overlay on the left side the first time and preserve later movement

**Files:**
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src-tauri\Cargo.toml`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src-tauri\src\main.rs`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\src-tauri\tauri.conf.json`

- [ ] **Step 1: Add a failing behavior checklist**

Document the desired behavior:
- first launch opens near the left side
- later launches reuse the user’s last position

- [ ] **Step 2: Verify current window config**

Run: `Get-Content src-tauri\\tauri.conf.json; Get-Content src-tauri\\src\\main.rs`
Expected: no explicit left-side placement memory exists

- [ ] **Step 3: Implement window state handling**

Add the minimal Tauri-side support needed to:
- size the window for left-side use
- position it on the left on first launch
- remember the later user position across launches

- [ ] **Step 4: Verify Tauri backend builds**

Run: `cargo check --manifest-path src-tauri/Cargo.toml`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src-tauri/Cargo.toml src-tauri/src/main.rs src-tauri/tauri.conf.json
git commit -m "feat: remember left-side overlay window placement"
```

## Chunk 3: Left Overlay UI Redesign

### Task 4: Reshape the UI into the chosen balanced left overlay

**Files:**
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\ui\index.html`
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\ui\styles.css`

- [ ] **Step 1: Write a failing UI checklist**

Checklist:
- compact header
- compressed controls area
- short ranking section at the top
- operational detail panel below
- better fit for left-side gameplay

- [ ] **Step 2: Confirm current layout is still wide**

Run: `Get-Content ui\\index.html; Get-Content ui\\styles.css`
Expected: layout is still wide and dashboard-like

- [ ] **Step 3: Implement structural HTML and CSS changes**

Rebuild the shell into a left overlay layout with:
- compact top bar
- two-row controls
- ranking panel above
- detail panel below
- stronger selected-state and visual grouping

- [ ] **Step 4: Verify production frontend build**

Run: `npm run build`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add ui/index.html ui/styles.css
git commit -m "feat: redesign overlay for left-side gameplay"
```

### Task 5: Make the interaction model fit the new operational layout

**Files:**
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\ui\main.js`

- [ ] **Step 1: Write a failing behavior checklist**

Confirm the new UI needs:
- compact ranking rows
- selected item detail in the lower section
- better operational summaries

- [ ] **Step 2: Inspect current rendering logic**

Run: `Get-Content ui\\main.js`
Expected: current rendering still matches the previous broader dashboard layout

- [ ] **Step 3: Update rendering logic**

Refactor the client code to support:
- shorter ranking cards
- stronger detail summary
- cleaner material rows
- layout-friendly text density

- [ ] **Step 4: Verify frontend build again**

Run: `npm run build`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add ui/main.js
git commit -m "feat: adapt overlay interactions to left-panel workflow"
```

## Chunk 4: Documentation and Final Verification

### Task 6: Refresh documentation for the new release/layout behavior

**Files:**
- Modify: `C:\Users\clebs\Desktop\Rust\orbita-rs\albion\README.md`

- [ ] **Step 1: Write a failing docs checklist**

Document missing items:
- new release version alignment expectations
- left-side first-launch behavior
- redesigned overlay use

- [ ] **Step 2: Update README**

Add the new behavior and release notes succinctly.

- [ ] **Step 3: Verify README content**

Run: `Get-Content README.md`
Expected: new left-overlay and release/version details are present

- [ ] **Step 4: Commit**

```bash
git add README.md
git commit -m "docs: update overlay layout and release notes"
```

### Task 7: Run final verification

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

- [ ] **Step 4: Run lint**

Run: `cargo clippy --all-targets --all-features -- -D warnings`
Expected: PASS

- [ ] **Step 5: Run manual smoke**

Run: `npm run tauri dev`
Expected: app opens in left-side format, remains usable, and can still refresh data
