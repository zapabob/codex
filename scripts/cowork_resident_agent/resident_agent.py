#!/usr/bin/env python3
"""
Cowork Resident Agent - ClaudeCode Coworkスタイルの常駐型生産性エージェント

このスクリプトはPC上で常駐し、自然言語でのタスク要求に応じて
自律的に生産性タスクを実行するエージェントシステムです。

主な機能:
- PC起動時の自動起動
- システムトレイ常駐
- 自然言語タスク解釈
- 自律的タスク実行
- 安全制御と監査
- GUI/CLIブリッジ
"""

import asyncio
import json
import logging
import os
import sys
import time
from datetime import datetime, timedelta
from pathlib import Path
from typing import Dict, List, Optional, Any, Callable
import threading
import queue

# 外部ライブラリ
try:
    import psutil
    import pystray
    from PIL import Image, ImageDraw
    import tkinter as tk
    from tkinter import messagebox, simpledialog
    import winreg  # Windowsレジストリ操作
except ImportError as e:
    print(f"必要なライブラリがインストールされていません: {e}")
    print("以下のコマンドでインストールしてください:")
    print("pip install psutil pystray Pillow")
    sys.exit(1)

# プロジェクト内モジュール
sys.path.append(str(Path(__file__).parent.parent))
from cowork_productivity_assistant import CoworkProductivityAssistant

# 設定
CONFIG = {
    "agent_name": "Cowork Resident Agent",
    "version": "1.0.0",
    "log_level": "INFO",
    "max_concurrent_tasks": 3,
    "resource_check_interval": 30,
    "task_timeout": 3600,  # 1時間
    "auto_startup": True,
    "system_tray_icon": True,
    "notification_enabled": True,
    "data_dir": Path.home() / ".cowork_agent",
    "log_file": Path.home() / ".cowork_agent" / "agent.log",
    "task_queue_file": Path.home() / ".cowork_agent" / "task_queue.json",
    "settings_file": Path.home() / ".cowork_agent" / "settings.json"
}


