"""
Asana Connector
タスク管理サービスAsanaとの統合
"""

import asyncio
import logging
from typing import Dict, List, Optional, Any
import aiohttp

from .connector_base import ConnectorBase, ConnectorResult, ConnectorStatus


class AsanaConnector(ConnectorBase):
    """Asana API統合"""

    BASE_URL = "https://app.asana.com/api/1.0"

    def __init__(self, api_key: str):
        super().__init__("Asana", api_key=api_key)
        self.session: Optional[aiohttp.ClientSession] = None

    async def connect(self) -> ConnectorResult:
        """Asanaに接続"""
        try:
            self.status = ConnectorStatus.CONNECTING

            headers = {
                "Authorization": f"Bearer {self.config['api_key']}",
                "Content-Type": "application/json",
            }

            self.session = aiohttp.ClientSession(headers=headers)

            # 接続テスト（ユーザー情報取得）
            async with self.session.get(f"{self.BASE_URL}/users/me") as response:
                if response.status == 200:
                    self.status = ConnectorStatus.CONNECTED
                    user_data = await response.json()
                    self.logger.info("Asana接続成功")
                    return ConnectorResult(success=True, data=user_data)
                else:
                    self.status = ConnectorStatus.ERROR
                    error_text = await response.text()
                    return ConnectorResult(
                        success=False, error=f"接続失敗: {error_text}"
                    )

        except Exception as e:
            self.status = ConnectorStatus.ERROR
            self.logger.error(f"Asana接続エラー: {e}")
            return ConnectorResult(success=False, error=str(e))

    async def disconnect(self) -> ConnectorResult:
        """接続切断"""
        try:
            if self.session:
                await self.session.close()
                self.session = None

            self.status = ConnectorStatus.DISCONNECTED
            self.logger.info("Asana切断完了")
            return ConnectorResult(success=True)

        except Exception as e:
            self.logger.error(f"Asana切断エラー: {e}")
            return ConnectorResult(success=False, error=str(e))

    async def create_task(
        self,
        workspace_id: str,
        name: str,
        notes: Optional[str] = None,
        assignee: Optional[str] = None,
    ) -> ConnectorResult:
        """タスク作成"""
        if not self.is_connected():
            return ConnectorResult(success=False, error="未接続")

        try:
            data = {"data": {"workspace": workspace_id, "name": name}}
            if notes:
                data["data"]["notes"] = notes
            if assignee:
                data["data"]["assignee"] = assignee

            async with self.session.post(
                f"{self.BASE_URL}/tasks", json=data
            ) as response:
                if response.status == 201:
                    task_data = await response.json()
                    return ConnectorResult(success=True, data=task_data)
                else:
                    error_text = await response.text()
                    return ConnectorResult(
                        success=False, error=f"タスク作成失敗: {error_text}"
                    )
        except Exception as e:
            return ConnectorResult(success=False, error=str(e))
