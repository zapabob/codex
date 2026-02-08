# Handover Meta-Prompt: GUI Enhancement & Production Readiness

## Context & Objectives
You are taking over a project focused on the **Codex** ecosystem. The immediate priority is to improve the GUI and prepare it for production-level implementation.

### Current State
1. **Dual GUI Environments**:
    - **`gui/` (Legacy/Next.js)**: A comprehensive but slow Next.js application. Uses `pnpm` for dependencies. Contains the existing `gui-tests` suite (Playwright).
    - **`codex-gui-x/` (New/Vite)**: A high-performance Vite-based React application. This is the preferred direction for future development but currently lacks full test coverage.
2. **Recent Fixes**:
    - Cleaned `config.toml` (removed invalid/experimental keys to align with strict schema).
    - Fixed `Git4DVisualization.tsx` (Vite) TypeScript errors (hoisting, `useRef` init) and added error display.
    - Updated `issue-labeler.yml` for correct secret handling.
    - Successfully built and installed `codex 2.14.1` using 12-core parallel build.

### GUI Test Suite Findings (`gui-tests`)
- **Results**: 6 Pass / 19 Fail.
- **Critical Issues**: Persistent timeouts due to slow hydration in the Next.js `gui` app. Mismatched selectors for Japanese labels (e.g., `エージェント`).
- **Dependency Note**: The `gui/` directory **MUST** use `pnpm` to avoid dependency conflicts encountered with standard `npm`.

## Priority Tasks for the Next Agent

### 1. GUI Consolidation & Optimization
- **Goal**: Move features from `gui/` (Next.js) to `codex-gui-x/` (Vite) to leverage the faster build system and cleaner architecture.
- **Focus**: Port the `DashboardLayout`, `Sidebar`, and specialized views (QC, Security, Tasks) to the Vite environment.
- **Refinement**: Implement "Production Grade" UI/UX. Use the rich aesthetics guidelines (glassmorphism, micro-animations, curated palettes) to ensure a premium feel.

### 2. Test Alignment
- **Goal**: Align `gui-tests` with the actual UI state.
- **Action**: Update Playwright selectors in `gui-tests/tests/gui-basic.spec.js` to match the current DOM structure.
- **Target**: Eventually point the tests to `codex-gui-x` once the features are migrated.

### 3. Production Readiness
- **Binary Stability**: Ensure the `codex` command remains functional and correctly handles the cleaned `config.toml`.
- **Error Handling**: Expand the visible error reporting (like the one added to `Git4DVisualization.tsx`) across all major UI components.
- **Verification**: Run `npm run typecheck` in the target GUI project and ensure zero errors.

## Instructions
1. **Read these artifacts first**:
    - `_docs/2026-02-08_codebase-cleanup.md`
    - `C:\Users\downl\.gemini\antigravity\brain\46dc582d-3238-41c2-b089-1ce9f35b5ae5\walkthrough-gui-tests.md`
2. **Standard Workflow**:
    - Create an implementation plan before major migrations.
    - Use `pnpm` for the legacy `gui` and `npm` for `codex-gui-x`.
    - Maintain the Implementation Log in `_docs/` following the `yyyy-mm-dd_feature.md` format.

Good luck.