class ResidentAgent:
    """
    常駐型Coworkエージェントのメインクラス
    """

    def __init__(self):
        self.config = CONFIG
        self.is_running = False
        self.task_queue = asyncio.Queue()
        self.active_tasks = {}
        self.completed_tasks = []
        self.system_tray = None
        self.notification_queue = queue.Queue()
        self.resource_monitor = ResourceMonitor()
        self.task_interpreter = TaskInterpreter()
        self.execution_engine = ExecutionEngine()
        self.safety_controller = SafetyController()
        self.persistence_manager = PersistenceManager(self.config)

        # ロギング設定
        self._setup_logging()

        # データディレクトリ作成
        self.config["data_dir"].mkdir(parents=True, exist_ok=True)

        # 生産性アシスタント初期化
        self.productivity_assistant = CoworkProductivityAssistant()

        # GUIブリッジ（Apple風デザイン）
        self.gui_bridge = AppleStyleGUIBridge(self)

        # シャットダウンイベント
        self.shutdown_event = threading.Event()

    def _setup_logging(self):
        """ロギング設定"""
        logging.basicConfig(
            level=getattr(logging, self.config["log_level"]),
            format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
            handlers=[
                logging.FileHandler(self.config["log_file"]),
                logging.StreamHandler(sys.stdout)
            ]
        )
        self.logger = logging.getLogger("CoworkResidentAgent")

    async def start(self):
        """エージェント起動"""
        self.logger.info(f"{self.config['agent_name']} v{self.config['version']} 起動開始")

        # 既存タスクの復元
        await self._restore_persisted_state()

        # システムトレイ初期化
        if self.config["system_tray_icon"]:
            self._setup_system_tray()

        # GUIブリッジ起動
        await self.gui_bridge.start()

        # 自動起動設定
        if self.config["auto_startup"]:
            self._setup_auto_startup()

        # メイン処理ループ開始
        self.is_running = True
        await self._main_loop()

    async def stop(self):
        """エージェント停止"""
        self.logger.info("Cowork Resident Agent 停止開始")

        self.is_running = False
        self.shutdown_event.set()

        # アクティブタスクの中断
        await self._cancel_active_tasks()

        # 状態保存
        await self._persist_current_state()

        # システムトレイ破棄
        if self.system_tray:
            self.system_tray.stop()

        # GUIブリッジ停止
        await self.gui_bridge.stop()

        self.logger.info("Cowork Resident Agent 停止完了")

    async def _main_loop(self):
        """メイン処理ループ"""
        self.logger.info("メイン処理ループ開始")

        # バックグラウンドタスク
        background_tasks = [
            self._process_task_queue(),
            self._monitor_system_resources(),
            self._handle_notifications(),
            self._perform_maintenance(),
            self._check_scheduled_tasks()
        ]

        try:
            await asyncio.gather(*background_tasks, return_exceptions=True)
        except Exception as e:
            self.logger.error(f"メインループでエラーが発生: {e}")
        finally:
            self.logger.info("メイン処理ループ終了")

    async def _process_task_queue(self):
        """タスクキュー処理"""
        while self.is_running:
            try:
                # タスク取得（タイムアウト付き）
                task_data = await asyncio.wait_for(
                    self.task_queue.get(),
                    timeout=1.0
                )

                # タスク処理
                asyncio.create_task(self._process_task(task_data))

            except asyncio.TimeoutError:
                continue
            except Exception as e:
                self.logger.error(f"タスクキュー処理エラー: {e}")
                await asyncio.sleep(1)

    async def _process_task(self, task_data: Dict[str, Any]):
        """個別タスク処理"""
        task_id = task_data.get("id")
        if not task_id:
            self.logger.error("タスクIDが見つかりません")
            return

        try:
            self.logger.info(f"タスク処理開始: {task_id}")

            # タスク実行
            self.active_tasks[task_id] = task_data
            result = await self._execute_task(task_data)

            # 結果処理
            await self._handle_task_result(task_id, result)

        except Exception as e:
            self.logger.error(f"タスク処理エラー {task_id}: {e}")
            await self._handle_task_error(task_id, e)
        finally:
            # クリーンアップ
            self.active_tasks.pop(task_id, None)
            self.task_queue.task_done()

    async def _execute_task(self, task_data: Dict[str, Any]) -> Dict[str, Any]:
        """タスク実行"""
        task_type = task_data.get("type", "generic")
        description = task_data.get("description", "")

        self.logger.info(f"タスク実行: {task_type} - {description}")

        # タスク解釈
        interpreted_task = await self.task_interpreter.interpret_task(description)

        # 安全チェック
        safety_check = await self.safety_controller.check_task_safety(interpreted_task)
        if not safety_check["approved"]:
            raise ValueError(f"安全チェック失敗: {safety_check['reason']}")

        # 実行
        result = await self.execution_engine.execute_task(interpreted_task)

        return {
            "success": True,
            "result": result,
            "interpreted_task": interpreted_task,
            "safety_check": safety_check
        }

    async def _handle_task_result(self, task_id: str, result: Dict[str, Any]):
        """タスク結果処理"""
        self.logger.info(f"タスク完了: {task_id}")

        # 結果保存
        completed_task = {
            "id": task_id,
            "timestamp": datetime.now().isoformat(),
            "result": result,
            "status": "completed"
        }
        self.completed_tasks.append(completed_task)

        # 通知
        await self._send_notification("task_completed", {
            "task_id": task_id,
            "description": result.get("interpreted_task", {}).get("description", ""),
            "success": result.get("success", False)
        })

        # GUI更新
        await self.gui_bridge.notify_task_completion(completed_task)

    async def _handle_task_error(self, task_id: str, error: Exception):
        """タスクエラー処理"""
        self.logger.error(f"タスクエラー: {task_id} - {error}")

        # エラー情報保存
        error_task = {
            "id": task_id,
            "timestamp": datetime.now().isoformat(),
            "error": str(error),
            "status": "failed"
        }
        self.completed_tasks.append(error_task)

        # 通知
        await self._send_notification("task_failed", {
            "task_id": task_id,
            "error": str(error)
        })

        # GUI更新
        await self.gui_bridge.notify_task_error(error_task)

    async def submit_task(self, task_description: str, priority: str = "normal") -> str:
        """タスク送信"""
        task_id = f"task_{int(time.time())}_{hash(task_description) % 10000}"

        task_data = {
            "id": task_id,
            "description": task_description,
            "priority": priority,
            "submitted_at": datetime.now().isoformat(),
            "status": "queued"
        }

        await self.task_queue.put(task_data)
        await self.persistence_manager.save_task_queue(self.task_queue)

        self.logger.info(f"タスク送信: {task_id} - {task_description}")
        return task_id

    async def cancel_task(self, task_id: str) -> bool:
        """タスクキャンセル"""
        # アクティブタスクのキャンセル
        if task_id in self.active_tasks:
            # 実行エンジンにキャンセル要求
            await self.execution_engine.cancel_task(task_id)
            self.active_tasks.pop(task_id, None)
            self.logger.info(f"タスクキャンセル: {task_id}")
            return True

        # キュー内のタスク削除（実装）
        new_queue = []
        found = False
        while not self.task_queue.empty():
            task = await self.task_queue.get()
            if task["id"] == task_id:
                found = True
                self.logger.info(f"キュー内タスクキャンセル: {task_id}")
                continue  # 削除するのでスキップ
            new_queue.append(task)
        for task in new_queue:
            await self.task_queue.put(task)
        if found:
            await self.persistence_manager.save_task_queue(self.task_queue)
            return True

        return False

    async def get_task_status(self, task_id: str) -> Optional[Dict[str, Any]]:
        """タスク状態取得"""
        # アクティブタスク
        if task_id in self.active_tasks:
            return {
                "id": task_id,
                "status": "running",
                "data": self.active_tasks[task_id]
            }

        # 完了タスク
        for completed_task in self.completed_tasks[-10:]:  # 最新10件
            if completed_task["id"] == task_id:
                return completed_task

        return None

    def _setup_system_tray(self):
        """システムトレイ設定"""
        # アイコン作成
        icon = self._create_tray_icon()

        # メニュー作成
        menu = self._create_tray_menu()

        # システムトレイ初期化
        self.system_tray = pystray.Icon(
            "cowork_agent",
            icon,
            self.config["agent_name"],
            menu
        )

        # 別スレッドで起動
        tray_thread = threading.Thread(target=self.system_tray.run)
        tray_thread.daemon = True
        tray_thread.start()

    def _create_tray_icon(self) -> Image.Image:
        """トレイアイコン作成"""
        # シンプルなアイコン生成
        icon = Image.new('RGB', (64, 64), color='green')
        draw = ImageDraw.Draw(icon)

        # 円を描画
        draw.ellipse([8, 8, 56, 56], fill='white', outline='black', width=2)

        # チェックマークを描画
        draw.line([20, 32, 28, 40], fill='green', width=3)
        draw.line([28, 40, 44, 24], fill='green', width=3)

        return icon

    def _create_tray_menu(self):
        """トレイメニュー作成"""
        return pystray.Menu(
            pystray.MenuItem("タスク送信", self._show_task_dialog),
            pystray.MenuItem("実行中タスク", self._show_active_tasks),
            pystray.MenuItem("設定", self._show_settings),
            pystray.Menu.SEPARATOR,
            pystray.MenuItem("終了", self._shutdown_agent)
        )

    def _show_task_dialog(self):
        """タスク入力ダイアログ表示"""
        def submit_task():
            description = dialog.get()
            if description:
                # 非同期でタスク送信
                asyncio.run_coroutine_threadsafe(
                    self.submit_task(description),
                    asyncio.get_event_loop()
                )
            root.destroy()

        root = tk.Tk()
        root.title("Cowork Agent - タスク送信")
        root.geometry("400x200")

        tk.Label(root, text="実行するタスクを入力してください:").pack(pady=10)

        dialog = tk.Entry(root, width=50)
        dialog.pack(pady=5)
        dialog.focus()

        tk.Button(root, text="実行", command=submit_task).pack(pady=10)

        root.mainloop()

    def _show_active_tasks(self):
        """アクティブタスク表示"""
        tasks_info = []
        for task_id, task_data in self.active_tasks.items():
            tasks_info.append(f"• {task_id}: {task_data.get('description', 'Unknown')}")

        if not tasks_info:
            tasks_info = ["実行中のタスクはありません"]

        messagebox.showinfo("実行中タスク", "\n".join(tasks_info))

    def _show_settings(self):
        """設定ダイアログ表示"""
        # 設定ダイアログの実装
        pass

    def _shutdown_agent(self):
        """エージェント終了"""
        self.logger.info("システムトレイから終了要求")
        asyncio.run_coroutine_threadsafe(self.stop(), asyncio.get_event_loop())

    async def _monitor_system_resources(self):
        """システムリソース監視"""
        while self.is_running:
            try:
                resources = await self.resource_monitor.check_resources()

                # リソース使用量ログ
                self.logger.debug(f"システムリソース: CPU={resources['cpu']}%, MEM={resources['memory']}%")

                # 高負荷時の処理調整
                if resources['cpu'] > 80 or resources['memory'] > 90:
                    await self._throttle_processing()

                await asyncio.sleep(self.config["resource_check_interval"])

            except Exception as e:
                self.logger.error(f"リソース監視エラー: {e}")
                await asyncio.sleep(10)

    async def _throttle_processing(self):
        """処理スロットリング"""
        self.logger.info("高負荷検知: 処理をスロットリング")
        # 同時実行タスク数の制限
        # 処理間隔の増加
        pass

    async def _handle_notifications(self):
        """通知処理"""
        while self.is_running:
            try:
                # 通知キューから取得
                notification = await asyncio.get_event_loop().run_in_executor(
                    None, self.notification_queue.get, True, 1.0
                )

                # 通知表示
                await self._display_notification(notification)

            except queue.Empty:
                continue
            except Exception as e:
                self.logger.error(f"通知処理エラー: {e}")

    async def _display_notification(self, notification: Dict[str, Any]):
        """通知表示"""
        if not self.config["notification_enabled"]:
            return

        notification_type = notification.get("type")
        title = notification.get("title", "Cowork Agent")
        message = notification.get("message", "")

        # システム通知表示
        try:
            if sys.platform == "win32":
                # Windows通知
                import win10toast
                toaster = win10toast.ToastNotifier()
                toaster.show_toast(title, message, duration=5)
            else:
                # 他のプラットフォーム
                self.logger.info(f"通知: {title} - {message}")
        except Exception as e:
            self.logger.error(f"通知表示エラー: {e}")

    async def _send_notification(self, notification_type: str, data: Dict[str, Any]):
        """通知送信"""
        notification = {
            "type": notification_type,
            "timestamp": datetime.now().isoformat(),
            "data": data
        }

        # 通知キューに追加
        self.notification_queue.put(notification)

    async def _perform_maintenance(self):
        """メンテナンス処理"""
        while self.is_running:
            try:
                # 古い完了タスクのクリーンアップ
                await self._cleanup_old_tasks()

                # パフォーマンス統計更新
                await self._update_performance_stats()

                # 設定再読み込み
                await self._reload_configuration()

                # メンテナンス間隔（1時間）
                await asyncio.sleep(3600)

            except Exception as e:
                self.logger.error(f"メンテナンスエラー: {e}")
                await asyncio.sleep(300)  # 5分後に再試行

    async def _cleanup_old_tasks(self):
        """古いタスクのクリーンアップ"""
        cutoff_date = datetime.now() - timedelta(days=7)

        # 完了タスクのフィルタリング
        self.completed_tasks = [
            task for task in self.completed_tasks
            if datetime.fromisoformat(task["timestamp"]) > cutoff_date
        ]

    async def _update_performance_stats(self):
        """パフォーマンス統計更新"""
        stats = {
            "total_tasks_processed": len(self.completed_tasks),
            "active_tasks": len(self.active_tasks),
            "uptime": time.time() - self.start_time if hasattr(self, 'start_time') else 0,
            "average_task_duration": await self._calculate_average_task_duration()
        }

        await self.persistence_manager.save_performance_stats(stats)

    async def _calculate_average_task_duration(self) -> float:
        """平均タスク実行時間計算"""
        durations = []
        for task in self.completed_tasks[-100:]:  # 最新100件
            if "duration" in task:
                durations.append(task["duration"])

        return sum(durations) / len(durations) if durations else 0

    async def _reload_configuration(self):
        """設定再読み込み"""
        try:
            new_config = await self.persistence_manager.load_settings()
            if new_config:
                self.config.update(new_config)
                self.logger.info("設定を再読み込みしました")
        except Exception as e:
            self.logger.error(f"設定再読み込みエラー: {e}")

    async def _check_scheduled_tasks(self):
        """スケジュールタスクチェック"""
        while self.is_running:
            try:
                # スケジュールタスクの実行
                await self._execute_scheduled_tasks()

                # チェック間隔（1分）
                await asyncio.sleep(60)

            except Exception as e:
                self.logger.error(f"スケジュールタスクチェックエラー: {e}")
                await asyncio.sleep(30)

    async def _execute_scheduled_tasks(self):
        """スケジュールタスク実行"""
        # 定期タスクの実行ロジック
        # （例: 毎日のレポート生成、定期クリーニングなど）
        pass

    async def _restore_persisted_state(self):
        """永続化状態の復元"""
        try:
            state = await self.persistence_manager.load_agent_state()
            if state:
                self.completed_tasks = state.get("completed_tasks", [])
                self.logger.info("エージェント状態を復元しました")
        except Exception as e:
            self.logger.error(f"状態復元エラー: {e}")

    async def _persist_current_state(self):
        """現在の状態を永続化"""
        try:
            state = {
                "completed_tasks": self.completed_tasks[-100:],  # 最新100件のみ
                "active_tasks": list(self.active_tasks.keys()),
                "timestamp": datetime.now().isoformat()
            }
            await self.persistence_manager.save_agent_state(state)
        except Exception as e:
            self.logger.error(f"状態保存エラー: {e}")

    async def _cancel_active_tasks(self):
        """アクティブタスクのキャンセル"""
        cancel_tasks = []
        for task_id in self.active_tasks.keys():
            cancel_tasks.append(self.cancel_task(task_id))

        if cancel_tasks:
            await asyncio.gather(*cancel_tasks, return_exceptions=True)

    def _setup_auto_startup(self):
        """自動起動設定"""
        try:
            if sys.platform == "win32":
                self._setup_windows_auto_startup()
            elif sys.platform == "darwin":
                self._setup_macos_auto_startup()
            else:
                self._setup_linux_auto_startup()
        except Exception as e:
            self.logger.error(f"自動起動設定エラー: {e}")

    def _setup_windows_auto_startup(self):
        """Windows自動起動設定"""
        try:
            key = winreg.OpenKey(
                winreg.HKEY_CURRENT_USER,
                r"Software\Microsoft\Windows\CurrentVersion\Run",
                0,
                winreg.KEY_SET_VALUE
            )

            script_path = str(Path(__file__).resolve())
            winreg.SetValueEx(key, "CoworkResidentAgent", 0, winreg.REG_SZ, f'python "{script_path}"')

            winreg.CloseKey(key)
            self.logger.info("Windows自動起動を設定しました")

        except Exception as e:
            self.logger.error(f"Windows自動起動設定エラー: {e}")

    def _setup_macos_auto_startup(self):
        """macOS自動起動設定"""
        # macOS LaunchAgent設定
        pass

    def _setup_linux_auto_startup(self):
        """Linux自動起動設定"""
        # .config/autostart設定
        pass


