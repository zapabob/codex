#!/usr/bin/env python3
"""
Cowork Productivity Assistant - Apple Design Inspired GUI
ClaudeCode Coworkスタイルの生産性自動化GUI（Apple風デザイン）

特徴:
- macOSスタイルの洗練されたUIデザイン
- 機能検索とインテリジェント実行
- デスクトップショートカット自動作成
- タスクトレイ常駐機能
- ダークモード対応
"""

import tkinter as tk
from tkinter import ttk, messagebox, scrolledtext, filedialog
import tkinter.font as tkfont
from PIL import Image, ImageTk, ImageDraw
import threading
import time
import os
import sys
import json
import subprocess
import platform
from pathlib import Path
import asyncio
from datetime import datetime
import pystray
from pystray import MenuItem as item
import winreg  # Windows用

# プロジェクト内モジュール
sys.path.append(str(Path(__file__).parent))
from cowork_productivity_assistant import CoworkProductivityAssistant

class AppleStyleGUI:
    """Apple風デザインのCowork GUI"""

    def __init__(self):
        self.root = None
        self.assistant = CoworkProductivityAssistant()
        self.current_task = None
        self.task_history = []
        self.system_tray = None
        self.is_dark_mode = True

        # Apple風カラーパレット
        self.colors = {
            'light': {
                'bg': '#f5f5f7',
                'surface': '#ffffff',
                'primary': '#007aff',
                'secondary': '#5856d6',
                'text': '#1d1d1f',
                'text_secondary': '#86868b',
                'border': '#d1d1d6',
                'success': '#34c759',
                'warning': '#ff9500',
                'error': '#ff3b30'
            },
            'dark': {
                'bg': '#1d1d1f',
                'surface': '#2c2c2e',
                'primary': '#0a84ff',
                'secondary': '#5e5ce6',
                'text': '#f2f2f7',
                'text_secondary': '#98989d',
                'border': '#48484a',
                'success': '#30d158',
                'warning': '#ff9f0a',
                'error': '#ff453a'
            }
        }

        # Apple風フォント設定
        self.fonts = {
            'large_title': ('SF Pro Display', 28, 'bold'),
            'title1': ('SF Pro Display', 22, 'bold'),
            'title2': ('SF Pro Text', 17, 'bold'),
            'title3': ('SF Pro Text', 15, 'bold'),
            'headline': ('SF Pro Text', 13, 'bold'),
            'body': ('SF Pro Text', 13, 'normal'),
            'callout': ('SF Pro Text', 12, 'normal'),
            'subhead': ('SF Pro Text', 11, 'bold'),
            'footnote': ('SF Pro Text', 11, 'normal'),
            'caption1': ('SF Pro Text', 10, 'normal'),
            'caption2': ('SF Pro Text', 9, 'normal')
        }

    def create_main_window(self):
        """メインウィンドウ作成（Apple風デザイン）"""
        self.root = tk.Tk()
        self.root.title("Cowork Assistant")
        self.root.geometry("1200x800")
        self.root.configure(bg=self.get_color('bg'))

        # Apple風ウィンドウ設定
        self.root.overrideredirect(False)  # 標準ウィンドウ
        self.root.resizable(True, True)

        # スタイル設定
        self.setup_styles()

        # メインフレーム
        main_frame = tk.Frame(self.root, bg=self.get_color('bg'))
        main_frame.pack(fill=tk.BOTH, expand=True, padx=20, pady=20)

        # ヘッダー
        self.create_header(main_frame)

        # 検索・実行セクション
        self.create_search_section(main_frame)

        # 機能グリッド
        self.create_feature_grid(main_frame)

        # タスク履歴
        self.create_task_history(main_frame)

        # ステータスバー
        self.create_status_bar()

        # ショートカット作成
        self.create_desktop_shortcut()

        # システムトレイ設定
        self.setup_system_tray()

    def setup_styles(self):
        """Apple風スタイル設定"""
        style = ttk.Style()

        # ボタンスタイル
        style.configure('Primary.TButton',
                       font=self.fonts['body'],
                       background=self.get_color('primary'),
                       foreground='white',
                       borderwidth=0,
                       relief='flat',
                       padding=(16, 8))

        style.configure('Secondary.TButton',
                       font=self.fonts['body'],
                       background=self.get_color('secondary'),
                       foreground='white',
                       borderwidth=0,
                       relief='flat',
                       padding=(16, 8))

        # エントリースタイル
        style.configure('Search.TEntry',
                       font=self.fonts['body'],
                       borderwidth=1,
                       relief='flat',
                       padding=(12, 8))

    def create_header(self, parent):
        """Apple風ヘッダー作成"""
        header_frame = tk.Frame(parent, bg=self.get_color('bg'))
        header_frame.pack(fill=tk.X, pady=(0, 24))

        # タイトル
        title_label = tk.Label(header_frame,
                              text="Cowork Assistant",
                              font=self.fonts['large_title'],
                              bg=self.get_color('bg'),
                              fg=self.get_color('text'))
        title_label.pack(anchor=tk.W)

        # サブタイトル
        subtitle_label = tk.Label(header_frame,
                                 text="生産性向上のためのインテリジェントアシスタント",
                                 font=self.fonts['body'],
                                 bg=self.get_color('bg'),
                                 fg=self.get_color('text_secondary'))
        subtitle_label.pack(anchor=tk.W, pady=(4, 0))

    def create_search_section(self, parent):
        """検索・実行セクション作成"""
        search_frame = tk.Frame(parent, bg=self.get_color('surface'),
                               relief='flat', borderwidth=1)
        search_frame.pack(fill=tk.X, pady=(0, 24))

        # 検索ラベル
        search_label = tk.Label(search_frame,
                               text="何を自動化しますか？",
                               font=self.fonts['title2'],
                               bg=self.get_color('surface'),
                               fg=self.get_color('text'))
        search_label.pack(anchor=tk.W, padx=20, pady=(20, 12))

        # 検索入力フレーム
        input_frame = tk.Frame(search_frame, bg=self.get_color('surface'))
        input_frame.pack(fill=tk.X, padx=20, pady=(0, 20))

        # 検索エントリー
        self.search_var = tk.StringVar()
        search_entry = tk.Entry(input_frame,
                               textvariable=self.search_var,
                               font=self.fonts['body'],
                               bg=self.get_color('bg'),
                               fg=self.get_color('text'),
                               insertbackground=self.get_color('primary'),
                               relief='flat',
                               borderwidth=1)
        search_entry.pack(side=tk.LEFT, fill=tk.X, expand=True, ipady=8, ipadx=12)

        # 実行ボタン
        execute_btn = tk.Button(input_frame,
                               text="実行",
                               font=self.fonts['body'],
                               bg=self.get_color('primary'),
                               fg='white',
                               relief='flat',
                               borderwidth=0,
                               padx=24,
                               pady=8,
                               command=self.execute_task)
        execute_btn.pack(side=tk.RIGHT, padx=(12, 0))

        # キーボードショートカット設定
        self.root.bind('<Return>', lambda e: self.execute_task())

    def create_feature_grid(self, parent):
        """機能グリッド作成"""
        grid_frame = tk.Frame(parent, bg=self.get_color('bg'))
        grid_frame.pack(fill=tk.X, pady=(0, 24))

        # グリッドタイトル
        grid_title = tk.Label(grid_frame,
                             text="利用可能な機能",
                             font=self.fonts['title2'],
                             bg=self.get_color('bg'),
                             fg=self.get_color('text'))
        grid_title.pack(anchor=tk.W, pady=(0, 16))

        # 機能ボタングリッド
        features_frame = tk.Frame(grid_frame, bg=self.get_color('bg'))
        features_frame.pack(fill=tk.X)

        # 機能リスト
        features = [
            ("📁", "ファイル整理", "ファイルを自動整理・分類"),
            ("📊", "データ分析", "CSV/Excel分析とレポート生成"),
            ("🌐", "Web操作", "スクレイピングとフォーム自動入力"),
            ("📄", "文書処理", "PDF/Word/Excelのテキスト抽出"),
            ("🖼️", "画像処理", "OCRと画像分析"),
            ("🔍", "研究支援", "Web検索と情報収集"),
            ("📋", "レポート作成", "データからの自動レポート生成"),
            ("⚙️", "ワークフロー", "複雑タスクの自動実行")
        ]

        # 4列グリッド
        for i, (icon, title, desc) in enumerate(features):
            row = i // 4
            col = i % 4

            # 機能ボタン
            feature_btn = tk.Button(features_frame,
                                   text=f"{icon}\n{title}",
                                   font=self.fonts['callout'],
                                   bg=self.get_color('surface'),
                                   fg=self.get_color('text'),
                                   relief='flat',
                                   borderwidth=1,
                                   padx=16,
                                   pady=12,
                                   command=lambda t=title, d=desc: self.select_feature(t, d))
            feature_btn.grid(row=row, column=col, padx=8, pady=8, sticky='nsew')

            # ツールチップ設定
            self.create_tooltip(feature_btn, desc)

        # グリッド設定
        for i in range(4):
            features_frame.columnconfigure(i, weight=1)

    def create_task_history(self, parent):
        """タスク履歴セクション作成"""
        history_frame = tk.Frame(parent, bg=self.get_color('surface'),
                                relief='flat', borderwidth=1)
        history_frame.pack(fill=tk.BOTH, expand=True)

        # 履歴タイトル
        history_title = tk.Label(history_frame,
                                text="実行履歴",
                                font=self.fonts['title2'],
                                bg=self.get_color('surface'),
                                fg=self.get_color('text'))
        history_title.pack(anchor=tk.W, padx=20, pady=(20, 12))

        # 履歴リストボックス
        self.history_listbox = tk.Listbox(history_frame,
                                         font=self.fonts['body'],
                                         bg=self.get_color('bg'),
                                         fg=self.get_color('text'),
                                         selectbackground=self.get_color('primary'),
                                         selectforeground='white',
                                         relief='flat',
                                         borderwidth=0)
        self.history_listbox.pack(fill=tk.BOTH, expand=True, padx=20, pady=(0, 20))

        # 履歴操作ボタン
        btn_frame = tk.Frame(history_frame, bg=self.get_color('surface'))
        btn_frame.pack(fill=tk.X, padx=20, pady=(0, 20))

        clear_btn = tk.Button(btn_frame,
                             text="履歴クリア",
                             font=self.fonts['callout'],
                             bg=self.get_color('secondary'),
                             fg='white',
                             relief='flat',
                             borderwidth=0,
                             padx=12,
                             pady=6,
                             command=self.clear_history)
        clear_btn.pack(side=tk.RIGHT)

    def create_status_bar(self):
        """ステータスバー作成"""
        status_frame = tk.Frame(self.root, bg=self.get_color('surface'),
                               relief='flat', borderwidth=1, height=30)
        status_frame.pack(fill=tk.X, side=tk.BOTTOM)

        # ステータスラベル
        self.status_var = tk.StringVar(value="準備完了")
        status_label = tk.Label(status_frame,
                               textvariable=self.status_var,
                               font=self.fonts['caption1'],
                               bg=self.get_color('surface'),
                               fg=self.get_color('text_secondary'))
        status_label.pack(side=tk.LEFT, padx=12, pady=6)

        # バージョン情報
        version_label = tk.Label(status_frame,
                                text="Cowork Assistant v1.0.0",
                                font=self.fonts['caption2'],
                                bg=self.get_color('surface'),
                                fg=self.get_color('text_secondary'))
        version_label.pack(side=tk.RIGHT, padx=12, pady=6)

    def create_tooltip(self, widget, text):
        """ツールチップ作成"""
        def show_tooltip(event):
            tooltip = tk.Toplevel()
            tooltip.wm_overrideredirect(True)
            tooltip.wm_geometry(f"+{event.x_root+10}+{event.y_root+10}")

            label = tk.Label(tooltip, text=text, font=self.fonts['footnote'],
                           bg=self.get_color('surface'), fg=self.get_color('text'),
                           relief='solid', borderwidth=1, padx=8, pady=4)
            label.pack()

            def hide_tooltip():
                tooltip.destroy()

            widget.tooltip = tooltip
            widget.bind('<Leave>', lambda e: hide_tooltip())

        widget.bind('<Enter>', show_tooltip)

    def get_color(self, key):
        """現在のテーマに基づく色取得"""
        theme = 'dark' if self.is_dark_mode else 'light'
        return self.colors[theme][key]

    def execute_task(self):
        """タスク実行"""
        task_description = self.search_var.get().strip()
        if not task_description:
            messagebox.showwarning("警告", "タスクを入力してください。")
            return

        # UI更新
        self.status_var.set("タスク実行中...")
        self.search_var.set("")

        # 非同期実行
        threading.Thread(target=self._execute_task_async, args=(task_description,), daemon=True).start()

    def _execute_task_async(self, task_description):
        """非同期タスク実行"""
        try:
            # タスク実行
            result = asyncio.run(self.assistant.execute_task(task_description))

            # UI更新
            self.root.after(0, lambda: self.update_task_result(result, task_description))

        except Exception as e:
            self.root.after(0, lambda: self.show_error(f"タスク実行エラー: {str(e)}"))

    def update_task_result(self, result, task_description):
        """タスク結果更新"""
        # ステータス更新
        if result.get('success'):
            self.status_var.set("タスク完了")
            status_color = self.get_color('success')
        else:
            self.status_var.set("タスク失敗")
            status_color = self.get_color('error')

        # 履歴追加
        timestamp = datetime.now().strftime("%H:%M:%S")
        history_item = f"[{timestamp}] {task_description[:50]}..."
        self.history_listbox.insert(0, history_item)
        self.task_history.append({
            'description': task_description,
            'result': result,
            'timestamp': datetime.now()
        })

    def select_feature(self, title, description):
        """機能選択"""
        # 機能に応じたデフォルトタスクを設定
        default_tasks = {
            "ファイル整理": "ダウンロードフォルダを整理して分類してください",
            "データ分析": "sales_data.csvを分析してレポートを作成してください",
            "Web操作": "https://example.comからデータをスクレイピングしてください",
            "文書処理": "documentsフォルダ内のPDFからテキストを抽出してください",
            "画像処理": "imagesフォルダ内の写真からテキストを読み取ってください",
            "研究支援": "AI技術の最新トレンドを調査してください",
            "レポート作成": "先月のデータをまとめてレポートを作成してください",
            "ワークフロー": "毎日のルーティンタスクを自動化してください"
        }

        if title in default_tasks:
            self.search_var.set(default_tasks[title])

    def clear_history(self):
        """履歴クリア"""
        self.history_listbox.delete(0, tk.END)
        self.task_history.clear()

    def show_error(self, message):
        """エラー表示"""
        self.status_var.set("エラー発生")
        messagebox.showerror("エラー", message)

    def create_desktop_shortcut(self):
        """デスクトップショートカット作成"""
        try:
            if platform.system() == "Windows":
                self._create_windows_shortcut()
            elif platform.system() == "Darwin":  # macOS
                self._create_macos_shortcut()
            else:  # Linux
                self._create_linux_shortcut()
        except Exception as e:
            print(f"ショートカット作成エラー: {e}")

    def _create_windows_shortcut(self):
        """Windowsショートカット作成"""
        try:
            import winshell
            from win32com.client import Dispatch

            desktop = winshell.desktop()
            shortcut_path = os.path.join(desktop, "Cowork Assistant.lnk")

            shell = Dispatch('WScript.Shell')
            shortcut = shell.CreateShortCut(shortcut_path)
            shortcut.Targetpath = sys.executable
            shortcut.Arguments = f'"{__file__}"'
            shortcut.WorkingDirectory = os.path.dirname(__file__)
            shortcut.IconLocation = sys.executable
            shortcut.save()

        except ImportError:
            # win32comが使えない場合の代替
            self._create_windows_shortcut_fallback()

    def _create_windows_shortcut_fallback(self):
        """Windowsショートカットの代替作成"""
        desktop = os.path.join(os.path.expanduser("~"), "Desktop")
        script_path = os.path.join(desktop, "Cowork Assistant.bat")

        with open(script_path, 'w') as f:
            f.write(f'@echo off\npython "{__file__}"\n')

    def _create_macos_shortcut(self):
        """macOSエイリアス作成"""
        # macOSでのエイリアス作成は複雑なので、バッチファイルを作成
        desktop = os.path.join(os.path.expanduser("~"), "Desktop")
        script_path = os.path.join(desktop, "Cowork Assistant.command")

        with open(script_path, 'w') as f:
            f.write(f'#!/bin/bash\npython3 "{__file__}"\n')

        # 実行権限付与
        os.chmod(script_path, 0o755)

    def _create_linux_shortcut(self):
        """Linuxデスクトップエントリ作成"""
        desktop = os.path.join(os.path.expanduser("~"), "Desktop")
        desktop_file = os.path.join(desktop, "cowork-assistant.desktop")

        desktop_content = f"""[Desktop Entry]
Version=1.0
Type=Application
Name=Cowork Assistant
Comment=AI Productivity Assistant
Exec=python3 "{__file__}"
Icon=applications-office
Terminal=false
StartupWMClass=CoworkAssistant
"""

        with open(desktop_file, 'w') as f:
            f.write(desktop_content)

        # 実行権限付与
        os.chmod(desktop_file, 0o755)

    def setup_system_tray(self):
        """システムトレイ設定"""
        try:
            # トレイアイコン作成
            icon_image = self._create_tray_icon()

            # メニューの作成
            menu = (
                item('Cowork Assistant', lambda: self.show_main_window()),
                item('機能検索', lambda: self.show_feature_search()),
                item('設定', lambda: self.show_settings()),
                item('終了', lambda: self.quit_application())
            )

            # システムトレイアイコン設定
            self.system_tray = pystray.Icon(
                "cowork_assistant",
                icon_image,
                "Cowork Assistant",
                menu
            )

            # 別スレッドで実行
            tray_thread = threading.Thread(target=self.system_tray.run, daemon=True)
            tray_thread.start()

        except Exception as e:
            print(f"システムトレイ設定エラー: {e}")

    def _create_tray_icon(self):
        """トレイアイコン作成"""
        # シンプルなアイコン生成
        icon = Image.new('RGBA', (64, 64), (0, 0, 0, 0))
        draw = ImageDraw.Draw(icon)

        # Apple風デザインのアイコン
        # 背景円
        draw.ellipse([8, 8, 56, 56], fill=(10, 132, 255, 255))

        # チェックマーク
        draw.line([20, 32, 28, 40], fill=(255, 255, 255, 255), width=3)
        draw.line([28, 40, 44, 24], fill=(255, 255, 255, 255), width=3)

        return icon

    def show_main_window(self):
        """メインウィンドウ表示"""
        if self.root:
            self.root.deiconify()
            self.root.lift()
            self.root.focus_force()

    def show_feature_search(self):
        """機能検索ウィンドウ表示"""
        search_window = tk.Toplevel(self.root)
        search_window.title("機能検索")
        search_window.geometry("600x400")
        search_window.configure(bg=self.get_color('bg'))

        # 検索機能の実装
        # （Coworkの全機能を検索・実行可能にする）
        features = [
            "ファイル整理", "データ分析", "Webスクレイピング", "文書処理",
            "画像OCR", "レポート生成", "ワークフロー自動化", "研究支援",
            "メール処理", "スケジュール管理", "タスク整理", "ノート整理"
        ]

        # 検索エントリー
        search_var = tk.StringVar()
        search_entry = tk.Entry(search_window,
                               textvariable=search_var,
                               font=self.fonts['body'])
        search_entry.pack(fill=tk.X, padx=20, pady=20)

        # 結果リスト
        result_listbox = tk.Listbox(search_window, font=self.fonts['body'])
        result_listbox.pack(fill=tk.BOTH, expand=True, padx=20, pady=(0, 20))

        # 検索結果表示
        for feature in features:
            result_listbox.insert(tk.END, feature)

        # 実行ボタン
        execute_btn = tk.Button(search_window,
                               text="実行",
                               command=lambda: self._execute_selected_feature(
                                   result_listbox.get(result_listbox.curselection())
                               ))
        execute_btn.pack(pady=(0, 20))

    def _execute_selected_feature(self, feature):
        """選択された機能を実行"""
        if feature:
            # 機能に応じたタスクを設定
            feature_tasks = {
                "ファイル整理": "ダウンロードフォルダを整理してください",
                "データ分析": "データを分析してレポートを作成してください",
                "Webスクレイピング": "Webサイトからデータを収集してください",
                "文書処理": "文書からテキストを抽出してください",
                "画像OCR": "画像から文字を読み取ってください",
                "レポート生成": "データをまとめてレポートを作成してください",
                "ワークフロー自動化": "日常業務を自動化してください",
                "研究支援": "情報を調査してまとめを作成してください"
            }

            if feature in feature_tasks:
                self.search_var.set(feature_tasks[feature])
                self.execute_task()

    def show_settings(self):
        """設定ウィンドウ表示"""
        settings_window = tk.Toplevel(self.root)
        settings_window.title("設定")
        settings_window.geometry("400x300")
        settings_window.configure(bg=self.get_color('bg'))

        # 設定項目（簡易版）
        tk.Label(settings_window, text="設定ウィンドウ", font=self.fonts['title2']).pack(pady=20)

    def quit_application(self):
        """アプリケーション終了"""
        if self.system_tray:
            self.system_tray.stop()
        self.root.quit()

    def run(self):
        """GUI実行"""
        try:
            self.create_main_window()
            self.root.mainloop()
        except Exception as e:
            messagebox.showerror("エラー", f"GUI起動エラー: {str(e)}")
        finally:
            if self.system_tray:
                self.system_tray.stop()


def main():
    """メイン関数"""
    # 高DPI対応
    try:
        from ctypes import windll
        windll.shcore.SetProcessDpiAwareness(1)
    except:
        pass

    # GUI起動
    gui = AppleStyleGUI()
    gui.run()


if __name__ == "__main__":
    main()