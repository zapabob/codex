#!/usr/bin/env python3
"""
ClaudeCowork統合機能のパフォーマンスベンチマーク
キャッシュ効果、並列処理、リソース管理の性能測定
"""

import asyncio
import time
import json
import statistics
from pathlib import Path
from typing import Dict, List, Any
from datetime import datetime
import sys

# テスト対象モジュール
sys.path.append(str(Path(__file__).parent))
from cowork_performance_optimizer import (
    get_performance_cache,
    get_resource_manager,
    get_performance_monitor,
)
from cowork_session_manager import SessionManager
from cowork_document_generator import DocumentGenerationEngine

# Configure logging
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


class PerformanceBenchmark:
    """パフォーマンスベンチマーク実行クラス"""

    def __init__(self):
        self.results: Dict[str, List[float]] = {}
        self.cache = get_performance_cache()
        self.resource_manager = get_resource_manager()
        self.performance_monitor = get_performance_monitor()
        self.temp_dir = Path(__file__).parent.parent / "temp_benchmark"
        self.temp_dir.mkdir(parents=True, exist_ok=True)

    async def benchmark_cache_performance(self) -> Dict[str, Any]:
        """キャッシュパフォーマンスベンチマーク"""
        logger.info("[BENCHMARK] キャッシュパフォーマンスベンチマーク開始...")

        # テストデータ生成
        test_data = {"key": "value", "number": 12345, "list": [1, 2, 3, 4, 5]}

        # キャッシュなしでの実行時間
        no_cache_times = []
        for i in range(10):
            start = time.time()
            # シミュレート: 重い処理
            await asyncio.sleep(0.01)
            _ = json.dumps(test_data)
            no_cache_times.append(time.time() - start)

        # キャッシュありでの実行時間
        cache_times = []
        cache_key = "benchmark_test"
        for i in range(10):
            start = time.time()
            cached = self.cache.get(cache_key)
            if cached is None:
                # シミュレート: 重い処理
                await asyncio.sleep(0.01)
                result = json.dumps(test_data)
                self.cache.set(cache_key, result)
            cache_times.append(time.time() - start)

        avg_no_cache = statistics.mean(no_cache_times)
        avg_cache = statistics.mean(cache_times)
        speedup = avg_no_cache / avg_cache if avg_cache > 0 else 0

        result = {
            "test": "cache_performance",
            "no_cache_avg_ms": avg_no_cache * 1000,
            "cache_avg_ms": avg_cache * 1000,
            "speedup": speedup,
            "cache_hit_rate": len([t for t in cache_times if t < 0.001])
            / len(cache_times)
            * 100,
        }

        logger.info(f"[OK] キャッシュパフォーマンス: {speedup:.2f}x高速化")
        return result

    async def benchmark_parallel_processing(self) -> Dict[str, Any]:
        """並列処理パフォーマンスベンチマーク"""
        logger.info("[BENCHMARK] 並列処理パフォーマンスベンチマーク開始...")

        async def dummy_task(task_id: int):
            """ダミータスク"""
            await asyncio.sleep(0.1)
            return f"task_{task_id}_completed"

        # シーケンシャル実行
        sequential_times = []
        for i in range(3):
            start = time.time()
            tasks = [dummy_task(j) for j in range(5)]
            await asyncio.gather(*tasks)
            sequential_times.append(time.time() - start)

        # 並列実行（リソースマネージャー使用）
        parallel_times = []
        for i in range(3):
            start = time.time()
            tasks = [
                self.resource_manager.execute_with_limit(f"task_{j}", dummy_task(j))
                for j in range(5)
            ]
            await asyncio.gather(*tasks)
            parallel_times.append(time.time() - start)

        avg_sequential = statistics.mean(sequential_times)
        avg_parallel = statistics.mean(parallel_times)
        speedup = avg_sequential / avg_parallel if avg_parallel > 0 else 0

        result = {
            "test": "parallel_processing",
            "sequential_avg_ms": avg_sequential * 1000,
            "parallel_avg_ms": avg_parallel * 1000,
            "speedup": speedup,
            "concurrent_tasks": 5,
        }

        logger.info(f"[OK] 並列処理: {speedup:.2f}x高速化")
        return result

    async def benchmark_document_generation(self) -> Dict[str, Any]:
        """ドキュメント生成パフォーマンスベンチマーク"""
        logger.info("[BENCHMARK] ドキュメント生成パフォーマンスベンチマーク開始...")

        engine = DocumentGenerationEngine()

        # Excel生成
        excel_times = []
        for i in range(5):
            start = time.time()
            excel_data = {
                "sheets": [
                    {
                        "name": f"Sheet{i}",
                        "rows": [["A", "B", "C"], [1, 2, 3], [4, 5, 6]],
                    }
                ]
            }
            excel_path = self.temp_dir / f"benchmark_excel_{i}.xlsx"
            result = engine.generate_excel(str(excel_path), excel_data)
            excel_times.append(time.time() - start)
            if excel_path.exists():
                excel_path.unlink()

        # Word生成
        word_times = []
        for i in range(5):
            start = time.time()
            word_content = {
                "title": f"Document {i}",
                "sections": [{"heading": "Section", "paragraphs": ["Test paragraph"]}],
            }
            word_path = self.temp_dir / f"benchmark_word_{i}.docx"
            result = engine.generate_word(str(word_path), word_content)
            word_times.append(time.time() - start)
            if word_path.exists():
                word_path.unlink()

        result = {
            "test": "document_generation",
            "excel_avg_ms": statistics.mean(excel_times) * 1000,
            "word_avg_ms": statistics.mean(word_times) * 1000,
            "excel_min_ms": min(excel_times) * 1000,
            "excel_max_ms": max(excel_times) * 1000,
            "word_min_ms": min(word_times) * 1000,
            "word_max_ms": max(word_times) * 1000,
        }

        logger.info(
            f"[OK] ドキュメント生成: Excel {result['excel_avg_ms']:.2f}ms, Word {result['word_avg_ms']:.2f}ms"
        )
        return result

    async def benchmark_session_management(self) -> Dict[str, Any]:
        """セッション管理パフォーマンスベンチマーク"""
        logger.info("[BENCHMARK] セッション管理パフォーマンスベンチマーク開始...")

        manager = SessionManager(self.temp_dir / "benchmark_sessions")

        # セッション作成
        create_times = []
        for i in range(10):
            start = time.time()
            session = manager.create_session(f"Benchmark Session {i}", {})
            create_times.append(time.time() - start)

        # タスク追加
        add_task_times = []
        sessions = manager.list_sessions()
        if sessions:
            session_id = sessions[0].id
            for i in range(10):
                start = time.time()
                task = {"name": f"Task {i}", "status": "pending"}
                manager.add_task(session_id, task)
                add_task_times.append(time.time() - start)

        result = {
            "test": "session_management",
            "create_session_avg_ms": statistics.mean(create_times) * 1000,
            "add_task_avg_ms": statistics.mean(add_task_times) * 1000
            if add_task_times
            else 0,
            "create_min_ms": min(create_times) * 1000,
            "create_max_ms": max(create_times) * 1000,
        }

        logger.info(
            f"[OK] セッション管理: 作成 {result['create_session_avg_ms']:.2f}ms"
        )
        return result

    async def run_all_benchmarks(self) -> Dict[str, Any]:
        """全ベンチマーク実行"""
        logger.info("=" * 60)
        logger.info("ClaudeCowork統合機能 パフォーマンスベンチマーク開始")
        logger.info("=" * 60)

        benchmarks = [
            ("キャッシュパフォーマンス", self.benchmark_cache_performance),
            ("並列処理", self.benchmark_parallel_processing),
            ("ドキュメント生成", self.benchmark_document_generation),
            ("セッション管理", self.benchmark_session_management),
        ]

        results = {}

        for name, benchmark_func in benchmarks:
            try:
                result = await benchmark_func()
                results[name] = result
            except Exception as e:
                logger.error(f"[ERROR] {name}ベンチマーク失敗: {e}")
                results[name] = {"error": str(e)}

        # サマリー生成
        summary = {
            "timestamp": datetime.now().isoformat(),
            "results": results,
            "cache_stats": self.cache.get_stats(),
            "resource_stats": self.resource_manager.get_stats(),
            "performance_stats": self.performance_monitor.get_stats(),
        }

        # レポート保存
        report_path = self.temp_dir / "benchmark_report.json"
        with open(report_path, "w", encoding="utf-8") as f:
            json.dump(summary, f, indent=2, ensure_ascii=False)

        logger.info("\n" + "=" * 60)
        logger.info("パフォーマンスベンチマーク完了")
        logger.info("=" * 60)
        logger.info(f"レポート保存先: {report_path}")

        # サマリー表示
        self.print_summary(summary)

        return summary

    def print_summary(self, summary: Dict[str, Any]):
        """サマリー表示"""
        print("\n[BENCHMARK] ベンチマーク結果サマリー")
        print("-" * 60)

        for name, result in summary["results"].items():
            if "error" in result:
                print(f"[ERROR] {name}: エラー - {result['error']}")
            elif "speedup" in result:
                print(f"[OK] {name}: {result['speedup']:.2f}x高速化")
            elif "avg_ms" in result:
                print(
                    f"[OK] {name}: 平均 {result.get('avg_ms', result.get('excel_avg_ms', 0)):.2f}ms"
                )

        print("-" * 60)
        print(
            f"キャッシュサイズ: {summary['cache_stats']['size']}/{summary['cache_stats']['max_size']}"
        )
        print(f"同時実行数制限: {summary['resource_stats']['max_concurrent']}")


async def main():
    """メイン実行"""
    benchmark = PerformanceBenchmark()
    await benchmark.run_all_benchmarks()


if __name__ == "__main__":
    asyncio.run(main())