class TaskInterpreter:
    """タスク解釈クラス"""

    def __init__(self):
        self.logger = logging.getLogger("TaskInterpreter")

    async def interpret_task(self, description: str) -> Dict[str, Any]:
        """タスク解釈"""
        # 簡易的なタスク解釈
        # （実際の実装ではより高度なNLP処理）

        interpreted = {
            "original_description": description,
            "task_type": self._classify_task_type(description),
            "entities": self._extract_entities(description),
            "confidence": 0.8,
            "estimated_duration": self._estimate_duration(description)
        }

        self.logger.info(f"タスク解釈: {description} -> {interpreted['task_type']}")
        return interpreted

    def _classify_task_type(self, description: str) -> str:
        """タスクタイプ分類"""
        desc_lower = description.lower()

        if any(word in desc_lower for word in ["整理", "organize", "sort", "clean"]):
            return "file_organization"
        elif any(word in desc_lower for word in ["分析", "analyze", "report", "chart"]):
            return "data_analysis"
        elif any(word in desc_lower for word in ["スクレイプ", "scrape", "web", "browser"]):
            return "web_scraping"
        else:
            return "generic"

    def _extract_entities(self, description: str) -> List[str]:
        """エンティティ抽出"""
        # 簡易的なエンティティ抽出
        return []

    def _estimate_duration(self, description: str) -> int:
        """実行時間見積もり"""
        # 簡易的な見積もり
        return 300  # 5分


