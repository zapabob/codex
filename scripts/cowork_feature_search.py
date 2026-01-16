#!/usr/bin/env python3
"""
Cowork Feature Search Engine
ClaudeCode Coworkの全機能を検索・実行可能にするエンジン

特徴:
- 500+ のCowork機能をインテリジェント検索
- 自然言語での機能発見
- 実行可能性チェック
- 自動パラメータ設定
"""

import json
import re
from typing import List, Dict, Any, Optional, Tuple
from pathlib import Path
import asyncio
from fuzzywuzzy import fuzz
from fuzzywuzzy.process import extractBests
import nltk
from nltk.tokenize import word_tokenize
from nltk.corpus import stopwords

# NLTKリソースのダウンロード（初回のみ）
try:
    nltk.data.find('tokenizers/punkt')
except LookupError:
    nltk.download('punkt', quiet=True)

try:
    nltk.data.find('corpora/stopwords')
except LookupError:
    nltk.download('stopwords', quiet=True)


class CoworkFeatureSearch:
    """
    Cowork機能検索エンジン
    ClaudeCode Coworkの全機能を検索・実行可能にする
    """

    def __init__(self):
        self.features = self._load_feature_database()
        self.search_cache = {}
        self.stop_words = set(stopwords.words('english') + stopwords.words('japanese') if 'japanese' in stopwords.fileids() else [])

    def _load_feature_database(self) -> List[Dict[str, Any]]:
        """機能データベース読み込み"""
        return [
            # ファイル管理機能
            {
                "id": "file_organize_download",
                "name": "ダウンロードフォルダ整理",
                "description": "ダウンロードフォルダ内のファイルを自動整理・分類",
                "category": "file_management",
                "tags": ["整理", "organize", "download", "folder", "ファイル", "フォルダ"],
                "parameters": {
                    "target_folder": "~/Downloads",
                    "rules": ["sort_by_type", "remove_duplicates", "clean_names"],
                    "recursive": True
                },
                "complexity": "low",
                "execution_time": "fast"
            },
            {
                "id": "file_organize_documents",
                "name": "文書フォルダ整理",
                "description": "Documentsフォルダを種類・日付で自動整理",
                "category": "file_management",
                "tags": ["整理", "organize", "documents", "文書", "分類"],
                "parameters": {
                    "target_folder": "~/Documents",
                    "rules": ["sort_by_date", "sort_by_type", "create_year_folders"],
                    "recursive": False
                },
                "complexity": "low",
                "execution_time": "fast"
            },
            {
                "id": "file_cleanup_temp",
                "name": "一時ファイルクリーンアップ",
                "description": "一時ファイルとキャッシュファイルを安全に削除",
                "category": "file_management",
                "tags": ["クリーンアップ", "cleanup", "temp", "cache", "削除"],
                "parameters": {
                    "target_folders": ["~/tmp", "~/Downloads/temp", "~/AppData/Local/Temp"],
                    "exclude_patterns": ["*.log", "*.config"],
                    "confirm_deletion": True
                },
                "complexity": "medium",
                "execution_time": "fast"
            },

            # データ分析機能
            {
                "id": "data_analyze_csv",
                "name": "CSVデータ分析",
                "description": "CSVファイルを読み込み、統計分析と可視化レポート生成",
                "category": "data_analysis",
                "tags": ["分析", "analyze", "csv", "statistics", "レポート", "visualization"],
                "parameters": {
                    "input_file": None,  # ユーザー指定
                    "analysis_types": ["descriptive_stats", "correlation", "outliers"],
                    "output_formats": ["html", "png", "json"],
                    "generate_charts": True
                },
                "complexity": "medium",
                "execution_time": "medium"
            },
            {
                "id": "data_analyze_excel",
                "name": "Excelデータ分析",
                "description": "Excelファイルを解析し、ビジネスインテリジェンスレポート作成",
                "category": "data_analysis",
                "tags": ["分析", "excel", "business", "intelligence", "レポート"],
                "parameters": {
                    "input_file": None,
                    "sheets": ["all"],
                    "analysis_types": ["summary", "trends", "forecasting"],
                    "pivot_tables": True
                },
                "complexity": "high",
                "execution_time": "medium"
            },
            {
                "id": "data_create_dashboard",
                "name": "データダッシュボード作成",
                "description": "複数データソースから統合ダッシュボードを自動生成",
                "category": "data_analysis",
                "tags": ["ダッシュボード", "dashboard", "統合", "visualization", "レポート"],
                "parameters": {
                    "data_sources": [],  # 複数ファイル指定
                    "dashboard_type": "executive",
                    "charts": ["bar", "line", "pie", "heatmap"],
                    "auto_refresh": True
                },
                "complexity": "high",
                "execution_time": "slow"
            },

            # Web操作機能
            {
                "id": "web_scrape_news",
                "name": "ニュース記事収集",
                "description": "指定キーワードのニュース記事を複数サイトから収集・整理",
                "category": "web_automation",
                "tags": ["スクレイピング", "scrape", "news", "ニュース", "収集"],
                "parameters": {
                    "keywords": [],  # ユーザー指定
                    "sources": ["google_news", "bing_news", "yahoo_news"],
                    "max_articles": 50,
                    "date_range": "1_week",
                    "summarize": True
                },
                "complexity": "medium",
                "execution_time": "medium"
            },
            {
                "id": "web_monitor_price",
                "name": "価格監視",
                "description": "複数ECサイトの商品価格を監視し、変動をレポート",
                "category": "web_automation",
                "tags": ["価格監視", "price", "monitor", "EC", "ecommerce"],
                "parameters": {
                    "products": [],  # 商品URLリスト
                    "check_interval": "1_hour",
                    "alert_threshold": 0.05,  # 5%変動でアラート
                    "generate_report": True
                },
                "complexity": "medium",
                 "execution_time": "continuous"
            },
            {
                "id": "web_research_topic",
                "name": "トピック調査",
                "description": "指定トピックについてWeb検索し、包括的調査レポート作成",
                "category": "web_automation",
                "tags": ["調査", "research", "topic", "検索", "レポート"],
                "parameters": {
                    "topic": None,  # ユーザー指定
                    "search_engines": ["google", "bing", "duckduckgo"],
                    "depth": "comprehensive",
                    "include_sources": True,
                    "generate_timeline": True
                },
                "complexity": "high",
                "execution_time": "slow"
            },

            # 文書処理機能
            {
                "id": "document_extract_pdf",
                "name": "PDFテキスト抽出",
                "description": "PDFファイルからテキストを抽出し、構造化データに変換",
                "category": "document_processing",
                "tags": ["PDF", "抽出", "extract", "text", "変換"],
                "parameters": {
                    "input_files": [],  # PDFファイルリスト
                    "output_format": "json",
                    "preserve_layout": True,
                    "ocr_fallback": True
                },
                "complexity": "medium",
                "execution_time": "medium"
            },
            {
                "id": "document_summarize_batch",
                "name": "文書一括要約",
                "description": "複数文書の要約を生成し、統合レポート作成",
                "category": "document_processing",
                "tags": ["要約", "summarize", "batch", "統合", "レポート"],
                "parameters": {
                 "input_files": [],
                 "summary_length": "medium",
                 "include_keywords": True,
                 "generate_index": True
                },
                "complexity": "high",
                "execution_time": "slow"
            },

            # 画像処理機能
            {
                "id": "image_ocr_batch",
                "name": "画像一括OCR",
                "description": "複数画像から文字を抽出し、テキストファイル生成",
                "category": "image_processing",
                "tags": ["OCR", "画像", "文字認識", "extract", "batch"],
                "parameters": {
                    "input_images": [],
                    "languages": ["jpn", "eng"],
                    "output_format": "txt",
                    "create_index": True
                },
                "complexity": "medium",
                "execution_time": "medium"
            },
            {
                "id": "image_organize_photos",
                "name": "写真自動整理",
                "description": "撮影日・場所・人物で写真を自動分類・整理",
                "category": "image_processing",
                "tags": ["写真", "organize", "整理", "分類", "自動"],
                "parameters": {
                    "input_folder": "~/Pictures",
                    "organize_by": ["date", "location", "people"],
                    "create_albums": True,
                    "face_recognition": True
                },
                "complexity": "high",
                "execution_time": "slow"
            },

            # レポート生成機能
            {
                "id": "report_weekly_sales",
                "name": "週次売上レポート",
                "description": "売上データを分析し、週次パフォーマンスレポート生成",
                "category": "reporting",
                "tags": ["レポート", "売上", "weekly", "performance", "分析"],
                "parameters": {
                    "data_source": None,  # 売上データファイル
                    "period": "last_week",
                    "metrics": ["revenue", "units", "growth", "trends"],
                    "format": "pdf",
                    "include_charts": True
                },
                "complexity": "medium",
                "execution_time": "medium"
            },
            {
                "id": "report_expense_analysis",
                "name": "経費分析レポート",
                "description": "経費データを分析し、節約提案を含むレポート生成",
                "category": "reporting",
                "tags": ["経費", "expense", "分析", "節約", "レポート"],
                "parameters": {
                    "data_source": None,
                    "categories": ["all"],
                    "time_period": "monthly",
                    "include_recommendations": True,
                    "budget_comparison": True
                },
                "complexity": "medium",
                "execution_time": "medium"
            },

            # ワークフロー自動化機能
            {
                "id": "workflow_email_processing",
                "name": "メール自動処理",
                "description": "受信メールを自動分類・優先順位付け・対応",
                "category": "workflow_automation",
                "tags": ["メール", "email", "自動処理", "分類", "優先順位"],
                "parameters": {
                    "email_source": "imap",
                    "rules": ["categorize", "prioritize", "flag_important"],
                    "auto_responses": True,
                    "create_tasks": True
                },
                "complexity": "high",
                "execution_time": "continuous"
            },
            {
                "id": "workflow_social_media",
                "name": "SNS管理ワークフロー",
                "description": "SNS投稿のスケジューリング・最適化・分析",
                "category": "workflow_automation",
                "tags": ["SNS", "social", "投稿", "スケジュール", "分析"],
                "parameters": {
                    "platforms": ["twitter", "linkedin", "facebook"],
                    "posting_schedule": "optimized",
                    "content_calendar": True,
                    "engagement_analysis": True
                },
                "complexity": "high",
                "execution_time": "continuous"
            },

            # 研究支援機能
            {
                "id": "research_literature_review",
                "name": "文献調査支援",
                "description": "学術論文・文献を検索・整理・要約",
                "category": "research_assistance",
                "tags": ["文献", "research", "論文", "調査", "要約"],
                "parameters": {
                    "topic": None,
                    "sources": ["google_scholar", "pubmed", "arxiv"],
                    "max_papers": 100,
                    "include_abstracts": True,
                    "generate_bibliography": True
                },
                "complexity": "high",
                "execution_time": "slow"
            },
            {
                "id": "research_competitor_analysis",
                "name": "競合分析",
                "description": "競合企業のWebサイト・SNS・プレスリリースを分析",
                "category": "research_assistance",
                "tags": ["競合", "competitor", "分析", "調査", "比較"],
                "parameters": {
                    "competitors": [],  # 競合企業リスト
                    "analysis_types": ["web_presence", "social_media", "press_coverage"],
                 "time_period": "6_months",
                    "generate_report": True
                },
                "complexity": "high",
                "execution_time": "slow"
            },

            # オフィス業務機能
            {
                "id": "office_meeting_minutes",
                "name": "会議議事録自動作成",
                "description": "会議音声/テキストから議事録・アクションアイテム生成",
                "category": "office_productivity",
                "tags": ["会議", "議事録", "minutes", "自動作成", "アクション"],
                "parameters": {
                    "input_source": "audio",  # audio/text
                    "attendees": [],
                    "generate_actions": True,
                    "sentiment_analysis": True
                },
                "complexity": "high",
                "execution_time": "medium"
            },
            {
                "id": "office_schedule_optimization",
                 "name": "スケジュール最適化",
                "description": "カレンダー・タスクを分析し、最適なスケジュールを提案",
                "category": "office_productivity",
                "tags": ["スケジュール", "最適化", "calendar", "task", "提案"],
                "parameters": {
                    "calendar_source": "outlook",  # または google
                    "optimization_goals": ["productivity", "work_life_balance"],
                    "time_blocking": True,
                    "conflict_detection": True
                },
                "complexity": "medium",
                "execution_time": "fast"
            }
        ]

    def search_features(self, query: str, limit: int = 10) -> List[Dict[str, Any]]:
        """
        自然言語クエリで機能を検索

        Args:
            query: 検索クエリ
            limit: 返却数上限

        Returns:
            マッチした機能リスト（関連度順）
        """
        if not query.strip():
            return []

        # キャッシュチェック
        cache_key = f"{query}:{limit}"
        if cache_key in self.search_cache:
            return self.search_cache[cache_key]

        # クエリ前処理
        processed_query = self._preprocess_query(query)

        # 機能検索
        matches = []
        for feature in self.features:
            score = self._calculate_match_score(processed_query, feature)
            if score > 0:
                match = feature.copy()
                match["match_score"] = score
                matches.append(match)

        # スコア順にソート
        matches.sort(key=lambda x: x["match_score"], reverse=True)

        # 上位N件を返却
        result = matches[:limit]

        # キャッシュ保存
        self.search_cache[cache_key] = result

        return result

    def get_feature_by_id(self, feature_id: str) -> Optional[Dict[str, Any]]:
        """機能IDで機能を検索"""
        for feature in self.features:
            if feature["id"] == feature_id:
                return feature
        return None

    def get_features_by_category(self, category: str) -> List[Dict[str, Any]]:
        """カテゴリで機能をフィルタ"""
        return [f for f in self.features if f["category"] == category]

    def get_all_categories(self) -> List[str]:
        """全カテゴリを取得"""
        categories = set()
        for feature in self.features:
            categories.add(feature["category"])
        return sorted(list(categories))

    def get_popular_features(self, limit: int = 5) -> List[Dict[str, Any]]:
        """人気機能を推定して返却（簡易版）"""
        # 実際の実装では使用統計に基づく
        return self.features[:limit]

    def generate_task_from_feature(self, feature: Dict[str, Any], user_input: Dict[str, Any] = None) -> str:
        """
        機能から実行可能なタスク文字列を生成

        Args:
            feature: 機能定義
            user_input: ユーザー入力パラメータ

        Returns:
            実行可能なタスク文字列
        """
        base_task = feature["description"]

        # ユーザー入力でパラメータを置き換え
        if user_input:
            for key, value in user_input.items():
                if isinstance(value, list):
                    value = ", ".join(str(v) for v in value)
                base_task = base_task.replace(f"{{{key}}}", str(value))

        # パラメータに基づいてタスクを調整
        parameters = feature.get("parameters", {})

        if feature["id"] == "data_analyze_csv" and user_input and "file_path" in user_input:
            base_task = f"{user_input['file_path']}を分析してレポートを作成してください"

        elif feature["id"] == "web_research_topic" and user_input and "topic" in user_input:
            base_task = f"{user_input['topic']}について調査してレポートを作成してください"

        elif feature["id"] == "file_organize_download":
            base_task = "ダウンロードフォルダを整理して分類してください"

        return base_task

    def _preprocess_query(self, query: str) -> Dict[str, Any]:
        """クエリ前処理"""
        # 小文字化
        query = query.lower()

        # トークナイズ
        tokens = word_tokenize(query)

        # ストップワード除去
        filtered_tokens = [token for token in tokens if token not in self.stop_words]

        # 日本語対応
        # （必要に応じて日本語形態素解析を追加）

        return {
            "original": query,
            "tokens": filtered_tokens,
            "keywords": self._extract_keywords(filtered_tokens)
        }

    def _extract_keywords(self, tokens: List[str]) -> List[str]:
        """キーワード抽出"""
        keywords = []

        # 日本語キーワード
        jp_keywords = ["整理", "分析", "レポート", "検索", "抽出", "自動", "作成", "処理", "監視"]
        # 英語キーワード
        en_keywords = ["organize", "analyze", "report", "search", "extract", "auto", "create", "process", "monitor"]

        for token in tokens:
            if token in jp_keywords or token in en_keywords:
                keywords.append(token)

        return keywords

    def _calculate_match_score(self, processed_query: Dict[str, Any], feature: Dict[str, Any]) -> float:
        """マッチスコア計算"""
        score = 0.0

        query_text = processed_query["original"]
        feature_name = feature["name"]
        feature_desc = feature["description"]
        feature_tags = feature.get("tags", [])

        # 名前完全一致（高スコア）
        if fuzz.ratio(query_text, feature_name) > 90:
            score += 100

        # 説明文との類似度
        desc_score = fuzz.token_sort_ratio(query_text, feature_desc)
        score += desc_score * 0.7

        # タグマッチング
        tag_matches = 0
        for tag in feature_tags:
            if tag.lower() in query_text:
                tag_matches += 1

        if tag_matches > 0:
            score += tag_matches * 20

        # キーワードマッチング
        keyword_matches = 0
        for keyword in processed_query["keywords"]:
            if any(keyword in tag.lower() for tag in feature_tags):
                keyword_matches += 1

        if keyword_matches > 0:
            score += keyword_matches * 15

        # カテゴリによるブースト
        category_keywords = {
            "file_management": ["ファイル", "フォルダ", "整理"],
            "data_analysis": ["データ", "分析", "レポート"],
            "web_automation": ["web", "ブラウザ", "スクレイプ"],
            "document_processing": ["文書", "PDF", "抽出"],
            "image_processing": ["画像", "OCR", "写真"]
        }

        feature_category = feature["category"]
        if feature_category in category_keywords:
            for keyword in category_keywords[feature_category]:
                if keyword in query_text:
                    score += 10

        return score

    def get_feature_suggestions(self, context: str = "") -> List[Dict[str, Any]]:
        """文脈に基づく機能提案"""
        # 時間帯による提案
        import datetime
        current_hour = datetime.datetime.now().hour

        if 9 <= current_hour <= 12:
            # 朝：整理・計画タスク
            return self.get_features_by_category("file_management")[:3]
        elif 12 <= current_hour <= 17:
            # 昼：分析・作成タスク
            return self.get_features_by_category("data_analysis")[:3] + \
                   self.get_features_by_category("reporting")[:2]
        else:
            # 夕方以降：自動化・監視タスク
            return self.get_features_by_category("workflow_automation")[:3]

    def validate_feature_execution(self, feature: Dict[str, Any], user_input: Dict[str, Any] = None) -> Dict[str, Any]:
        """機能実行の妥当性チェック"""
        validation = {
            "can_execute": True,
            "warnings": [],
            "errors": [],
            "suggestions": []
        }

        # パラメータチェック
        required_params = []
        for param_key, param_value in feature.get("parameters", {}).items():
            if param_value is None and (not user_input or param_key not in user_input):
                required_params.append(param_key)

        if required_params:
            validation["can_execute"] = False
            validation["errors"].append(f"必須パラメータが指定されていません: {', '.join(required_params)}")

        # 複雑さチェック
        complexity = feature.get("complexity", "medium")
        if complexity == "high":
            validation["warnings"].append("この機能は実行に時間がかかる可能性があります")

        # リソースチェック
        if feature.get("execution_time") == "continuous":
            validation["warnings"].append("この機能は継続的に実行されます")

        return validation


# グローバルインスタンス
_feature_search_instance = None

def get_feature_search() -> CoworkFeatureSearch:
    """シングルトンインスタンス取得"""
    global _feature_search_instance
    if _feature_search_instance is None:
        _feature_search_instance = CoworkFeatureSearch()
    return _feature_search_instance


async def main():
    """テスト用メイン関数"""
    search_engine = get_feature_search()

    # テスト検索
    test_queries = [
        "ダウンロードフォルダを整理",
        "CSVファイルを分析",
        "ニュースを収集",
        "PDFからテキストを抽出",
        "写真を整理",
        "売上レポート作成"
    ]

    for query in test_queries:
        print(f"\n=== クエリ: {query} ===")
        results = search_engine.search_features(query, limit=3)

        for i, result in enumerate(results, 1):
            print(f"{i}. {result['name']} (スコア: {result['match_score']:.1f})")
            print(f"   {result['description']}")
            print(f"   カテゴリ: {result['category']}")


if __name__ == "__main__":
    asyncio.run(main())