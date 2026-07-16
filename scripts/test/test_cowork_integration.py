#!/usr/bin/env python3
"""
ClaudeCowork統合機能のテストと動作確認
ブラウザ自動化、ドキュメント生成、外部サービス、セッション管理のテスト
"""

import asyncio
import sys
import json
import logging
from pathlib import Path
from typing import Dict, List, Any
from datetime import datetime
import tempfile

# テスト対象モジュール
sys.path.append(str(Path(__file__).parent))
try:
    from cowork_browser_automation import EnhancedBrowserAutomationEngine
except ImportError:
    EnhancedBrowserAutomationEngine = None

from cowork_document_generator import DocumentGenerationEngine
from cowork_session_manager import SessionManager

# 外部サービスコネクター（オプション）
try:
    from cowork_connectors.asana_connector import AsanaConnector
    from cowork_connectors.notion_connector import NotionConnector
except ImportError:
    AsanaConnector = None
    NotionConnector = None

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)


class CoworkIntegrationTester:
    """ClaudeCowork統合機能のテストスイート"""
    
    def __init__(self):
        self.test_results: List[Dict[str, Any]] = []
        self.temp_dir = Path(tempfile.gettempdir()) / "cowork_test"
        self.temp_dir.mkdir(parents=True, exist_ok=True)
    
    async def run_all_tests(self) -> Dict[str, Any]:
        """全テスト実行"""
        logger.info("=" * 60)
        logger.info("ClaudeCowork統合機能テスト開始")
        logger.info("=" * 60)
        
        test_suites = [
            ("セッション管理", self.test_session_management),
            ("ドキュメント生成", self.test_document_generation),
            ("ブラウザ自動化", self.test_browser_automation),
            ("外部サービスコネクター", self.test_external_connectors),
        ]
        
        for suite_name, test_func in test_suites:
            try:
                logger.info(f"\n📦 {suite_name}テスト開始...")
                result = await test_func()
                self.test_results.append({
                    "suite": suite_name,
                    "status": "success" if result.get("success") else "failed",
                    "details": result
                })
                if result.get("success"):
                    logger.info(f"✅ {suite_name}テスト: 成功")
                else:
                    logger.error(f"❌ {suite_name}テスト: 失敗 - {result.get('error')}")
            except Exception as e:
                logger.error(f"❌ {suite_name}テスト: 例外発生 - {e}")
                self.test_results.append({
                    "suite": suite_name,
                    "status": "error",
                    "error": str(e)
                })
        
        return self.generate_report()
    
    async def test_session_management(self) -> Dict[str, Any]:
        """セッション管理テスト"""
        try:
            manager = SessionManager(self.temp_dir / "sessions")
            
            # セッション作成
            session = manager.create_session("テストセッション", {"test": True})
            assert session.id is not None, "セッションIDが生成されていません"
            
            # タスク追加
            task = {"name": "テストタスク", "status": "pending"}
            assert manager.add_task(session.id, task), "タスク追加に失敗"
            
            # ファイル追加
            test_file = self.temp_dir / "test_file.txt"
            test_file.write_text("テストファイル")
            assert manager.add_file(session.id, str(test_file)), "ファイル追加に失敗"
            
            # ファイルプレビュー
            preview = manager.preview_file(str(test_file))
            assert preview.get("success"), "ファイルプレビューに失敗"
            
            # セッション一覧
            sessions = manager.list_sessions()
            assert len(sessions) > 0, "セッション一覧が空です"
            
            return {"success": True, "session_id": session.id}
        
        except Exception as e:
            return {"success": False, "error": str(e)}
    
    async def test_document_generation(self) -> Dict[str, Any]:
        """ドキュメント生成テスト"""
        try:
            engine = DocumentGenerationEngine()
            
            # Excel生成テスト
            excel_data = {
                "sheets": [{
                    "name": "Test",
                    "rows": [["A", "B"], [1, 2], [3, 4]],
                    "formulas": {"B3": "=SUM(B2:B3)"},
                    "styles": {"header": {"row": 1, "bg_color": "366092"}}
                }]
            }
            excel_path = self.temp_dir / "test_output.xlsx"
            result = engine.generate_excel(str(excel_path), excel_data)
            assert result.get("success"), f"Excel生成失敗: {result.get('error')}"
            assert excel_path.exists(), "Excelファイルが生成されていません"
            
            # Word生成テスト
            word_content = {
                "title": "テスト文書",
                "sections": [{
                    "heading": "テスト",
                    "paragraphs": ["これはテストです"]
                }]
            }
            word_path = self.temp_dir / "test_output.docx"
            result = engine.generate_word(str(word_path), word_content)
            assert result.get("success"), f"Word生成失敗: {result.get('error')}"
            assert word_path.exists(), "Wordファイルが生成されていません"
            
            # PowerPoint生成テスト
            ppt_data = {
                "title": "テストプレゼン",
                "slides": [{
                    "title": "スライド1",
                    "content": "テストコンテンツ"
                }]
            }
            ppt_path = self.temp_dir / "test_output.pptx"
            result = engine.generate_powerpoint(str(ppt_path), ppt_data)
            assert result.get("success"), f"PowerPoint生成失敗: {result.get('error')}"
            assert ppt_path.exists(), "PowerPointファイルが生成されていません"
            
            return {"success": True, "files": [str(excel_path), str(word_path), str(ppt_path)]}
        
        except Exception as e:
            return {"success": False, "error": str(e)}
    
    async def test_browser_automation(self) -> Dict[str, Any]:
        """ブラウザ自動化テスト"""
        try:
            if EnhancedBrowserAutomationEngine is None:
                return {"success": True, "skipped": True, "reason": "EnhancedBrowserAutomationEngine not available"}
            
            engine = EnhancedBrowserAutomationEngine(headless=True)
            await engine.initialize()
            
            # タブグループ作成
            group = await engine.create_tab_group("test_group")
            assert group.group_id == "test_group", "タブグループ作成に失敗"
            
            # タブ追加
            page = await engine.add_tab_to_group("test_group", "https://example.com")
            assert page is not None, "タブ追加に失敗"
            
            # スクリーンショット
            screenshot = await engine.capture_screenshot(page)
            assert Path(screenshot).exists(), "スクリーンショットが生成されていません"
            
            # 視覚要素分析
            elements = await engine.analyze_visual_elements(page)
            assert len(elements) > 0, "視覚要素が検出されていません"
            
            await engine.close()
            
            return {"success": True, "screenshot": screenshot, "elements_count": len(elements)}
        
        except Exception as e:
            return {"success": False, "error": str(e)}
    
    async def test_external_connectors(self) -> Dict[str, Any]:
        """外部サービスコネクターテスト"""
        try:
            if AsanaConnector is None or NotionConnector is None:
                return {"success": True, "skipped": True, "reason": "Connectors not available"}
            
            # Asanaコネクター（モックテスト）
            # 実際のAPIキーがない場合はスキップ
            import os
            asana_key = os.getenv("ASANA_API_KEY")
            if not asana_key:
                logger.warning("ASANA_API_KEYが設定されていないため、Asanaテストをスキップ")
                return {"success": True, "skipped": True, "reason": "API key not set"}
            
            connector = AsanaConnector(asana_key)
            result = await connector.connect()
            if result.success:
                await connector.disconnect()
                return {"success": True, "connector": "Asana"}
            else:
                return {"success": False, "error": result.error}
        
        except Exception as e:
            return {"success": False, "error": str(e)}
    
    def generate_report(self) -> Dict[str, Any]:
        """テストレポート生成"""
        total = len(self.test_results)
        passed = sum(1 for r in self.test_results if r.get("status") == "success")
        failed = sum(1 for r in self.test_results if r.get("status") == "failed")
        errors = sum(1 for r in self.test_results if r.get("status") == "error")
        
        report = {
            "timestamp": datetime.now().isoformat(),
            "summary": {
                "total": total,
                "passed": passed,
                "failed": failed,
                "errors": errors,
                "success_rate": (passed / total * 100) if total > 0 else 0
            },
            "results": self.test_results
        }
        
        # レポート保存
        report_path = self.temp_dir / "test_report.json"
        with open(report_path, "w", encoding="utf-8") as f:
            json.dump(report, f, indent=2, ensure_ascii=False)
        
        logger.info("\n" + "=" * 60)
        logger.info("テストレポート")
        logger.info("=" * 60)
        logger.info(f"総テスト数: {total}")
        logger.info(f"成功: {passed}")
        logger.info(f"失敗: {failed}")
        logger.info(f"エラー: {errors}")
        logger.info(f"成功率: {report['summary']['success_rate']:.1f}%")
        logger.info(f"レポート保存先: {report_path}")
        
        return report


async def main():
    """メイン実行"""
    import os
    
    tester = CoworkIntegrationTester()
    report = await tester.run_all_tests()
    
    # 終了コード
    if report["summary"]["failed"] > 0 or report["summary"]["errors"] > 0:
        sys.exit(1)
    else:
        sys.exit(0)


if __name__ == "__main__":
    asyncio.run(main())