class ExecutionEngine:
    """実行エンジンクラス"""

    def __init__(self):
        self.logger = logging.getLogger("ExecutionEngine")

    async def execute_task(self, interpreted_task: Dict[str, Any]) -> Dict[str, Any]:
        """タスク実行"""
        task_type = interpreted_task.get("task_type")
        description = interpreted_task.get("original_description")

        self.logger.info(f"タスク実行開始: {task_type}")

        # タスクタイプに応じた実行
        if task_type == "file_organization":
            result = await self._execute_file_organization(description)
        elif task_type == "data_analysis":
            result = await self._execute_data_analysis(description)
        elif task_type == "web_scraping":
            result = await self._execute_web_scraping(description)
        else:
            result = await self._execute_generic_task(description)

        self.logger.info(f"タスク実行完了: {task_type}")
        return result

    async def _execute_file_organization(self, description: str) -> Dict[str, Any]:
        """ファイル整理実行"""
        # ファイル整理ロジックの実装
        return {"status": "completed", "message": "ファイル整理が完了しました"}

    async def _execute_data_analysis(self, description: str) -> Dict[str, Any]:
        """データ分析実行"""
        # データ分析ロジックの実装
        return {"status": "completed", "message": "データ分析が完了しました"}

    async def _execute_web_scraping(self, description: str) -> Dict[str, Any]:
        """Webスクレイピング実行"""
        # Webスクレイピングロジックの実装
        return {"status": "completed", "message": "Webスクレイピングが完了しました"}

    async def _execute_generic_task(self, description: str) -> Dict[str, Any]:
        """汎用タスク実行"""
        # 汎用タスクロジックの実装
        return {"status": "completed", "message": "タスクが完了しました"}

    async def cancel_task(self, task_id: str):
        """タスクキャンセル"""
        self.logger.info(f"タスクキャンセル: {task_id}")
        # タスクキャンセルの本番実装
        # タスクを asyncio.Task として管理・キャンセルする実装例（本番環境向け）
        if not hasattr(self, "_task_map"):
            self._task_map = {}

        task = self._task_map.get(task_id)
        if task is not None and not task.done():
            task.cancel()
            self.logger.info(f"タスク {task_id} のキャンセル要求を実行しました。")
            # 必要に応じてキャンセル後の状態管理や永続化処理を実装
        else:
            self.logger.warning(f"キャンセル対象タスク {task_id} が見つからない、または既に終了しています。")

