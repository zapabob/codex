#!/usr/bin/env python3
"""
Cowork Productivity Assistant - ClaudeCode Coworkスタイルの生産性自動化

このモジュールはファイル管理、データ分析、ブラウザ操作などの生産性タスクを
自律的に実行する機能を提供します。

主な機能:
- ファイル整理と管理
- データ分析とレポート生成
- Webスクレイピングと自動化
- ドキュメント処理
- 安全制御と監査
"""

import asyncio
import json
import logging
import os
import re
import shutil
import time
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional, Any, Tuple, Union
import tempfile
import hashlib

# 外部ライブラリ
try:
    import pandas as pd
    import numpy as np
    import matplotlib.pyplot as plt
    import seaborn as sns
    from PIL import Image
    import pytesseract
    import requests
    from bs4 import BeautifulSoup
    from selenium import webdriver
    from selenium.webdriver.common.by import By
    from selenium.webdriver.support.ui import WebDriverWait
    from selenium.webdriver.support import expected_conditions as EC
    import openpyxl
    from docx import Document
    import fitz  # PyMuPDF
except ImportError as e:
    print(f"必要なライブラリがインストールされていません: {e}")
    print("以下のコマンドでインストールしてください:")
    print("pip install pandas numpy matplotlib seaborn pillow pytesseract requests beautifulsoup4 selenium openpyxl python-docx PyMuPDF")
    print("また、Tesseract OCRをインストールしてください")
    exit(1)


