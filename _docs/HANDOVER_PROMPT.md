# Project Handover: GUI Integration & Migration

## Current Objective
Migrate legacy GUI features (Visualization, VR, Plans, Virtual OS) from the legacy `gui` (Next.js) project to the new `codex-gui-x` (Vite) project.

## Current Status (2026-02-07)
- **Component Migration**: All core components from `gui/src/components` have been moved to `codex-gui-x/src/components`.
- **Backend Sync**: Resolved `schemars` trait bound errors in `codex-rs/core/src/config/mod.rs` to allow clean back-and-forth between TOML and JSON.
- **Frontend Integration**: Updated `App.tsx` with navigation for the new views.
- **Refactoring Progress**:
  - `PlanCreator.tsx`: Fixed API client instantiation, imports, and property mapping (snake_case from API to local camelCase).
  - `VirtualEnvironmentManager.tsx`: Standardized MUI props (`small` vs `sm`, `outlined` vs `outline`), resolved React purity issues (`Date.now()` during render).
  - `Git4DVisualization.tsx`: Updated Three.js type definitions and resolved hook dependency warnings.
- **Build Status**: `npm run build` currently reports **344 errors**. These are mostly concentrated in:
  - WebXR API types (missing `@types/webxr` or explicit definitions).
  - Property mismatches in MUI components that were slightly different between Next.js and Vite environments.
  - Remaining snake_case vs camelCase mapping issues in some components.

## Recent Changes & Important Files
- `codex-gui-x/src/components/plan/PlanCreator.tsx`: Major clean-up.
- `codex-gui-x/src/components/virtual-os/VirtualEnvironmentManager.tsx`: MUI standardization.
- `codex-gui-x/src/components/visualization/Git4DVisualization.tsx`: Three.js integration fix.
- `codex-gui-x/src/lib/types/virtual-os.ts`: Shared types for the Virtual OS module.
- `codex-gui-x/src/mui/Grid2.tsx`: Recreated MUI Grid2 component for Vite compatibility.

## Pending Tasks (Next Steps)
1. **Resolve Build Errors**:
   - Install `@types/webxr` or provide global type definitions for `XR` types used in `webxr-manager.ts`.
   - Fix remaining `createdAt` vs `created_at` in rendering sections of `PlanCreator.tsx`.
   - Double-check all MUI `Button` and `Select` components for prop compatibility.
2. **WebXR Support**: Ensure hand tracking and VR scenes correctly initialize with `@react-three/xr`.
3. **Manual Verification**: Run `npm run dev` and navigate through all new views to ensure no runtime crashes.

## Environment Notes
- Vite environment variables use `import.meta.env.VITE_API_URL` instead of `process.env.NEXT_PUBLIC_API_URL`.
- Path aliases are configured for `@/` pointing to `src/`.

## Artifacts
- `task.md`: Current task tracking.
- `implementation_plan.md`: Initial plan for migration.
- `walkthrough.md`: Proof of work (so far).
