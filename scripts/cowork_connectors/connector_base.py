"""
Connector Base Class
外部サービスコネクターの基底クラス
"""

import logging
from abc import ABC, abstractmethod
from dataclasses import dataclass
from enum import Enum
from typing import Dict, List, Optional, Any


class ConnectorStatus(Enum):
    """コネクターの状態"""

    DISCONNECTED = "disconnected"
    CONNECTING = "connecting"
    CONNECTED = "connected"
    ERROR = "error"


@dataclass
class ConnectorResult:
    """コネクター操作の結果"""

    success: bool
    data: Optional[Dict[str, Any]] = None
    error: Optional[str] = None
    metadata: Optional[Dict[str, Any]] = None


class ConnectorBase(ABC):
    """外部サービスコネクターの基底クラス"""

    def __init__(self, name: str, **kwargs):
        self.name = name
        self.status = ConnectorStatus.DISCONNECTED
        self.logger = logging.getLogger(f"Connector.{name}")
        self.config = kwargs

    @abstractmethod
    async def connect(self) -> ConnectorResult:
        """サービスに接続"""
        pass

    @abstractmethod
    async def disconnect(self) -> ConnectorResult:
        """接続を切断"""
        pass

    async def test_connection(self) -> ConnectorResult:
        """接続テスト"""
        if not self.is_connected():
            return ConnectorResult(success=False, error="未接続")
        return ConnectorResult(success=True)

    def is_connected(self) -> bool:
        """接続状態を確認"""
        return self.status == ConnectorStatus.CONNECTED

    def get_status(self) -> ConnectorStatus:
        """現在の状態を取得"""
        return self.status