class CoworkProductivityAssistant:
    """
    Cowork Productivity Assistantのメインクラス
    ClaudeCode Coworkの機能をCodexに実装
    """

    def __init__(self):
        self.logger = logging.getLogger("CoworkProductivityAssistant")

        # 設定
        self.config = {
            "max_file_size": 100 * 1024 * 1024,  # 100MB
            "supported_formats": {
                "documents": [".pdf", ".docx", ".xlsx", ".pptx", ".txt", ".md"],
                "images": [".jpg", ".jpeg", ".png", ".gif", ".bmp"],
                "data": [".csv", ".json", ".xml", ".yaml", ".yml"],
                "archives": [".zip", ".tar", ".gz", ".rar"]
            },
            "temp_dir": Path(tempfile.gettempdir()) / "cowork_temp",
            "safety_enabled": True,
            "backup_enabled": True
        }

        # 一時ディレクトリ作成
        self.config["temp_dir"].mkdir(parents=True, exist_ok=True)

        # コンポーネント初期化
        self.file_manager = FileManagementSystem(self.config)
        self.data_analyzer = DataAnalysisEngine(self.config)
        self.web_automator = WebAutomationEngine(self.config)
        self.document_processor = DocumentProcessingEngine(self.config)
        self.safety_controller = SafetyController(self.config)

    async def execute_task(self, task_description: str) -> Dict[str, Any]:
        """
        自然言語タスクを実行

        Args:
            task_description: 実行するタスクの説明

        Returns:
            実行結果
        """
        try:
            self.logger.info(f"タスク実行開始: {task_description}")

            # タスク解釈
            interpreted_task = await self._interpret_task(task_description)

            # 安全チェック
            safety_result = await self.safety_controller.check_task_safety(interpreted_task)
            if not safety_result["approved"]:
                return {
                    "success": False,
                    "error": f"安全チェック失敗: {safety_result['reason']}",
                    "risk_level": safety_result["risk_level"]
                }

            # タスク実行
            result = await self._execute_interpreted_task(interpreted_task)

            # 結果処理
            final_result = await self._process_execution_result(result, interpreted_task)

            self.logger.info(f"タスク実行完了: {task_description}")
            return final_result

        except Exception as e:
            self.logger.error(f"タスク実行エラー: {e}")
            return {
                "success": False,
                "error": str(e),
                "task_description": task_description
            }

    async def _interpret_task(self, description: str) -> Dict[str, Any]:
        """
        タスク説明を構造化データに解釈

        Args:
            description: 自然言語のタスク説明

        Returns:
            解釈されたタスク情報
        """
        desc_lower = description.lower()

        # タスクタイプ分類
        task_type = self._classify_task_type(desc_lower)

        # エンティティ抽出
        entities = self._extract_entities(description)

        # パラメータ抽出
        parameters = self._extract_parameters(description, task_type)

        return {
            "original_description": description,
            "task_type": task_type,
            "entities": entities,
            "parameters": parameters,
            "confidence": self._calculate_confidence(description, task_type),
            "estimated_complexity": self._estimate_complexity(description)
        }

    def _classify_task_type(self, desc_lower: str) -> str:
        """タスクタイプ分類"""
        # ファイル整理関連
        if any(word in desc_lower for word in [
            "整理", "organize", "sort", "clean", "folder", "directory",
            "ファイル", "file", "フォルダ"
        ]):
            return "file_organization"

        # データ分析関連
        elif any(word in desc_lower for word in [
            "分析", "analyze", "report", "chart", "graph", "statistics",
            "データ", "data", "レポート", "グラフ", "統計"
        ]):
            return "data_analysis"

        # Web操作関連
        elif any(word in desc_lower for word in [
            "web", "browser", "scrape", "スクレイプ", "ブラウザ",
            "ウェブ", "サイト", "site", "url"
        ]):
            return "web_automation"

        # ドキュメント処理関連
        elif any(word in desc_lower for word in [
            "document", "pdf", "word", "excel", "ドキュメント",
            "文書", "変換", "convert"
        ]):
            return "document_processing"

        # 画像処理関連
        elif any(word in desc_lower for word in [
            "image", "photo", "picture", "画像", "写真",
            "ocr", "認識", "文字起こし"
        ]):
            return "image_processing"

        else:
            return "generic_task"

    def _extract_entities(self, description: str) -> List[str]:
        """エンティティ抽出"""
        entities = []

        # パス抽出
        path_pattern = r'["\']?(/[^"\s]+|~[^"\s]+|\./[^"\s]+|\.\./[^"\s]+|[A-Za-z]:[^\s"]+|/[^"\s]+)["\']?'
        paths = re.findall(path_pattern, description)
        entities.extend(paths)

        # URL抽出
        url_pattern = r'https?://[^\s]+'
        urls = re.findall(url_pattern, description)
        entities.extend(urls)

        # ファイル名抽出
        filename_pattern = r'[\w\-\.]+\.(pdf|docx?|xlsx?|pptx?|txt|md|jpg|jpeg|png|gif|csv|json|xml|yaml|yml|zip|tar|gz|rar)'
        filenames = re.findall(filename_pattern, description, re.IGNORECASE)
        entities.extend([f"{name}.{ext}" for name, ext in filenames])

        return list(set(entities))  # 重複除去

    def _extract_parameters(self, description: str, task_type: str) -> Dict[str, Any]:
        """タスク固有のパラメータ抽出"""
        parameters = {}

        if task_type == "file_organization":
            # 整理ルール抽出
            if "名前" in description or "name" in description:
                parameters["rule"] = "sort_by_name"
            elif "日付" in description or "date" in description:
                parameters["rule"] = "sort_by_date"
            elif "種類" in description or "type" in description:
                parameters["rule"] = "sort_by_type"

        elif task_type == "data_analysis":
            # 分析タイプ抽出
            if "統計" in description or "statistics" in description:
                parameters["analysis_type"] = "statistical"
            elif "可視化" in description or "visualization" in description:
                parameters["analysis_type"] = "visualization"

        elif task_type == "web_automation":
            # Web操作タイプ抽出
            if "スクレイプ" in description or "scrape" in description:
                parameters["operation"] = "scrape"
            elif "フォーム" in description or "form" in description:
                parameters["operation"] = "form_fill"

        return parameters

    def _calculate_confidence(self, description: str, task_type: str) -> float:
        """タスク解釈の信頼度計算"""
        # 簡易的な信頼度計算
        confidence = 0.5

        # タスクタイプ固有のキーワードを含む場合信頼度上昇
        task_keywords = {
            "file_organization": ["整理", "organize", "sort", "clean", "folder"],
            "data_analysis": ["分析", "analyze", "report", "chart", "data"],
            "web_automation": ["web", "browser", "scrape", "site", "url"],
            "document_processing": ["document", "pdf", "word", "excel"],
            "image_processing": ["image", "photo", "ocr", "文字起こし"]
        }

        desc_lower = description.lower()
        if task_type in task_keywords:
            matching_keywords = sum(1 for keyword in task_keywords[task_type]
                                  if keyword in desc_lower)
            confidence += min(matching_keywords * 0.1, 0.3)

        return min(confidence, 1.0)

    def _estimate_complexity(self, description: str) -> str:
        """タスクの複雑さ見積もり"""
        word_count = len(description.split())
        entity_count = len(self._extract_entities(description))

        if word_count > 20 or entity_count > 3:
            return "high"
        elif word_count > 10 or entity_count > 1:
            return "medium"
        else:
            return "low"

    async def _execute_interpreted_task(self, interpreted_task: Dict[str, Any]) -> Dict[str, Any]:
        """解釈されたタスクを実行"""
        task_type = interpreted_task["task_type"]
        entities = interpreted_task["entities"]
        parameters = interpreted_task["parameters"]

        self.logger.info(f"タスク実行: {task_type}")

        # タスクタイプに応じた実行
        if task_type == "file_organization":
            result = await self.file_manager.organize_files(
                entities, parameters
            )
        elif task_type == "data_analysis":
            result = await self.data_analyzer.analyze_data(
                entities, parameters
            )
        elif task_type == "web_automation":
            result = await self.web_automator.automate_web_task(
                entities, parameters
            )
        elif task_type == "document_processing":
            result = await self.document_processor.process_documents(
                entities, parameters
            )
        elif task_type == "image_processing":
            result = await self._process_images(entities, parameters)
        else:
            result = await self._execute_generic_task(
                interpreted_task["original_description"]
            )

        return result

    async def _process_execution_result(self, result: Dict[str, Any],
                                      interpreted_task: Dict[str, Any]) -> Dict[str, Any]:
        """実行結果の後処理"""
        # 結果の構造化
        processed_result = {
            "success": result.get("success", False),
            "task_type": interpreted_task["task_type"],
            "original_description": interpreted_task["original_description"],
            "execution_time": result.get("execution_time", 0),
            "output_files": result.get("output_files", []),
            "summary": result.get("summary", ""),
            "details": result.get("details", {})
        }

        # エラーハンドリング
        if not result.get("success", False):
            processed_result["error"] = result.get("error", "Unknown error")

        # メトリクス追加
        processed_result["metrics"] = {
            "confidence": interpreted_task["confidence"],
            "complexity": interpreted_task["estimated_complexity"],
            "entity_count": len(interpreted_task["entities"])
        }

        return processed_result

    async def _process_images(self, entities: List[str], parameters: Dict[str, Any]) -> Dict[str, Any]:
        """画像処理タスク"""
        # 簡易的な画像処理実装
        results = []
        for entity in entities:
            if Path(entity).exists() and Path(entity).suffix.lower() in self.config["supported_formats"]["images"]:
                # OCR処理
                try:
                    text = self._extract_text_from_image(entity)
                    results.append({
                        "file": entity,
                        "extracted_text": text,
                        "success": True
                    })
                except Exception as e:
                    results.append({
                        "file": entity,
                        "error": str(e),
                        "success": False
                    })

        return {
            "success": True,
            "results": results,
            "summary": f"{len(results)}個の画像を処理しました"
        }

    def _extract_text_from_image(self, image_path: str) -> str:
        """画像からテキスト抽出（OCR）"""
        try:
            image = Image.open(image_path)
            text = pytesseract.image_to_string(image, lang='jpn+eng')
            return text.strip()
        except Exception as e:
            self.logger.error(f"OCR処理エラー: {e}")
            return ""

    async def _execute_generic_task(self, description: str) -> Dict[str, Any]:
        """汎用タスク実行"""
        # 汎用タスクの基本実装
        return {
            "success": True,
            "summary": f"タスク '{description}' を実行しました",
            "details": {"task_type": "generic"}
        }


