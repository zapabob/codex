#!/usr/bin/env python3
"""
ClaudeCowork-style Session Manager
セッション管理、ファイルプレビュー、タスク履歴管理
"""

import json
import logging
import os
from pathlib import Path
from typing import Dict, List, Optional, Any
from dataclasses import dataclass, field, asdict
from datetime import datetime
from enum import Enum
import uuid
import mimetypes


class SessionStatus(Enum):
    """セッション状態"""

    ACTIVE = "active"
    PAUSED = "paused"
    COMPLETED = "completed"
    ARCHIVED = "archived"


@dataclass
class Session:
    """セッション情報"""

    id: str
    name: str
    created_at: datetime
    updated_at: datetime
    status: SessionStatus
    tasks: List[Dict[str, Any]] = field(default_factory=list)
    files: List[str] = field(default_factory=list)
    metadata: Dict[str, Any] = field(default_factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        """辞書に変換"""
        data = asdict(self)
        data["created_at"] = self.created_at.isoformat()
        data["updated_at"] = self.updated_at.isoformat()
        data["status"] = self.status.value
        return data

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "Session":
        """辞書から復元"""
        session = cls(
            id=data["id"],
            name=data["name"],
            created_at=datetime.fromisoformat(data["created_at"]),
            updated_at=datetime.fromisoformat(data["updated_at"]),
            status=SessionStatus(data["status"]),
            tasks=data.get("tasks", []),
            files=data.get("files", []),
            metadata=data.get("metadata", {}),
        )
        return session


class SessionManager:
    """
    ClaudeCoworkスタイルのセッション管理

    機能:
    - セッションの作成・削除・リネーム
    - ファイルプレビュー機能
    - タスク履歴管理
    - 状態の永続化
    """

    def __init__(self, sessions_dir: Optional[Path] = None):
        self.logger = logging.getLogger("SessionManager")

        if sessions_dir is None:
            sessions_dir = Path.home() / ".codex" / "sessions"
        self.sessions_dir = Path(sessions_dir)
        self.sessions_dir.mkdir(parents=True, exist_ok=True)

        self.sessions: Dict[str, Session] = {}
        self.active_session_id: Optional[str] = None

        # セッション読み込み
        self._load_sessions()

    def _load_sessions(self):
        """保存されたセッションを読み込み"""
        for session_file in self.sessions_dir.glob("*.json"):
            try:
                with open(session_file, "r", encoding="utf-8") as f:
                    data = json.load(f)
                    session = Session.from_dict(data)
                    self.sessions[session.id] = session
            except Exception as e:
                self.logger.warning(f"セッション読み込みエラー ({session_file}): {e}")

    def _save_session(self, session: Session):
        """セッションを保存"""
        session_file = self.sessions_dir / f"{session.id}.json"
        try:
            with open(session_file, "w", encoding="utf-8") as f:
                json.dump(session.to_dict(), f, indent=2, ensure_ascii=False)
        except Exception as e:
            self.logger.error(f"セッション保存エラー: {e}")

    def create_session(
        self, name: str, metadata: Optional[Dict[str, Any]] = None
    ) -> Session:
        """セッション作成"""
        session_id = str(uuid.uuid4())
        now = datetime.now()

        session = Session(
            id=session_id,
            name=name,
            created_at=now,
            updated_at=now,
            status=SessionStatus.ACTIVE,
            metadata=metadata or {},
        )

        self.sessions[session_id] = session
        self.active_session_id = session_id
        self._save_session(session)

        self.logger.info(f"セッション作成: {name} ({session_id})")
        return session

    def get_session(self, session_id: str) -> Optional[Session]:
        """セッション取得"""
        return self.sessions.get(session_id)

    def list_sessions(self, status: Optional[SessionStatus] = None) -> List[Session]:
        """セッション一覧取得"""
        sessions = list(self.sessions.values())
        if status:
            sessions = [s for s in sessions if s.status == status]
        return sorted(sessions, key=lambda s: s.updated_at, reverse=True)

    def rename_session(self, session_id: str, new_name: str) -> bool:
        """セッションリネーム"""
        session = self.sessions.get(session_id)
        if not session:
            return False

        session.name = new_name
        session.updated_at = datetime.now()
        self._save_session(session)

        self.logger.info(f"セッションリネーム: {session_id} -> {new_name}")
        return True

    def delete_session(self, session_id: str) -> bool:
        """セッション削除"""
        if session_id not in self.sessions:
            return False

        # ファイル削除
        session_file = self.sessions_dir / f"{session_id}.json"
        if session_file.exists():
            session_file.unlink()

        # メモリから削除
        del self.sessions[session_id]

        if self.active_session_id == session_id:
            self.active_session_id = None

        self.logger.info(f"セッション削除: {session_id}")
        return True

    def add_task(self, session_id: str, task: Dict[str, Any]) -> bool:
        """タスク追加"""
        session = self.sessions.get(session_id)
        if not session:
            return False

        task["id"] = str(uuid.uuid4())
        task["created_at"] = datetime.now().isoformat()
        session.tasks.append(task)
        session.updated_at = datetime.now()
        self._save_session(session)

        self.logger.info(f"タスク追加: {session_id} - {task.get('name', 'Unknown')}")
        return True

    def get_tasks(self, session_id: str) -> List[Dict[str, Any]]:
        """タスク一覧取得"""
        session = self.sessions.get(session_id)
        if not session:
            return []
        return session.tasks

    def add_file(self, session_id: str, file_path: str) -> bool:
        """ファイル追加"""
        session = self.sessions.get(session_id)
        if not session:
            return False

        if file_path not in session.files:
            session.files.append(file_path)
            session.updated_at = datetime.now()
            self._save_session(session)

        self.logger.info(f"ファイル追加: {session_id} - {file_path}")
        return True

    def get_files(self, session_id: str) -> List[str]:
        """ファイル一覧取得"""
        session = self.sessions.get(session_id)
        if not session:
            return []
        return session.files

    def preview_file(
        self, file_path: str, max_size: int = 1024 * 1024
    ) -> Dict[str, Any]:
        """ファイルプレビュー"""
        path = Path(file_path)

        if not path.exists():
            return {"success": False, "error": "ファイルが存在しません"}

        if path.is_dir():
            return {"success": False, "error": "ディレクトリはプレビューできません"}

        file_size = path.stat().st_size
        if file_size > max_size:
            return {
                "success": False,
                "error": f"ファイルサイズが大きすぎます ({file_size} bytes)",
            }

        mime_type, _ = mimetypes.guess_type(str(path))

        preview_data = {
            "success": True,
            "path": str(path),
            "name": path.name,
            "size": file_size,
            "mime_type": mime_type or "application/octet-stream",
            "modified": datetime.fromtimestamp(path.stat().st_mtime).isoformat(),
        }

        # テキストファイルの場合は内容も含める
        if mime_type and mime_type.startswith("text/"):
            try:
                with open(path, "r", encoding="utf-8") as f:
                    content = f.read()
                    preview_data["content"] = content[:5000]  # 最初の5000文字
                    preview_data["truncated"] = len(content) > 5000
            except Exception as e:
                preview_data["content_error"] = str(e)

        return preview_data

    def set_active_session(self, session_id: str) -> bool:
        """アクティブセッション設定"""
        if session_id not in self.sessions:
            return False

        self.active_session_id = session_id
        self.logger.info(f"アクティブセッション設定: {session_id}")
        return True

    def get_active_session(self) -> Optional[Session]:
        """アクティブセッション取得"""
        if self.active_session_id:
            return self.sessions.get(self.active_session_id)
        return None

    def update_session_status(self, session_id: str, status: SessionStatus) -> bool:
        """セッション状態更新"""
        session = self.sessions.get(session_id)
        if not session:
            return False

        session.status = status
        session.updated_at = datetime.now()
        self._save_session(session)

        self.logger.info(f"セッション状態更新: {session_id} -> {status.value}")
        return True


def main():
    """テスト実行"""
    manager = SessionManager()

    # セッション作成
    session = manager.create_session(
        "テストセッション", {"description": "ClaudeCowork統合テスト"}
    )
    print(f"セッション作成: {session.name} ({session.id})")

    # タスク追加
    task = {
        "name": "ファイル整理",
        "description": "ダウンロードフォルダを整理",
        "status": "pending",
    }
    manager.add_task(session.id, task)

    # ファイル追加
    manager.add_file(session.id, "test_file.txt")

    # セッション一覧
    sessions = manager.list_sessions()
    print(f"セッション数: {len(sessions)}")

    # ファイルプレビュー
    preview = manager.preview_file("test_file.txt")
    print(f"プレビュー結果: {preview}")


if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    main()
