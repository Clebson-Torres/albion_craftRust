# Albion Crafting Overlay V2 Design

## Overview

This V2 extends the existing crafting overlay into an operational decision tool for real in-game execution. The focus is no longer only identifying profitable bags and capes, but ranking opportunities according to the player's actual conditions:

- current server
- current city
- sourcing strategy
- premium status
- transport penalty
- available budget

The overlay should answer not only "what is profitable?" but also "what can I actually afford and execute right now?".

## Product Goal

Turn the current overlay into a market operations assistant that helps the player:

- choose the correct Albion server
- account for real buying and selling costs
- understand how many units can be crafted with current capital
- compare opportunities by total profit with the available budget
- buy materials faster using item icons and clearer breakdowns

## Scope

## In Scope for V2

- selectable server
- premium on or off preference
- configurable investment budget
- quantity craftable from budget
- total profit based on available budget
- buy and sell fee handling
- materials list with item icons
- richer opportunity detail panel
- ranking options centered on practical execution
- initialize and structure the project as a git repository
- prepare GitHub repository workflow
- CI validation for Rust, frontend, and Tauri build checks
- CD pipeline to publish Windows and Linux artifacts

## Out of Scope for V2

- automatic in-game city detection
- reading game state from the client
- route planner with exact pathing
- item liquidity forecasting beyond current heuristics
- automated shopping or trading actions
- custom fee editor with advanced taxation rules

## Delivery and Repository Goal

V2 should leave the project ready not only for local use, but also for repeatable delivery and collaboration. That means the codebase should be organized as a proper git repository, prepared for GitHub hosting, and able to produce release artifacts automatically through CI/CD.

## Key Product Decisions

- budget should be used both to filter what is feasible and to rank by total possible return
- premium is represented by a simple on or off toggle in V2
- server is a top-level preference because it changes the entire market context
- material icons should appear in the detail panel first, not only the item card
- the main list should remain compact even as more financial fields are added

## User Experience

### Header Controls

The top controls area should include:

- server
- current city
- sourcing strategy
- transport cost per unit
- premium on or off
- available budget

Changing any of these values should trigger a refresh and rerank opportunities.

### Main List

Each ranked opportunity should show:

- item icon
- item name
- best sale destination
- craft cost per unit
- net profit per unit
- quantity craftable with current budget
- total possible net profit

This list should optimize for quick scanning and comparison.

### Detail Panel

The selected opportunity should show:

- large item icon
- best sale destination
- craft cost per unit
- net profit per unit
- quantity craftable
- total budget consumed
- remaining budget
- total net profit
- fee summary
- confidence
- observed timestamp

Material breakdown should include:

- material icon
- material id or name
- quantity required per craft
- source city
- unit price
- total material cost contribution

## Calculation Model

For each opportunity, the engine should compute in this order:

1. base material cost
2. transport penalty
3. effective buy-side acquisition cost
4. effective sell-side revenue after fees
5. total cost per craft
6. net profit per craft
7. maximum craftable quantity within budget
8. total net profit within budget
9. score used for ranking

## Budget Rules

Budget should be treated as a hard cap for material acquisition.

For each opportunity:

- calculate the full cost per craft
- compute how many whole crafts fit in the budget
- compute the capital used
- compute remaining budget
- compute total net profit from the maximum number of crafts

This allows the ranking to answer:

- what fits in the current budget
- what yields the best total return right now

## Premium and Fee Handling

V2 should start with a simple premium toggle:

- premium on
- premium off

Each mode applies predefined fee assumptions for buying and selling. The first implementation should prefer simple preset constants rather than a customizable fee editor.

Required outputs:

- buy fee estimate
- sell fee estimate
- total fee impact
- net profit after fees

## Server Handling

The app should support switching among:

- west
- east
- europe

Changing server should switch the API base host and refresh the entire dataset.

## Icon Handling

Icons should be rendered using item-id-based URLs derived from the Albion render endpoint. This applies to:

- primary item icons in the ranking list
- large selected-item icon in the detail panel
- ingredient icons in the materials list

The design should allow future local caching, but V2 can start with remote URL rendering.

## Ranking Strategy

The ranking should support practical sorting modes:

- total net profit with budget
- net profit per unit
- return on capital

Default recommendation:

- rank by total net profit with budget

This better matches actual market action than raw unit margin alone.

## Architecture

The current Rust engine remains the source of truth for:

- recipes
- sourcing logic
- transport handling
- fee handling
- budget handling
- ranking

The Tauri frontend remains responsible for:

- controls
- rendering
- user interaction
- selection state
- item and material icon display

The delivery layer should include:

- git repository initialization and ignore rules
- GitHub Actions workflows
- release packaging for Tauri artifacts
- artifact publication strategy for Windows and Linux

To support this cleanly, the backend should introduce an explicit operational preferences object containing:

- server
- current city
- sourcing strategy
- transport cost per unit
- premium status
- budget

## Data Contract Changes

The Tauri payload should be expanded so each opportunity includes:

- item id
- craft cost per unit
- sell price
- sell target
- net profit per unit
- quantity craftable
- budget used
- budget remaining
- total net profit
- buy fee total
- sell fee total
- total fee impact
- ingredients with icon-compatible ids and full cost lines

## Git and GitHub Setup

The project should be prepared for source control and hosted collaboration.

Required setup:

- initialize git if the workspace is not yet a repository
- add a `.gitignore` suited for Rust, Node, Vite, Tauri, and local Codex output
- create a clean initial commit history baseline
- prepare the repository for GitHub push
- document how local development and release workflows map to the repository

## CI Strategy

GitHub Actions should validate every relevant layer of the app.

Required checks:

- Rust formatting
- Rust tests
- Rust linting
- frontend install and build
- Tauri backend compile check

Recommended matrix targets:

- Windows
- Linux

The CI pipeline should fail clearly when any layer breaks.

## CD Strategy

The project should produce distributable artifacts for:

- Windows executable
- Windows MSI installer
- Linux package or bundled binary artifact

Recommended release behavior:

- trigger on version tag or manual workflow dispatch
- build Tauri release bundles on GitHub Actions
- upload artifacts to GitHub Releases
- keep release naming predictable and versioned

The first implementation does not need full auto-update distribution, but it should reliably generate downloadable artifacts.

## Testing Strategy

### Engine Tests

- changing server changes API base source
- premium mode changes fee totals
- budget limits quantity craftable
- total net profit changes with budget
- transport penalty affects remote sourcing
- ranking changes when sorting by budget-based return

### Serialization Tests

- payload includes all new fields in frontend-friendly naming

### UI Verification

- server selector refreshes results
- premium toggle updates values
- budget change updates quantity and total profit
- material icons render in detail panel
- selected opportunity updates correctly after rerank

### CI/CD Verification

- repository can run checks from a fresh clone
- GitHub Actions passes on supported runners
- release workflow produces Windows and Linux artifacts
- artifact names and locations are documented

## UX Risks

- too many metrics can overwhelm the compact layout
- budget-based ranking may confuse users if unit profit is hidden
- fee presets must be visible enough to build trust

Mitigations:

- keep the list compact and move the full breakdown into the detail panel
- expose both unit profit and total profit
- show fee summary clearly in the detail section

## Recommended Implementation Order

1. add operational preferences model
2. add server selection support
3. add premium fee presets and net-fee calculations
4. add budget calculations
5. add total-profit ranking fields
6. add ingredient icons and richer detail panel
7. add sorting modes
8. initialize git and repository metadata
9. add GitHub Actions CI
10. add GitHub release workflow for Windows and Linux artifacts

## Decisions Made

- V2 is an operational extension of the current app, not a full rewrite of the engine
- budget should support both feasibility and ranking
- server selection is required
- premium is a binary toggle in V2
- materials should include icons
- ranking should prioritize total profit with available budget by default
- the project should be ready for GitHub-based CI/CD and packaged releases