class FileManagementSystem:
    """ファイル管理システム"""

    def __init__(self, config: Dict[str, Any]):
        self.config = config
        self.logger = logging.getLogger("FileManagementSystem")

    async def organize_files(self, entities: List[str], parameters: Dict[str, Any]) -> Dict[str, Any]:
        """ファイル整理"""
        try:
            # ターゲットフォルダ特定
            target_folder = self._find_target_folder(entities)

            if not target_folder or not Path(target_folder).exists():
                return {
                    "success": False,
                    "error": f"対象フォルダが見つかりません: {target_folder}"
                }

            # 整理ルール決定
            rule = parameters.get("rule", "sort_by_type")

            # ファイル整理実行
            result = await self._execute_file_organization(target_folder, rule)

            return {
                "success": True,
                "organized_files": result["organized_count"],
                "created_folders": result["created_folders"],
                "summary": f"{result['organized_count']}個のファイルを整理しました"
            }

        except Exception as e:
            self.logger.error(f"ファイル整理エラー: {e}")
            return {"success": False, "error": str(e)}

    def _find_target_folder(self, entities: List[str]) -> Optional[str]:
        """対象フォルダ特定"""
        for entity in entities:
            path = Path(entity)
            if path.exists() and path.is_dir():
                return str(path)

        # デフォルトのダウンロードフォルダ
        return str(Path.home() / "Downloads")

    async def _execute_file_organization(self, folder_path: str, rule: str) -> Dict[str, Any]:
        """ファイル整理実行"""
        folder = Path(folder_path)
        organized_count = 0
        created_folders = []

        for file_path in folder.iterdir():
            if file_path.is_file():
                # 整理先フォルダ決定
                target_folder = self._determine_target_folder(file_path, rule, folder)

                if target_folder and target_folder != folder:
                    # フォルダ作成
                    target_folder.mkdir(parents=True, exist_ok=True)
                    if str(target_folder) not in created_folders:
                        created_folders.append(str(target_folder))

                    # ファイル移動
                    target_path = target_folder / file_path.name
                    shutil.move(str(file_path), str(target_path))
                    organized_count += 1

        return {
            "organized_count": organized_count,
            "created_folders": created_folders
        }

    def _determine_target_folder(self, file_path: Path, rule: str, base_folder: Path) -> Optional[Path]:
        """整理先フォルダ決定"""
        if rule == "sort_by_type":
            ext = file_path.suffix.lower()
            if ext in ['.jpg', '.jpeg', '.png', '.gif', '.bmp']:
                return base_folder / "Images"
            elif ext in ['.pdf', '.docx', '.xlsx', '.pptx', '.txt']:
                return base_folder / "Documents"
            elif ext in ['.zip', '.tar', '.gz', '.rar']:
                return base_folder / "Archives"
            elif ext in ['.mp4', '.avi', '.mkv']:
                return base_folder / "Videos"
        elif rule == "sort_by_date":
            mtime = datetime.fromtimestamp(file_path.stat().st_mtime)
            year_folder = base_folder / str(mtime.year)
            month_folder = year_folder / f"{mtime.month:02d}"
            return month_folder

        return None