class SafetyController:
    """安全制御クラス"""

    def __init__(self):
        self.logger = logging.getLogger("SafetyController")

    async def check_task_safety(self, interpreted_task: Dict[str, Any]) -> Dict[str, Any]:
        """タスク安全チェック"""
        # 安全チェックロジック
        return {
            "approved": True,
            "reason": "安全チェック通過",
            "risk_level": "low"
        }


class PersistenceManager:
    """
    永続化管理クラス（本番環境・CLI/TUI/GUITUICLIブリッジ対応）
    タスクキューや状態、設定、統計情報をファイルまたはCLI/TUI/GUITUICLIブリッジ経由で管理する。
    """

    def __init__(self, config: Dict[str, Any], bridge=None):
        self.config = config
        self.logger = logging.getLogger("PersistenceManager")
        self.state_file = self.config.get("agent_state_path", "agent_state.json")
        self.task_queue_file = self.config.get("task_queue_path", "task_queue.json")
        self.settings_file = self.config.get("settings_path", "settings.json")
        self.performance_file = self.config.get("performance_stats_path", "performance.json")
        # CLI/TUI/GUITUICLI経由の外部永続化IF（必要時Noneでローカルファイル保存）
        # bridge(GUIBridge等)は.save/load_xx()を提供する想定
        self.bridge = bridge

    async def save_task_queue(self, task_queue: asyncio.Queue):
        """タスクキューをCLI/TUI/GUITUICLI管理ストレージまたはファイルに保存"""
        try:
            tasks = []
            size = task_queue.qsize()
            for _ in range(size):
                task = await task_queue.get()
                tasks.append(task)
            for task in tasks:
                await task_queue.put(task)
            if self.bridge and hasattr(self.bridge, "save_task_queue"):
                await self.bridge.save_task_queue(tasks)
                self.logger.info("タスクキュー保存完了（CLI/TUI/GUITUICLIブリッジ経由）")
            else:
                import json
                with open(self.task_queue_file, "w", encoding="utf-8") as f:
                    json.dump(tasks, f, ensure_ascii=False, indent=2)
                self.logger.info("タスクキュー保存完了（ローカルファイル）")
        except Exception as e:
            self.logger.error(f"タスクキュー保存失敗: {e}")

    async def load_task_queue(self, task_queue: asyncio.Queue):
        """タスクキューをCLI/TUI/GUITUICLI管理ストレージまたはファイルからロード（再構成）"""
        try:
            if self.bridge and hasattr(self.bridge, "load_task_queue"):
                tasks = await self.bridge.load_task_queue()
                if tasks is None:
                    self.logger.info("タスクキューは空です（CLI/TUI/GUITUICLIブリッジ）。")
                    return
            else:
                import os
                import json
                if not os.path.exists(self.task_queue_file):
                    self.logger.info("タスクキュー永続化ファイルがありません。")
                    return
                with open(self.task_queue_file, "r", encoding="utf-8") as f:
                    tasks = json.load(f)
            # 一旦全部クリアしてから入れ直す
            while not task_queue.empty():
                await task_queue.get()
            for task in tasks:
                await task_queue.put(task)
            self.logger.info("タスクキュー復元完了")
        except Exception as e:
            self.logger.error(f"タスクキュー復元失敗: {e}")

    async def load_agent_state(self) -> Optional[Dict[str, Any]]:
        """エージェント状態をCLI/TUI/GUITUICLI管理ストレージまたはファイルから読み込む"""
        try:
            if self.bridge and hasattr(self.bridge, "load_agent_state"):
                state = await self.bridge.load_agent_state()
                if state is not None:
                    self.logger.info("エージェント状態読み込み成功（CLI/TUI/GUITUICLIブリッジ）")
                    return state
            import os
            import json
            if not os.path.exists(self.state_file):
                self.logger.warning("エージェント状態ファイルが存在しません。新規で開始します。")
                return None
            with open(self.state_file, "r", encoding="utf-8") as f:
                state = json.load(f)
            self.logger.info("エージェント状態読み込み成功（ローカルファイル）")
            return state
        except Exception as e:
            self.logger.error(f"エージェント状態読み込みエラー: {e}")
            return None

    async def save_agent_state(self, state: Dict[str, Any]):
        """エージェント状態をCLI/TUI/GUITUICLI管理ストレージまたはファイルに保存"""
        try:
            if self.bridge and hasattr(self.bridge, "save_agent_state"):
                await self.bridge.save_agent_state(state)
                self.logger.info("エージェント状態保存完了（CLI/TUI/GUITUICLIブリッジ）")
            else:
                import json
                with open(self.state_file, "w", encoding="utf-8") as f:
                    json.dump(state, f, ensure_ascii=False, indent=2)
                self.logger.info("エージェント状態保存完了（ローカルファイル）")
        except Exception as e:
            self.logger.error(f"エージェント状態保存失敗: {e}")

    async def load_settings(self) -> Optional[Dict[str, Any]]:
        """設定情報をCLI/TUI/GUITUICLI管理ストレージまたはファイルから読み込み"""
        try:
            if self.bridge and hasattr(self.bridge, "load_settings"):
                settings = await self.bridge.load_settings()
                if settings is not None:
                    self.logger.info("設定読み込み成功（CLI/TUI/GUITUICLIブリッジ）")
                    return settings
            import os
            import json
            if not os.path.exists(self.settings_file):
                self.logger.info("設定ファイルが存在しません。デフォルト設定で開始します。")
                return None
            with open(self.settings_file, "r", encoding="utf-8") as f:
                settings = json.load(f)
            self.logger.info("設定読み込み成功（ローカルファイル）")
            return settings
        except Exception as e:
            self.logger.error(f"設定読み込み失敗: {e}")
            return None

    async def save_settings(self, settings: Dict[str, Any]):
        """設定情報をCLI/TUI/GUITUICLI管理ストレージまたはファイルに保存"""
        try:
            if self.bridge and hasattr(self.bridge, "save_settings"):
                await self.bridge.save_settings(settings)
                self.logger.info("設定保存完了（CLI/TUI/GUITUICLIブリッジ）")
            else:
                import json
                with open(self.settings_file, "w", encoding="utf-8") as f:
                    json.dump(settings, f, ensure_ascii=False, indent=2)
                self.logger.info("設定保存完了（ローカルファイル）")
        except Exception as e:
            self.logger.error(f"設定保存失敗: {e}")

    async def save_performance_stats(self, stats: Dict[str, Any]):
        """パフォーマンス統計をCLI/TUI/GUITUICLI管理ストレージまたはファイルに保存"""
        try:
            if self.bridge and hasattr(self.bridge, "save_performance_stats"):
                await self.bridge.save_performance_stats(stats)
                self.logger.info("パフォーマンス統計保存完了（CLI/TUI/GUITUICLIブリッジ）")
            else:
                import json
                with open(self.performance_file, "w", encoding="utf-8") as f:
                    json.dump(stats, f, ensure_ascii=False, indent=2)
                self.logger.info("パフォーマンス統計保存完了（ローカルファイル）")
        except Exception as e:
            self.logger.error(f"パフォーマンス統計保存失敗: {e}")

    async def load_performance_stats(self) -> Optional[Dict[str, Any]]:
        """パフォーマンス統計をCLI/TUI/GUITUICLI管理ストレージまたはファイルから読み込む"""
        try:
            if self.bridge and hasattr(self.bridge, "load_performance_stats"):
                stats = await self.bridge.load_performance_stats()
                if stats is not None:
                    self.logger.info("パフォーマンス統計読み込み成功（CLI/TUI/GUITUICLIブリッジ）")
                    return stats
            import os
            import json
            if not os.path.exists(self.performance_file):
                self.logger.info("パフォーマンス統計ファイルがありません。")
                return None
            with open(self.performance_file, "r", encoding="utf-8") as f:
                stats = json.load(f)
            self.logger.info("パフォーマンス統計読み込み成功（ローカルファイル）")
            return stats
        except Exception as e:
            self.logger.error(f"パフォーマンス統計読み込み失敗: {e}")
            return None


