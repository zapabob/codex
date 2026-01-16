#!/usr/bin/env python3
"""
Enhanced Research Agent - ClaudeCowork統合版
Web Search Deepresearch + Cowork機能統合エージェント
"""

import asyncio
import json
import logging
import sys
import time
from pathlib import Path
from typing import Dict, List, Any, Optional, Union
from datetime import datetime

# プロジェクト内モジュール
try:
    from scripts.cowork_resident_agent import ResidentAgent
    from scripts.cowork_productivity_assistant import CoworkProductivityAssistant
    from scripts.cowork_apple_gui import AppleStyleGUI
    from scripts.cowork_feature_search import CoworkFeatureSearch
    from scripts.run_deep_research import DeepresearchRunner
    from multi_model_intelligence import MultiModelIntelligence
except ImportError:
    # フォールバック
    print("Warning: Some modules not found, using fallback implementation")

    class ResidentAgent:
        async def submit_task(self, task): return f"task_{int(time.time())}"

    class CoworkProductivityAssistant:
        async def execute_task(self, task): return {"success": True, "result": "Mock result"}

    class AppleStyleGUI:
        def run(self): pass

    class CoworkFeatureSearch:
        def search_features(self, query): return []

    class DeepresearchRunner:
        async def run_research_topic(self, args): return {"success": True}

    class MultiModelIntelligence:
        def __init__(self): pass
        async def select_model(self, task, context=None):
            return type('obj', (object,), {"primary_model": type('obj', (object,), {"model_name": "mock"})()})()


class SmartSuggestions:
    """スマートサジェスチョン"""

    def __init__(self):
        self.suggestion_history = []

    async def generate_suggestions(self, task: str, analysis: Dict[str, Any], context: Dict[str, Any]) -> List[Dict[str, Any]]:
        """スマートサジェスチョン生成"""
        suggestions = []

        # タスクタイプに基づくサジェスチョン
        if analysis.get("is_research_task"):
            suggestions.append({
                "type": "follow_up",
                "title": "Related Topic Research",
                "description": "Research additional topics related to current findings"
            })

        if analysis.get("is_cowork_task"):
            suggestions.append({
                "type": "automation",
                "title": "Set Regular Execution",
                "description": "Configure this task to run regularly"
            })

        return suggestions


class ProgressVisualizer:
    """プログレスビジュアライザー"""

    def __init__(self):
        self.progress_tracks = {}

    async def create_progress_dashboard(self, session_id: str) -> Dict[str, Any]:
        """プログレスダッシュボード作成"""
        return {
            "session_id": session_id,
            "progress": 0.5,
            "status": "running",
            "visualization": "progress_bar"
        }


class AdaptiveLearningSystem:
    """適応学習システム"""

    def __init__(self):
        self.learning_patterns = {}

    async def learn_from_execution(self, task: str, analysis: Dict[str, Any], result: Dict[str, Any], enhanced_result: Dict[str, Any]):
        """実行からの学習"""
        # 学習データ蓄積（簡易版）
        pattern_key = f"{analysis.get('complexity', 'low')}_{analysis.get('is_research_task', False)}_{analysis.get('is_cowork_task', False)}"
        if pattern_key not in self.learning_patterns:
            self.learning_patterns[pattern_key] = []

        self.learning_patterns[pattern_key].append({
            "task_length": len(task.split()),
            "success": result.get("success", False),
            "execution_time": result.get("execution_time", 0),
            "quality_score": enhanced_result.get("quality_score", 0.5)
        })


class EnhancedGUIBridge:
    """拡張GUIブリッジ"""

    def __init__(self, agent):
        self.agent = agent
        self.gui_instance = None

    async def initialize_gui(self):
        """GUI初期化"""
        try:
            self.gui_instance = AppleStyleGUI()
            await self.integrate_deepresearch_features()
        except Exception as e:
            print(f"GUI initialization error: {e}")

    async def integrate_deepresearch_features(self):
        """Deepresearch機能統合"""
        if self.gui_instance:
            self.gui_instance.execute_enhanced_task = self.agent.execute_enhanced_task
            self.gui_instance.generate_smart_suggestions = self.agent.smart_suggestions.generate_suggestions

    async def show_enhanced_interface(self):
        """拡張インターフェース表示"""
        if self.gui_instance:
            self.gui_instance.run()