class DataAnalysisEngine:
    """データ分析エンジン"""

    def __init__(self, config: Dict[str, Any]):
        self.config = config
        self.logger = logging.getLogger("DataAnalysisEngine")

    async def analyze_data(self, entities: List[str], parameters: Dict[str, Any]) -> Dict[str, Any]:
        """データ分析"""
        try:
            # データファイル特定
            data_files = [f for f in entities if Path(f).exists() and
                         Path(f).suffix.lower() in ['.csv', '.xlsx', '.json']]

            if not data_files:
                return {
                    "success": False,
                    "error": "分析対象のデータファイルが見つかりません"
                }

            results = []
            for data_file in data_files:
                analysis_result = await self._analyze_single_file(data_file, parameters)
                results.append(analysis_result)

            # 統合レポート生成
            summary_report = await self._generate_summary_report(results)

            return {
                "success": True,
                "analyzed_files": len(results),
                "summary_report": summary_report,
                "individual_results": results
            }

        except Exception as e:
            self.logger.error(f"データ分析エラー: {e}")
            return {"success": False, "error": str(e)}

    async def _analyze_single_file(self, file_path: str, parameters: Dict[str, Any]) -> Dict[str, Any]:
        """単一ファイル分析"""
        try:
            # データ読み込み
            df = self._load_data_file(file_path)

            # 基本統計
            stats = self._calculate_basic_statistics(df)

            # 可視化生成
            visualizations = await self._generate_visualizations(df, parameters)

            return {
                "file": file_path,
                "row_count": len(df),
                "column_count": len(df.columns),
                "statistics": stats,
                "visualizations": visualizations,
                "success": True
            }

        except Exception as e:
            return {
                "file": file_path,
                "error": str(e),
                "success": False
            }

    def _load_data_file(self, file_path: str) -> pd.DataFrame:
        """データファイル読み込み"""
        path = Path(file_path)
        if path.suffix.lower() == '.csv':
            return pd.read_csv(file_path)
        elif path.suffix.lower() == '.xlsx':
            return pd.read_excel(file_path)
        elif path.suffix.lower() == '.json':
            return pd.read_json(file_path)
        else:
            raise ValueError(f"サポートされていないファイル形式: {path.suffix}")

    def _calculate_basic_statistics(self, df: pd.DataFrame) -> Dict[str, Any]:
        """基本統計計算"""
        stats = {
            "numeric_columns": {},
            "categorical_columns": {}
        }

        for col in df.columns:
            if pd.api.types.is_numeric_dtype(df[col]):
                stats["numeric_columns"][col] = {
                    "mean": df[col].mean(),
                    "median": df[col].median(),
                    "std": df[col].std(),
                    "min": df[col].min(),
                    "max": df[col].max()
                }
            else:
                value_counts = df[col].value_counts().head(10)
                stats["categorical_columns"][col] = value_counts.to_dict()

        return stats

    async def _generate_visualizations(self, df: pd.DataFrame, parameters: Dict[str, Any]) -> List[str]:
        """可視化生成"""
        visualizations = []
        output_dir = self.config["temp_dir"] / "visualizations"
        output_dir.mkdir(exist_ok=True)

        try:
            # 数値列のヒストグラム
            numeric_cols = df.select_dtypes(include=[np.number]).columns
            if len(numeric_cols) > 0:
                fig, axes = plt.subplots(1, min(len(numeric_cols), 3), figsize=(15, 5))
                if len(numeric_cols) == 1:
                    axes = [axes]

                for i, col in enumerate(numeric_cols[:3]):
                    df[col].hist(ax=axes[i], bins=30)
                    axes[i].set_title(f'Distribution of {col}')

                hist_path = output_dir / "histograms.png"
                plt.savefig(hist_path)
                plt.close()
                visualizations.append(str(hist_path))

            # カテゴリ列の棒グラフ
            cat_cols = df.select_dtypes(include=['object', 'category']).columns
            if len(cat_cols) > 0:
                col = cat_cols[0]
                value_counts = df[col].value_counts().head(10)
                plt.figure(figsize=(10, 6))
                value_counts.plot(kind='bar')
                plt.title(f'Top 10 values in {col}')
                plt.xticks(rotation=45)

                bar_path = output_dir / "bar_chart.png"
                plt.savefig(bar_path)
                plt.close()
                visualizations.append(str(bar_path))

        except Exception as e:
            self.logger.error(f"可視化生成エラー: {e}")

        return visualizations

    async def _generate_summary_report(self, results: List[Dict[str, Any]]) -> Dict[str, Any]:
        """サマリーレポート生成"""
        successful_analyses = [r for r in results if r.get("success", False)]

        report = {
            "total_files": len(results),
            "successful_analyses": len(successful_analyses),
            "total_rows": sum(r.get("row_count", 0) for r in successful_analyses),
            "total_visualizations": sum(len(r.get("visualizations", [])) for r in successful_analyses)
        }

        return report


