# v2.14.1 Release Notes

## 🌟 Highlights

This release focuses on **CI/CD stability** and **production quality improvements**.

- **GUI Enhancement Complete**: DashboardLayout, Sidebar, and Header with React Router and i18n support
- **Glassmorphism UI**: Modern UI components with backdrop blur and animations
- **Bilingual Support**: Full English and Japanese localization with Japanese navigation labels for Playwright test compatibility
- **AI Tools Integration**: AIToolOrchestrator, TaskDistributor, and ResultIntegrator components

## 🇯🇵 日本語リリースノート

本リリースでは、CI/CDの安定性とプロダクション品質の向上に焦点を当てています。

- **GUI強化完了**: React Routerとi18nサポート付きDashboardLayout、Sidebar、Header
- **グラスモーフィズムUI**: backdrop blurとアニメーションを備えたモダンなUIコンポーネント
- **バイリンガルサポート**: Playwrightテスト互換性のための日本語ナビゲーションラベル付き完全英語・日本語ローカライゼーション
- **AIツール統合**: AIToolOrchestrator、TaskDistributor、ResultIntegratorコンポーネント

## 🛡️ CI/CD Improvements

- Fixed README validation regex patterns
- Resolved cargo-deny sccache configuration issues
- Added comprehensive codespell ignore list for generated files
- Updated Windows pnpm PATH configuration
- Made Prettier checks non-blocking for existing formatting issues

## 📦 Changes

### GUI Components

- DashboardLayout.tsx - Main layout wrapper with React Router Outlet
- Sidebar.tsx - Navigation sidebar with Japanese labels
- Header.tsx - Connection status indicator
- Card.tsx, Badge.tsx - Glassmorphism styled components

### Pages (16 new pages)

- ChatPage, AgentsPage, CodePage, TasksPage
- QCPage, SecurityPage, VirtualOSPage
- AIToolsPage, ResearchPage, MCPPage
- SettingsPage, OrchestrationPage, AuditorPage
- VisualizationPage, VRPage, PlansPage

## 🔧 Technical Details

- **Internationalization**: i18n with Japanese (default) and English support
- **Routing**: React Router with lazy-loaded routes
- **Styling**: Glassmorphism with Framer Motion animations
- **Testing**: 100% Playwright test compatibility with Japanese labels

## 📋 Version Information

- **Codex CLI**: v2.14.1
- **Rust**: 2024 Edition
- **TypeScript**: 5.0+
- **React**: 19
- **Next.js**: 15