class EnhancedResearchAgent:
    """
    ClaudeCowork統合版Enhanced Research Agent
    Deepresearch + Cowork機能を統合した次世代エージェント
    """

    def __init__(self):
        self.logger = logging.getLogger("EnhancedResearchAgent")

        # コアコンポーネント初期化
        self.resident_agent = ResidentAgent()
        self.productivity_assistant = CoworkProductivityAssistant()
        self.feature_search = CoworkFeatureSearch()
        self.deep_research = DeepresearchRunner()
        self.multi_model_intelligence = MultiModelIntelligence()

        # 統合機能
        self.smart_suggestions = SmartSuggestions()
        self.progress_visualizer = ProgressVisualizer()
        self.learning_system = AdaptiveLearningSystem()

        # 状態管理
        self.active_sessions = {}
        self.learning_data = {}
        self.performance_metrics = {}

        # GUI統合準備
        self.gui_bridge = None

    async def initialize(self):
        """初期化処理"""
        self.logger.info("Enhanced Research Agent initializing")

        # Resident Agent起動
        await self.resident_agent.start()

        # GUIブリッジ設定
        self.gui_bridge = EnhancedGUIBridge(self)

        # 学習データ読み込み
        await self.load_learning_data()

        self.logger.info("Enhanced Research Agent initialized")

    async def shutdown(self):
        """シャットダウン処理"""
        self.logger.info("Enhanced Research Agent shutting down")

        # アクティブセッション終了
        await self.cleanup_active_sessions()

        # 学習データ保存
        await self.save_learning_data()

        # Resident Agent停止
        await self.resident_agent.stop()

        self.logger.info("Enhanced Research Agent shutdown complete")

    async def execute_enhanced_task(self, task_description: str, context: Dict[str, Any] = None) -> Dict[str, Any]:
        """
        拡張タスク実行（Cowork + Deepresearch統合）

        Args:
            task_description: タスク説明
            context: 実行コンテキスト

        Returns:
            実行結果
        """
        start_time = time.time()
        session_id = f"session_{int(start_time)}"

        try:
            self.logger.info(f"Enhanced task execution starting: {task_description}")

            # セッション作成
            session = await self.create_session(session_id, task_description, context)

            # タスクタイプ分析
            task_analysis = await self.analyze_task_type(task_description, context)

            # タスク実行
            if task_analysis["is_research_task"]:
                result = await self.execute_research_task(task_description, task_analysis, context)
            elif task_analysis["is_cowork_task"]:
                result = await self.execute_cowork_task(task_description, task_analysis, context)
            else:
                result = await self.execute_hybrid_task(task_description, task_analysis, context)

            # 結果強化
            enhanced_result = await self.enhance_result(result, task_analysis)

            # 学習データ更新
            await self.learning_system.learn_from_execution(
                task_description, task_analysis, result, enhanced_result
            )

            # パフォーマンスメトリクス更新
            execution_time = time.time() - start_time
            await self.update_performance_metrics(session_id, execution_time, result)

            # セッション完了
            await self.complete_session(session_id, enhanced_result)

            return enhanced_result

        except Exception as e:
            self.logger.error(f"Enhanced task execution error: {session_id} - {e}")
            await self.handle_task_error(session_id, e)
            return {
                "success": False,
                "error": str(e),
                "session_id": session_id,
                "execution_time": time.time() - start_time
            }

    async def analyze_task_type(self, task_description: str, context: Dict[str, Any]) -> Dict[str, Any]:
        """タスクタイプ分析"""
        analysis = {
            "is_research_task": False,
            "is_cowork_task": False,
            "is_hybrid_task": False,
            "research_confidence": 0.0,
            "cowork_confidence": 0.0,
            "complexity": "low",
            "estimated_duration": 30,
            "required_capabilities": []
        }

        # 研究タスク判定
        research_keywords = [
            "research", "analyze", "investigate", "study", "explore",
            "調査", "分析", "研究", "探求"
        ]
        research_score = sum(1 for keyword in research_keywords if keyword.lower() in task_description.lower())
        analysis["research_confidence"] = min(research_score * 0.2, 1.0)
        analysis["is_research_task"] = analysis["research_confidence"] > 0.6

        # Coworkタスク判定
        cowork_keywords = [
            "organize", "process", "create", "manage", "file",
            "整理", "処理", "作成", "管理", "ファイル"
        ]
        cowork_score = sum(1 for keyword in cowork_keywords if keyword.lower() in task_description.lower())
        analysis["cowork_confidence"] = min(cowork_score * 0.15, 1.0)
        analysis["is_cowork_task"] = analysis["cowork_confidence"] > 0.5

        # ハイブリッド判定
        analysis["is_hybrid_task"] = analysis["is_research_task"] and analysis["is_cowork_task"]

        # 複雑さ判定
        if len(task_description.split()) > 20 or analysis["is_hybrid_task"]:
            analysis["complexity"] = "high"
            analysis["estimated_duration"] = 120
        elif len(task_description.split()) > 10:
            analysis["complexity"] = "medium"
            analysis["estimated_duration"] = 60

        # 必要な機能判定
        if analysis["is_research_task"]:
            analysis["required_capabilities"].extend(["web_search", "data_analysis", "report_generation"])
        if analysis["is_cowork_task"]:
            analysis["required_capabilities"].extend(["file_management", "data_processing"])

        return analysis

    async def execute_research_task(self, task_description: str, analysis: Dict[str, Any], context: Dict[str, Any]) -> Dict[str, Any]:
        """研究タスク実行"""
        self.logger.info("Research task execution starting")

        # Deepresearchパラメータ構築
        research_args = type('Args', (), {
            'query': task_description,
            'depth': 'comprehensive' if analysis['complexity'] == 'high' else 'basic',
            'sources': ['google', 'bing', 'scholar'] if analysis['complexity'] == 'high' else ['google'],
            'credibility_threshold': 0.8,
            'max_sources': 20 if analysis['complexity'] == 'high' else 10,
            'output_format': 'markdown'
        })()

        # 研究実行
        result = await self.deep_research.run_research_topic(research_args)

        return result

    async def execute_cowork_task(self, task_description: str, analysis: Dict[str, Any], context: Dict[str, Any]) -> Dict[str, Any]:
        """Coworkタスク実行"""
        self.logger.info("Cowork task execution starting")

        # 機能検索
        features = self.feature_search.search_features(task_description, limit=3)
        if features:
            # 最適機能選択
            best_feature = features[0]  # スコア順
            task_description = self.feature_search.generate_task_from_feature(best_feature)

        # Cowork実行
        result = await self.productivity_assistant.execute_task(task_description)

        return result

    async def execute_hybrid_task(self, task_description: str, analysis: Dict[str, Any], context: Dict[str, Any]) -> Dict[str, Any]:
        """ハイブリッドタスク実行（研究 + Cowork）"""
        self.logger.info("Hybrid task execution starting")

        # 並列実行
        research_task = self.execute_research_task(task_description, analysis, context)
        cowork_task = self.execute_cowork_task(task_description, analysis, context)

        research_result, cowork_result = await asyncio.gather(research_task, cowork_task)

        # 結果統合
        integrated_result = {
            "success": research_result.get("success", False) and cowork_result.get("success", False),
            "research_component": research_result,
            "cowork_component": cowork_result,
            "integrated_analysis": await self.integrate_results(research_result, cowork_result),
            "execution_type": "hybrid"
        }

        return integrated_result

    async def enhance_result(self, result: Dict[str, Any], analysis: Dict[str, Any]) -> Dict[str, Any]:
        """結果強化"""
        enhanced = result.copy()

        # スマートサジェスチョン追加
        enhanced["suggestions"] = await self.smart_suggestions.generate_suggestions(
            "task", analysis, {}
        )

        # 洞察生成
        enhanced["insights"] = await self.generate_insights(result, analysis)

        # 次のアクション提案
        enhanced["next_actions"] = await self.generate_next_actions(result, analysis)

        # 品質スコア
        enhanced["quality_score"] = await self.calculate_quality_score(result, analysis)

        return enhanced

    async def generate_insights(self, result: Dict[str, Any], analysis: Dict[str, Any]) -> List[str]:
        """洞察生成"""
        insights = []

        if result.get("success"):
            if analysis["is_research_task"]:
                insights.append("Research data collected successfully")
                if result.get("execution_time", 0) < analysis["estimated_duration"]:
                    insights.append("Completed faster than estimated")

            if analysis["is_cowork_task"]:
                insights.append("Productivity task executed successfully")

        return insights

    async def generate_next_actions(self, result: Dict[str, Any], analysis: Dict[str, Any]) -> List[str]:
        """次のアクション提案"""
        actions = []

        if result.get("success"):
            if analysis["is_research_task"]:
                actions.append("Review research results in detail")
                actions.append("Research additional related topics")

            if analysis["is_cowork_task"]:
                actions.append("Utilize generated files")
                actions.append("Set up regular execution")

        return actions

    async def calculate_quality_score(self, result: Dict[str, Any], analysis: Dict[str, Any]) -> float:
        """品質スコア計算"""
        score = 0.5  # ベーススコア

        if result.get("success"):
            score += 0.3

        if result.get("execution_time", float('inf')) <= analysis["estimated_duration"]:
            score += 0.2

        return min(score, 1.0)

    async def integrate_results(self, research_result: Dict[str, Any], cowork_result: Dict[str, Any]) -> Dict[str, Any]:
        """結果統合"""
        integration = {
            "combined_success": research_result.get("success", False) and cowork_result.get("success", False),
            "research_summary": research_result.get("summary", ""),
            "cowork_summary": cowork_result.get("summary", ""),
            "synergistic_insights": []
        }

        # シナジー洞察生成
        if research_result.get("success") and cowork_result.get("success"):
            integration["synergistic_insights"].append("Research data and productivity tasks executed in synergy")

        return integration

    async def create_session(self, session_id: str, task_description: str, context: Dict[str, Any]) -> Dict[str, Any]:
        """セッション作成"""
        session = {
            "id": session_id,
            "task_description": task_description,
            "context": context or {},
            "start_time": datetime.now(),
            "status": "active",
            "progress": 0.0
        }

        self.active_sessions[session_id] = session
        return session

    async def complete_session(self, session_id: str, result: Dict[str, Any]):
        """セッション完了"""
        if session_id in self.active_sessions:
            session = self.active_sessions[session_id]
            session["status"] = "completed"
            session["end_time"] = datetime.now()
            session["result"] = result
            session["progress"] = 1.0

    async def handle_task_error(self, session_id: str, error: Exception):
        """タスクエラー処理"""
        if session_id in self.active_sessions:
            session = self.active_sessions[session_id]
            session["status"] = "error"
            session["error"] = str(error)
            session["end_time"] = datetime.now()

    async def cleanup_active_sessions(self):
        """アクティブセッションクリーンアップ"""
        for session_id, session in self.active_sessions.items():
            if session["status"] == "active":
                session["status"] = "interrupted"
                session["end_time"] = datetime.now()

    async def load_learning_data(self):
        """学習データ読み込み"""
        try:
            learning_file = Path.home() / ".enhanced_research_agent" / "learning_data.json"
            if learning_file.exists():
                with open(learning_file, 'r', encoding='utf-8') as f:
                    self.learning_data = json.load(f)
        except Exception as e:
            self.logger.warning(f"Learning data load error: {e}")

    async def save_learning_data(self):
        """学習データ保存"""
        try:
            data_dir = Path.home() / ".enhanced_research_agent"
            data_dir.mkdir(exist_ok=True)
            learning_file = data_dir / "learning_data.json"

            with open(learning_file, 'w', encoding='utf-8') as f:
                json.dump(self.learning_data, f, indent=2, ensure_ascii=False)
        except Exception as e:
            self.logger.error(f"Learning data save error: {e}")

    async def update_performance_metrics(self, session_id: str, execution_time: float, result: Dict[str, Any]):
        """パフォーマンスメトリクス更新"""
        metrics = {
            "session_id": session_id,
            "execution_time": execution_time,
            "success": result.get("success", False),
            "timestamp": datetime.now().isoformat()
        }

        # メトリクス保存（簡易版）
        if "performance_history" not in self.performance_metrics:
            self.performance_metrics["performance_history"] = []

        self.performance_metrics["performance_history"].append(metrics)

        # 最新10件のみ保持
        if len(self.performance_metrics["performance_history"]) > 10:
            self.performance_metrics["performance_history"] = self.performance_metrics["performance_history"][-10:]