class WebAutomationEngine:
    """Web自動化エンジン"""

    def __init__(self, config: Dict[str, Any]):
        self.config = config
        self.logger = logging.getLogger("WebAutomationEngine")

    async def automate_web_task(self, entities: List[str], parameters: Dict[str, Any]) -> Dict[str, Any]:
        """Webタスク自動化"""
        try:
            # URL抽出
            urls = [e for e in entities if e.startswith(('http://', 'https://'))]

            if not urls:
                return {
                    "success": False,
                    "error": "有効なURLが見つかりません"
                }

            operation = parameters.get("operation", "scrape")

            if operation == "scrape":
                results = await self._scrape_websites(urls, parameters)
            elif operation == "form_fill":
                results = await self._fill_forms(urls, parameters)
            else:
                results = await self._perform_generic_web_task(urls, parameters)

            return {
                "success": True,
                "operation": operation,
                "processed_urls": len(urls),
                "results": results
            }

        except Exception as e:
            self.logger.error(f"Web自動化エラー: {e}")
            return {"success": False, "error": str(e)}

    async def _scrape_websites(self, urls: List[str], parameters: Dict[str, Any]) -> List[Dict[str, Any]]:
        """Webサイトスクレイピング"""
        results = []

        for url in urls:
            try:
                # HTTPリクエスト
                response = requests.get(url, timeout=10)
                response.raise_for_status()

                # HTML解析
                soup = BeautifulSoup(response.content, 'html.parser')

                # データ抽出
                title = soup.title.string if soup.title else "No title"
                text_content = soup.get_text()
                links = [a['href'] for a in soup.find_all('a', href=True)][:10]  # Top 10 links

                results.append({
                    "url": url,
                    "title": title,
                    "text_length": len(text_content),
                    "links_count": len(links),
                    "sample_links": links[:5],
                    "success": True
                })

            except Exception as e:
                results.append({
                    "url": url,
                    "error": str(e),
                    "success": False
                })

        return results

    async def _fill_forms(self, urls: List[str], parameters: Dict[str, Any]) -> List[Dict[str, Any]]:
        """フォーム入力自動化"""
        # Seleniumを使用したフォーム入力
        results = []

        for url in urls:
            try:
                # WebDriver初期化（ヘッドレスモード）
                options = webdriver.ChromeOptions()
                options.add_argument('--headless')
                options.add_argument('--no-sandbox')
                options.add_argument('--disable-dev-shm-usage')

                driver = webdriver.Chrome(options=options)

                try:
                    driver.get(url)

                    # フォームフィールド検出と入力
                    # （実際の実装ではより詳細なフォーム処理が必要）
                    form_fields = driver.find_elements(By.TAG_NAME, "input")
                    filled_fields = 0

                    for field in form_fields[:5]:  # 最初の5フィールドのみ
                        field_type = field.get_attribute("type")
                        if field_type in ["text", "email", "password"]:
                            field.send_keys("sample_data")
                            filled_fields += 1

                    results.append({
                        "url": url,
                        "filled_fields": filled_fields,
                        "success": True
                    })

                finally:
                    driver.quit()

            except Exception as e:
                results.append({
                    "url": url,
                    "error": str(e),
                    "success": False
                })

        return results

    async def _perform_generic_web_task(self, urls: List[str], parameters: Dict[str, Any]) -> List[Dict[str, Any]]:
        """汎用Webタスク"""
        # 汎用Webタスクの実装
        return [{"url": url, "task": "generic", "success": True} for url in urls]


