#!/usr/bin/env python3
"""
ClaudeCowork統合機能のパフォーマンス最適化
キャッシュ、並列処理、リソース管理の最適化
"""

import asyncio
import functools
import hashlib
import json
import logging
import time
from pathlib import Path
from typing import Dict, List, Optional, Any, Callable
from datetime import datetime, timedelta
from collections import OrderedDict
import threading

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


class PerformanceCache:
    """インテリジェントキャッシュシステム"""

    def __init__(self, max_size: int = 100, ttl_seconds: int = 3600):
        self.max_size = max_size
        self.ttl_seconds = ttl_seconds
        self.cache: OrderedDict[str, Dict[str, Any]] = OrderedDict()
        self.lock = threading.Lock()

    def _generate_key(self, func_name: str, *args, **kwargs) -> str:
        """キャッシュキー生成"""
        key_data = {
            "func": func_name,
            "args": str(args),
            "kwargs": str(sorted(kwargs.items())),
        }
        key_str = json.dumps(key_data, sort_keys=True)
        return hashlib.sha256(key_str.encode()).hexdigest()

    def get(self, key: str) -> Optional[Any]:
        """キャッシュから取得"""
        with self.lock:
            if key not in self.cache:
                return None

            entry = self.cache[key]

            # TTLチェック
            if time.time() - entry["timestamp"] > self.ttl_seconds:
                del self.cache[key]
                return None

            # LRU: 最後に使用されたものとしてマーク
            self.cache.move_to_end(key)
            return entry["value"]

    def set(self, key: str, value: Any):
        """キャッシュに保存"""
        with self.lock:
            # サイズ制限
            if len(self.cache) >= self.max_size:
                # LRU: 最も古いものを削除
                self.cache.popitem(last=False)

            self.cache[key] = {"value": value, "timestamp": time.time()}

    def clear(self):
        """キャッシュクリア"""
        with self.lock:
            self.cache.clear()

    def get_stats(self) -> Dict[str, Any]:
        """キャッシュ統計"""
        with self.lock:
            return {
                "size": len(self.cache),
                "max_size": self.max_size,
                "ttl_seconds": self.ttl_seconds,
            }


def cached_result(cache: PerformanceCache, ttl_seconds: Optional[int] = None):
    """結果キャッシュデコレータ"""

    def decorator(func: Callable):
        @functools.wraps(func)
        async def wrapper(*args, **kwargs):
            key = cache._generate_key(func.__name__, *args, **kwargs)

            # キャッシュから取得
            cached = cache.get(key)
            if cached is not None:
                logger.debug(f"キャッシュヒット: {func.__name__}")
                return cached

            # 実行
            result = await func(*args, **kwargs)

            # キャッシュに保存
            cache.set(key, result)

            return result

        return wrapper

    return decorator


class ResourceManager:
    """リソース管理システム"""

    def __init__(self, max_concurrent_tasks: int = 5):
        self.max_concurrent_tasks = max_concurrent_tasks
        self.semaphore = asyncio.Semaphore(max_concurrent_tasks)
        self.active_tasks: Dict[str, asyncio.Task] = {}
        self.task_stats: Dict[str, Dict[str, Any]] = {}

    async def execute_with_limit(self, task_id: str, coro: Callable):
        """同時実行数制限付きタスク実行"""
        async with self.semaphore:
            start_time = time.time()
            try:
                result = await coro
                execution_time = time.time() - start_time

                self.task_stats[task_id] = {
                    "status": "completed",
                    "execution_time": execution_time,
                    "timestamp": datetime.now().isoformat(),
                }

                return result
            except Exception as e:
                execution_time = time.time() - start_time
                self.task_stats[task_id] = {
                    "status": "failed",
                    "execution_time": execution_time,
                    "error": str(e),
                    "timestamp": datetime.now().isoformat(),
                }
                raise

    def get_stats(self) -> Dict[str, Any]:
        """リソース統計"""
        return {
            "max_concurrent": self.max_concurrent_tasks,
            "active_tasks": len(self.active_tasks),
            "task_stats": self.task_stats,
        }


class PerformanceMonitor:
    """パフォーマンス監視システム"""

    def __init__(self):
        self.metrics: Dict[str, List[float]] = {}
        self.lock = threading.Lock()

    def record_metric(self, name: str, value: float):
        """メトリクス記録"""
        with self.lock:
            if name not in self.metrics:
                self.metrics[name] = []
            self.metrics[name].append(value)

            # 最新100件のみ保持
            if len(self.metrics[name]) > 100:
                self.metrics[name] = self.metrics[name][-100:]

    def get_average(self, name: str) -> Optional[float]:
        """平均値取得"""
        with self.lock:
            if name not in self.metrics or not self.metrics[name]:
                return None
            return sum(self.metrics[name]) / len(self.metrics[name])

    def get_stats(self) -> Dict[str, Any]:
        """統計情報取得"""
        with self.lock:
            stats = {}
            for name, values in self.metrics.items():
                if values:
                    stats[name] = {
                        "count": len(values),
                        "average": sum(values) / len(values),
                        "min": min(values),
                        "max": max(values),
                        "latest": values[-1] if values else None,
                    }
            return stats


# グローバルインスタンス
_performance_cache = PerformanceCache(max_size=200, ttl_seconds=3600)
_resource_manager = ResourceManager(max_concurrent_tasks=5)
_performance_monitor = PerformanceMonitor()


def get_performance_cache() -> PerformanceCache:
    """パフォーマンスキャッシュ取得"""
    return _performance_cache


def get_resource_manager() -> ResourceManager:
    """リソースマネージャー取得"""
    return _resource_manager


def get_performance_monitor() -> PerformanceMonitor:
    """パフォーマンスモニター取得"""
    return _performance_monitor
