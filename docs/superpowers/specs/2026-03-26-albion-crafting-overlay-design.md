# Albion Crafting Overlay Design

## Overview

This project is a compact desktop overlay for Albion Online focused on personal market profit through crafting. The first version targets bags and capes only, opens via hotkey, and shows a ranked list of the best crafting opportunities available at the moment.

The overlay compares:

- crafting cost from market-bought materials
- sale opportunities across regular city markets
- sale opportunities in the Black Market

The goal is to let the user make a fast decision in-game without tabbing through spreadsheets or multiple browser pages.

## Product Goal

Deliver a compact, low-friction crafting assistant that answers:

- what bag or cape is worth crafting right now
- where materials are cheapest
- where the crafted item should be sold
- whether the opportunity is trustworthy enough to act on

## Users

Primary user:

- solo or small-scale Albion crafter focused on personal profit

Usage context:

- user is actively in-game
- user wants a quick top list instead of a full market analysis suite
- user values speed, clarity, and confidence markers

## Scope

## In Scope for V1

- compact overlay-like desktop window
- global hotkey to show and hide the window
- support for bags and capes
- ranked top opportunities list
- comparison between city markets and Black Market
- detail panel for selected opportunity
- refresh loop based on Albion data API
- confidence indicator for each opportunity
- stale-data handling

## Out of Scope for V1

- direct game memory reading or client injection
- automatic detection of current city
- all item families
- advanced user-configurable filters
- true volume or turnover ranking from first-party transaction counts
- automated buy or sell actions

## Future Scope

- user-selected current city filter
- more item families
- tier and enchantment filters
- configurable profit floor
- custom transport assumptions
- favorite items and watchlists

## Roadmap Note

The first follow-up after V1 should be a current-city selector so the ranking can answer location-aware questions such as which opportunities make sense from the city where the player is currently crafting or buying inputs.

## Recommended Approach

Build the system as two clearly separated parts:

1. calculation engine
2. overlay UI shell

The calculation engine is responsible for API collection, normalization, recipe cost calculation, opportunity scoring, and ranking. The UI shell is responsible for hotkey behavior, window presentation, interaction, and rendering the current ranking.

This approach keeps domain logic testable and reusable while making it easy to evolve the interface later.

## Technology Direction

Recommended first implementation:

- Rust application
- overlay-style desktop window
- likely `egui` for a simple native first version, or `Tauri` if stronger UI polish becomes a priority

Recommendation:

- start with `egui` for V1 because it is simpler for a compact always-on-top utility and keeps the stack mostly Rust-native

## Data Source Assumptions

The project will use the Albion Online Data API as its market data source. The public API provides current price snapshots and history endpoints, but it does not expose a direct "most sold items" or "highest turnover" ranking suitable for a real top-opportunities query.

Because of that, the system will infer opportunity quality through heuristics rather than raw market volume.

## Core User Experience

### Main Flow

1. user presses a hotkey
2. compact window appears above the game
3. user sees a top list of current bag and cape crafting opportunities
4. user selects an item from the list
5. detail panel shows why it ranks well
6. user closes the overlay and acts in-game

### Main Screen

The overlay should default to a `Top 10 opportunities now` list. Each row should show:

- item name
- estimated craft cost
- best destination to sell
- estimated net profit
- confidence indicator

### Detail Panel

When the user selects an item, the detail panel should show:

- required materials
- total craft cost
- cheapest known source city for materials
- best sale target
- comparison between city market and Black Market result
- data recency
- confidence explanation

## System Design

### 1. Data Fetch Layer

Responsibilities:

- query Albion API endpoints for relevant item ids
- collect pricing data for bags, capes, and their required materials
- collect price history where needed for confidence heuristics
- normalize server, city, quality, and timestamp data

Design notes:

- polling interval should be modest, around 30 to 60 seconds
- requests should be batched where possible
- stale or missing responses should not break the UI

### 2. Recipe and Cost Layer

Responsibilities:

- define recipes for supported bags and capes
- calculate material requirements
- compute total craft cost from market prices
- identify cheapest input city when possible

Design notes:

- initial recipe support can be coded from a curated internal item set
- design should allow recipes to be extended later without rewriting ranking logic

### 3. Opportunity Engine

Responsibilities:

- compare craft cost against available sale prices
- compute gross and net profit
- compare city-market sale vs Black Market sale
- generate final ranked opportunity list

Outputs per opportunity:

- item id
- item name
- craft cost
- best sell target
- sell price
- gross profit
- net profit
- confidence score
- freshness metadata

### 4. Confidence Scoring

Because turnover is not directly available, the engine should create a confidence score from practical signals such as:

- data freshness
- history coverage presence
- stability of recent price range
- size of margin relative to cost
- completeness of required market data

The score should be used to deprioritize suspicious opportunities with incomplete or old data.

### 5. Overlay UI

Responsibilities:

- register a global hotkey
- open and hide a compact always-on-top window
- render ranking list and selected-item details
- preserve the last successful result set when refresh fails

UX requirements:

- small footprint
- readable over the game
- keyboard-friendly close/open loop
- no unnecessary visual clutter

## Error Handling

The overlay must remain usable even when the API is incomplete or slow.

Required behaviors:

- keep showing the last valid ranking during refresh failures
- mark stale data clearly
- omit opportunities with incomplete craft calculations
- show loading and degraded-state messages without blocking interaction

## Testing Strategy

### Unit Tests

- recipe cost calculation
- profit calculation
- best-destination comparison
- confidence scoring
- ranking order

### Integration Tests

- API response normalization
- handling of missing price fields
- stale timestamp handling
- fallback behavior when Black Market data is absent

### UI Verification

- hotkey opens and hides the window
- ranking list renders correctly
- selecting an item updates details
- stale-data labels appear when expected

## Delivery Plan Shape

Suggested implementation phases:

1. item and recipe modeling for bags and capes
2. Albion API client and normalization
3. craft cost and profit engine
4. opportunity ranking and confidence score
5. compact overlay UI with hotkey
6. resilience and test coverage

## Decisions Made

- focus on personal profit, not guild logistics
- first item family is bags and capes
- interface is a compact hotkey-driven overlay-style window
- primary view is a ranked opportunities list
- detail view explains the selected opportunity
- ranking compares city markets and Black Market
- future roadmap should include user-selected current city

## Open Questions for Planning

- exact list of supported bag and cape variants for V1
- whether profit should account for configurable tax assumptions in V1 or use fixed defaults
- whether the first server target is a single Albion region or multiple selectable regions