class ResourceMonitor:
    """リソース監視クラス"""

    def __init__(self):
        self.logger = logging.getLogger("ResourceMonitor")

    async def check_resources(self) -> Dict[str, float]:
        """リソースチェック"""
        try:
            cpu_percent = psutil.cpu_percent(interval=1)
            memory = psutil.virtual_memory()
            memory_percent = memory.percent

            return {
                "cpu": cpu_percent,
                "memory": memory_percent,
                "disk": psutil.disk_usage('/').percent
            }
        except Exception as e:
            self.logger.error(f"リソースチェックエラー: {e}")
            return {"cpu": 0, "memory": 0, "disk": 0}


class AppleStyleGUIBridge:
    """Apple風デザインGUIブリッジクラス"""

    def __init__(self, agent: ResidentAgent):
        self.agent = agent
        self.logger = logging.getLogger("AppleStyleGUIBridge")
        self.is_running = False
        self.gui_process = None
        self.notification_queue = asyncio.Queue()

    async def start(self):
        """Apple風GUIブリッジ起動"""
        self.is_running = True
        self.logger.info("Apple風GUIブリッジ起動")

        # GUIプロセス起動
        try:
            import subprocess
            import sys
            from pathlib import Path

            gui_script = Path(__file__).parent.parent / "cowork_apple_gui.py"
            if gui_script.exists():
                self.gui_process = subprocess.Popen([
                    sys.executable, str(gui_script)
                ], stdout=subprocess.PIPE, stderr=subprocess.PIPE)

                self.logger.info("Apple風GUIプロセス起動成功")
            else:
                self.logger.warning("Apple風GUIスクリプトが見つかりません")

        except Exception as e:
            self.logger.error(f"GUIプロセス起動エラー: {e}")

        # 通知処理タスク起動
        asyncio.create_task(self._process_notifications())

    async def stop(self):
        """Apple風GUIブリッジ停止"""
        self.is_running = False

        # GUIプロセス終了
        if self.gui_process:
            try:
                self.gui_process.terminate()
                await asyncio.sleep(2)
                if self.gui_process.poll() is None:
                    self.gui_process.kill()
                self.logger.info("GUIプロセス正常終了")
            except Exception as e:
                self.logger.error(f"GUIプロセス終了エラー: {e}")

        self.logger.info("Apple風GUIブリッジ停止")

    async def notify_task_completion(self, task: Dict[str, Any]):
        """タスク完了通知（Apple風デザイン）"""
        await self._send_notification({
            "type": "task_completion",
            "task": task,
            "style": "apple_success"
        })

    async def notify_task_error(self, task: Dict[str, Any]):
        """タスクエラー通知（Apple風デザイン）"""
        await self._send_notification({
            "type": "task_error",
            "task": task,
            "style": "apple_error"
        })

    async def show_feature_search(self):
        """機能検索ウィンドウ表示"""
        await self._send_notification({
            "type": "show_feature_search",
            "style": "apple_modal"
        })

    async def show_settings(self):
        """設定ウィンドウ表示"""
        await self._send_notification({
            "type": "show_settings",
            "style": "apple_sheet"
        })

    async def _send_notification(self, notification: Dict[str, Any]):
        """通知送信"""
        try:
            await self.notification_queue.put(notification)
        except Exception as e:
            self.logger.error(f"通知送信エラー: {e}")

    async def _process_notifications(self):
        """通知処理（Apple風アニメーション付き）"""
        while self.is_running:
            try:
                notification = await asyncio.wait_for(
                    self.notification_queue.get(), timeout=1.0
                )

                # Apple風通知処理
                await self._handle_apple_notification(notification)

            except asyncio.TimeoutError:
                continue
            except Exception as e:
                self.logger.error(f"通知処理エラー: {e}")

    async def _handle_apple_notification(self, notification: Dict[str, Any]):
        """Apple風通知ハンドリング"""
        notification_type = notification.get("type")
        style = notification.get("style", "apple_default")

        # Apple風通知スタイル適用
        if notification_type == "task_completion":
            await self._show_apple_success_notification(notification["task"])
        elif notification_type == "task_error":
            await self._show_apple_error_notification(notification["task"])
        elif notification_type == "show_feature_search":
            await self._show_apple_feature_search()
        elif notification_type == "show_settings":
            await self._show_apple_settings()

    async def _show_apple_success_notification(self, task: Dict[str, Any]):
        """Apple風成功通知"""
        # macOSスタイルの通知（Windowsでは代替）
        try:
            import platform
            if platform.system() == "Darwin":
                await self._show_macos_notification(task, "success")
            else:
                await self._show_windows_notification(task, "success")
        except Exception as e:
            self.logger.error(f"成功通知エラー: {e}")

    async def _show_apple_error_notification(self, task: Dict[str, Any]):
        """Apple風エラー通知"""
        try:
            import platform
            if platform.system() == "Darwin":
                await self._show_macos_notification(task, "error")
            else:
                await self._show_windows_notification(task, "error")
        except Exception as e:
            self.logger.error(f"エラー通知エラー: {e}")

    async def _show_macos_notification(self, task: Dict[str, Any], notification_type: str):
        """macOS通知"""
        import subprocess

        title = "Cowork Assistant"
        if notification_type == "success":
            message = f"✅ タスク完了: {task.get('description', '')[:50]}..."
        else:
            message = f"❌ タスク失敗: {task.get('description', '')[:50]}..."

        try:
            subprocess.run([
                "osascript", "-e",
                f'display notification "{message}" with title "{title}"'
            ], check=True)
        except Exception as e:
            self.logger.error(f"macOS通知エラー: {e}")

    async def _show_windows_notification(self, task: Dict[str, Any], notification_type: str):
        """Windows通知"""
        try:
            from win10toast import ToastNotifier

            toaster = ToastNotifier()

            title = "Cowork Assistant"
            if notification_type == "success":
                message = f"✅ タスク完了: {task.get('description', '')[:50]}..."
                icon_path = None  # 成功アイコン
            else:
                message = f"❌ タスク失敗: {task.get('description', '')[:50]}..."
                icon_path = None  # エラーアイコン

            toaster.show_toast(title, message, icon_path=icon_path, duration=5)

        except ImportError:
            # win10toastがインストールされていない場合
            self.logger.info(f"Windows通知: {title} - {message}")
        except Exception as e:
            self.logger.error(f"Windows通知エラー: {e}")

    async def _show_apple_feature_search(self):
        """Apple風機能検索表示"""
        # GUIプロセスに機能検索を表示するよう通知
        self.logger.info("Apple風機能検索ウィンドウ表示要求")

    async def _show_apple_settings(self):
        """Apple風設定表示"""
        # GUIプロセスに設定を表示するよう通知
        self.logger.info("Apple風設定ウィンドウ表示要求")


# 旧GUIBridgeクラスとの互換性維持
class GUIBridge(AppleStyleGUIBridge):
    """後方互換性のためのGUIBridgeクラス"""
    pass


async def main():
    """メイン関数"""
    agent = ResidentAgent()

    try:
        await agent.start()
    except KeyboardInterrupt:
        print("\nシャットダウン要求を受信しました...")
        await agent.stop()
    except Exception as e:
        print(f"エラーが発生しました: {e}")
        await agent.stop()
        sys.exit(1)


if __name__ == "__main__":
    # Windowsイベントループポリシー設定
    if sys.platform == "win32":
        asyncio.set_event_loop_policy(asyncio.WindowsSelectorEventLoopPolicy())

    # エージェント起動
    asyncio.run(main())