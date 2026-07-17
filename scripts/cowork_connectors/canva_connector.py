"""
Canva Connector
デザインサービスCanvaとの統合（基本実装）
"""

import asyncio
import logging
from typing import Dict, List, Optional, Any
import aiohttp

from .connector_base import ConnectorBase, ConnectorResult, ConnectorStatus


class CanvaConnector(ConnectorBase):
    """Canva API統合（基本実装）"""

    BASE_URL = "https://api.canva.com/rest/v1"

    def __init__(self, api_key: str):
        super().__init__("Canva", api_key=api_key)
        self.session: Optional[aiohttp.ClientSession] = None

    async def connect(self) -> ConnectorResult:
        """Canvaに接続"""
        try:
            self.status = ConnectorStatus.CONNECTING

            headers = {
                "Authorization": f"Bearer {self.api_key}",
                "Content-Type": "application/json",
            }

            self.session = aiohttp.ClientSession(headers=headers)

            # 接続テスト（ユーザー情報取得）
            async with self.session.get(f"{self.BASE_URL}/users/me") as response:
                if response.status == 200:
                    self.status = ConnectorStatus.CONNECTED
                    user_data = await response.json()
                    self.logger.info("Canva接続成功")
                    return ConnectorResult(success=True, data=user_data)
                else:
                    self.status = ConnectorStatus.ERROR
                    error_text = await response.text()
                    return ConnectorResult(
                        success=False, error=f"接続失敗: {error_text}"
                    )

        except Exception as e:
            self.status = ConnectorStatus.ERROR
            self.logger.error(f"Canva接続エラー: {e}")
            return ConnectorResult(success=False, error=str(e))

    async def disconnect(self) -> ConnectorResult:
        """接続切断"""
        try:
            if self.session:
                await self.session.close()
                self.session = None

            self.status = ConnectorStatus.DISCONNECTED
            self.logger.info("Canva切断完了")
            return ConnectorResult(success=True)

        except Exception as e:
            self.logger.error(f"Canva切断エラー: {e}")
            return ConnectorResult(success=False, error=str(e))

    async def test_connection(self) -> ConnectorResult:
        """接続テスト"""
        if not self.is_connected():
            return ConnectorResult(success=False, error="未接続")

        try:
            async with self.session.get(f"{self.BASE_URL}/users/me") as response:
                if response.status == 200:
                    return ConnectorResult(success=True)
                else:
                    return ConnectorResult(
                        success=False, error=f"HTTP {response.status}"
                    )
        except Exception as e:
            return ConnectorResult(success=False, error=str(e))

    async def get_templates(self, category: Optional[str] = None) -> ConnectorResult:
        """テンプレート一覧取得"""
        if not self.is_connected():
            return ConnectorResult(success=False, error="未接続")

        try:
            params = {}
            if category:
                params["category"] = category

            async with self.session.get(
                f"{self.BASE_URL}/templates", params=params
            ) as response:
                if response.status == 200:
                    templates_data = await response.json()
                    self.logger.info(
                        f"テンプレート取得成功: {len(templates_data.get('data', []))}件"
                    )
                    return ConnectorResult(success=True, data=templates_data)
                else:
                    error_text = await response.text()
                    return ConnectorResult(
                        success=False, error=f"テンプレート取得失敗: {error_text}"
                    )

        except Exception as e:
            self.logger.error(f"テンプレート取得エラー: {e}")
            return ConnectorResult(success=False, error=str(e))

    async def create_design(
        self, template_id: str, customizations: Dict[str, Any]
    ) -> ConnectorResult:
        """デザイン作成"""
        if not self.is_connected():
            return ConnectorResult(success=False, error="未接続")

        try:
            data = {"template_id": template_id, "customizations": customizations}

            async with self.session.post(
                f"{self.BASE_URL}/designs", json=data
            ) as response:
                if response.status == 201:
                    design_data = await response.json()
                    self.logger.info(f"デザイン作成成功: {design_data.get('id')}")
                    return ConnectorResult(success=True, data=design_data)
                else:
                    error_text = await response.text()
                    return ConnectorResult(
                        success=False, error=f"デザイン作成失敗: {error_text}"
                    )

        except Exception as e:
            self.logger.error(f"デザイン作成エラー: {e}")
            return ConnectorResult(success=False, error=str(e))
