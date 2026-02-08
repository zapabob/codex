import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import LanguageDetector from "i18next-browser-languagedetector";

const resources = {
  ja: {
    translation: {
      nav: {
        dashboard: "ダッシュボード",
        agents: "エージェント",
        code: "コード実行",
        tasks: "タスク管理",
        qc: "QC管理",
        security: "セキュリティ",
        virtualOs: "仮想OS",
        aiTools: "AIツール統合",
        research: "Deep Research",
        webResearch: "Web Research",
        mcp: "MCPサーバー",
        settings: "設定",
        analytics: "分析",
        docs: "ドキュメント",
        performance: "パフォーマンス",
      },
      app: {
        title: "Codex Control",
        subtitle: "AI Assistant Platform",
        welcome: "ようこそ、Codexへ",
      },
      common: {
        loading: "読み込み中...",
        error: "エラーが発生しました",
        retry: "再試行",
        cancel: "キャンセル",
        save: "保存",
        delete: "削除",
        edit: "編集",
        create: "作成",
        close: "閉じる",
        confirm: "確認",
        search: "検索",
        filter: "フィルター",
      },
    },
  },
  en: {
    translation: {
      nav: {
        dashboard: "Dashboard",
        agents: "Agents",
        code: "Code Execution",
        tasks: "Task Management",
        qc: "QC Management",
        security: "Security",
        virtualOs: "Virtual OS",
        aiTools: "AI Tools",
        research: "Deep Research",
        webResearch: "Web Research",
        mcp: "MCP Server",
        settings: "Settings",
        analytics: "Analytics",
        docs: "Documentation",
        performance: "Performance",
      },
      app: {
        title: "Codex Control",
        subtitle: "AI Assistant Platform",
        welcome: "Welcome to Codex",
      },
      common: {
        loading: "Loading...",
        error: "An error occurred",
        retry: "Retry",
        cancel: "Cancel",
        save: "Save",
        delete: "Delete",
        edit: "Edit",
        create: "Create",
        close: "Close",
        confirm: "Confirm",
        search: "Search",
        filter: "Filter",
      },
    },
  },
};

i18n
  .use(LanguageDetector)
  .use(initReactI18next)
  .init({
    resources,
    fallbackLng: "ja",
    interpolation: {
      escapeValue: false,
    },
    detection: {
      order: ["localStorage", "navigator"],
      caches: ["localStorage"],
    },
  });

export default i18n;