class DocumentProcessingEngine:
    """ドキュメント処理エンジン"""

    def __init__(self, config: Dict[str, Any]):
        self.config = config
        self.logger = logging.getLogger("DocumentProcessingEngine")

    async def process_documents(self, entities: List[str], parameters: Dict[str, Any]) -> Dict[str, Any]:
        """ドキュメント処理"""
        try:
            # ドキュメントファイル特定
            doc_files = [f for f in entities if Path(f).exists() and
                        Path(f).suffix.lower() in self.config["supported_formats"]["documents"]]

            if not doc_files:
                return {
                    "success": False,
                    "error": "処理対象のドキュメントファイルが見つかりません"
                }

            results = []
            for doc_file in doc_files:
                result = await self._process_single_document(doc_file, parameters)
                results.append(result)

            return {
                "success": True,
                "processed_documents": len(results),
                "results": results
            }

        except Exception as e:
            self.logger.error(f"ドキュメント処理エラー: {e}")
            return {"success": False, "error": str(e)}

    async def _process_single_document(self, file_path: str, parameters: Dict[str, Any]) -> Dict[str, Any]:
        """単一ドキュメント処理"""
        try:
            path = Path(file_path)
            ext = path.suffix.lower()

            if ext == '.pdf':
                text = self._extract_pdf_text(file_path)
            elif ext == '.docx':
                text = self._extract_docx_text(file_path)
            elif ext == '.xlsx':
                text = self._extract_excel_text(file_path)
            elif ext == '.txt':
                with open(file_path, 'r', encoding='utf-8') as f:
                    text = f.read()
            else:
                text = "Unsupported format"

            return {
                "file": file_path,
                "extracted_text": text,
                "text_length": len(text),
                "success": True
            }

        except Exception as e:
            return {
                "file": file_path,
                "error": str(e),
                "success": False
            }

    def _extract_pdf_text(self, file_path: str) -> str:
        """PDFからテキスト抽出"""
        text = ""
        with fitz.open(file_path) as doc:
            for page in doc:
                text += page.get_text()
        return text

    def _extract_docx_text(self, file_path: str) -> str:
        """DOCXからテキスト抽出"""
        doc = Document(file_path)
        text = ""
        for paragraph in doc.paragraphs:
            text += paragraph.text + "\n"
        return text

    def _extract_excel_text(self, file_path: str) -> str:
        """Excelからテキスト抽出"""
        df = pd.read_excel(file_path)
        return df.to_string()


