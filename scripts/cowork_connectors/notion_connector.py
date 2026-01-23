"""
Notion Connector
ナレッジベースNotionとの統合
"""

import asyncio
import logging
from typing import Dict, List, Optional, Any
import aiohttp

from .connector_base import ConnectorBase, ConnectorResult, ConnectorStatus


class NotionConnector(ConnectorBase):
    """Notion API統合"""
    
    BASE_URL = "https://api.notion.com/v1"
    
    def __init__(self, api_key: str):
        super().__init__("Notion", api_key=api_key)
        self.session: Optional[aiohttp.ClientSession] = None
    
    async def connect(self) -> ConnectorResult:
        """Notionに接続"""
        try:
            self.status = ConnectorStatus.CONNECTING
            
            headers = {
                "Authorization": f"Bearer {self.config['api_key']}",
                "Notion-Version": "2022-06-28",
                "Content-Type": "application/json"
            }
            
            self.session = aiohttp.ClientSession(headers=headers)
            
            # 接続テスト（ユーザー情報取得）
            async with self.session.get(f"{self.BASE_URL}/users/me") as response:
                if response.status == 200:
                    self.status = ConnectorStatus.CONNECTED
                    user_data = await response.json()
                    self.logger.info("Notion接続成功")
                    return ConnectorResult(success=True, data=user_data)
                else:
                    self.status = ConnectorStatus.ERROR
                    error_text = await response.text()
                    return ConnectorResult(success=False, error=f"接続失敗: {error_text}")
        
        except Exception as e:
            self.status = ConnectorStatus.ERROR
            self.logger.error(f"Notion接続エラー: {e}")
            return ConnectorResult(success=False, error=str(e))
    
    async def disconnect(self) -> ConnectorResult:
        """接続切断"""
        try:
            if self.session:
                await self.session.close()
                self.session = None
            
            self.status = ConnectorStatus.DISCONNECTED
            self.logger.info("Notion切断完了")
            return ConnectorResult(success=True)
        
        except Exception as e:
            self.logger.error(f"Notion切断エラー: {e}")
            return ConnectorResult(success=False, error=str(e))
    
    async def create_page(
        self,
        parent_database_id: str,
        properties: Dict[str, Any]
    ) -> ConnectorResult:
        """ページ作成"""
        if not self.is_connected():
            return ConnectorResult(success=False, error="未接続")
        
        try:
            data = {
                "parent": {"database_id": parent_database_id},
                "properties": properties
            }
            
            async with self.session.post(f"{self.BASE_URL}/pages", json=data) as response:
                if response.status == 200:
                    page_data = await response.json()
                    return ConnectorResult(success=True, data=page_data)
                else:
                    error_text = await response.text()
                    return ConnectorResult(success=False, error=f"ページ作成失敗: {error_text}")
        except Exception as e:
            return ConnectorResult(success=False, error=str(e))