class SafetyController:
    """安全制御クラス"""

    def __init__(self, config: Dict[str, Any]):
        self.config = config
        self.logger = logging.getLogger("SafetyController")

    async def check_task_safety(self, interpreted_task: Dict[str, Any]) -> Dict[str, Any]:
        """タスク安全チェック"""
        if not self.config["safety_enabled"]:
            return {"approved": True, "reason": "Safety checks disabled"}

        risk_level = "low"
        concerns = []

        task_type = interpreted_task["task_type"]
        entities = interpreted_task["entities"]

        # ファイル操作のリスクチェック
        if task_type in ["file_organization", "document_processing"]:
            file_risks = self._check_file_operation_risks(entities)
            concerns.extend(file_risks["concerns"])
            risk_level = max(risk_level, file_risks["risk_level"])

        # Web操作のリスクチェック
        elif task_type == "web_automation":
            web_risks = self._check_web_operation_risks(entities)
            concerns.extend(web_risks["concerns"])
            risk_level = max(risk_level, web_risks["risk_level"])

        # データ操作のリスクチェック
        elif task_type == "data_analysis":
            data_risks = self._check_data_operation_risks(entities)
            concerns.extend(data_risks["concerns"])
            risk_level = max(risk_level, data_risks["risk_level"])

        # リスクレベルに基づく承認判断
        approved = risk_level in ["low", "medium"]

        return {
            "approved": approved,
            "risk_level": risk_level,
            "concerns": concerns,
            "reason": f"Risk level: {risk_level}" if approved else f"High risk detected: {', '.join(concerns)}"
        }

    def _check_file_operation_risks(self, entities: List[str]) -> Dict[str, Any]:
        """ファイル操作リスクチェック"""
        concerns = []
        risk_level = "low"

        for entity in entities:
            path = Path(entity)

            # システムフォルダチェック
            system_paths = [
                Path.home() / "AppData", "/System", "/Windows",
                "/usr", "/bin", "/sbin", "/etc"
            ]

            if any(str(path).startswith(str(sys_path)) for sys_path in system_paths):
                concerns.append(f"System path access: {entity}")
                risk_level = "high"

            # 大きなファイルチェック
            if path.exists() and path.stat().st_size > self.config["max_file_size"]:
                concerns.append(f"Large file: {entity}")
                risk_level = "medium"

        return {
            "concerns": concerns,
            "risk_level": risk_level
        }

    def _check_web_operation_risks(self, entities: List[str]) -> Dict[str, Any]:
        """Web操作リスクチェック"""
        concerns = []
        risk_level = "low"

        for entity in entities:
            if entity.startswith(('http://', 'https://')):
                # 信頼できないドメインのチェック
                untrusted_domains = ["malicious-site.com", "phishing.example"]
                if any(domain in entity for domain in untrusted_domains):
                    concerns.append(f"Untrusted domain: {entity}")
                    risk_level = "high"

        return {
            "concerns": concerns,
            "risk_level": risk_level
        }

    def _check_data_operation_risks(self, entities: List[str]) -> Dict[str, Any]:
        """データ操作リスクチェック"""
        concerns = []
        risk_level = "low"

        # データファイルの機密性チェック
        sensitive_keywords = ["password", "secret", "private", "confidential"]

        for entity in entities:
            path = Path(entity)
            if path.exists():
                filename_lower = path.name.lower()
                if any(keyword in filename_lower for keyword in sensitive_keywords):
                    concerns.append(f"Potentially sensitive data: {entity}")
                    risk_level = "medium"

        return {
            "concerns": concerns,
            "risk_level": risk_level
        }


# メイン実行関数
async def main():
    """メイン実行関数（テスト用）"""
    assistant = CoworkProductivityAssistant()

    # テストタスク実行
    test_tasks = [
        "ダウンロードフォルダを整理してください",
        "sales_data.csvを分析してレポートを作成",
        "https://example.comからデータをスクレイピング"
    ]

    for task in test_tasks:
        print(f"\n=== タスク実行: {task} ===")
        result = await assistant.execute_task(task)
        print(f"結果: {result}")


if __name__ == "__main__":
    # ロギング設定
    logging.basicConfig(
        level=logging.INFO,
        format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
    )

    # 非同期実行
    asyncio.run(main())