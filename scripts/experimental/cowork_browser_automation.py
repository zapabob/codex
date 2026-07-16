#!/usr/bin/env python3
"""
ClaudeCowork-style Enhanced Browser Automation Engine
視覚的理解、マルチタブ操作、UI要素自動認識を備えた高度なブラウザ自動化
"""

import asyncio
import base64
import json
import logging
import os
from pathlib import Path
from typing import Dict, List, Optional, Any, Tuple
from dataclasses import dataclass, field
from datetime import datetime
import tempfile

try:
    from playwright.async_api import async_playwright, Browser, BrowserContext, Page, TimeoutError as PlaywrightTimeout
    import pytesseract
    from PIL import Image
    import io
except ImportError as e:
    print(f"必要なライブラリがインストールされていません: {e}")
    print("以下のコマンドでインストールしてください:")
    print("pip install playwright pillow pytesseract")
    print("python -m playwright install chromium")
    exit(1)

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


@dataclass
class VisualElement:
    """視覚的に認識されたUI要素"""
    element_type: str  # button, input, link, text, image, etc.
    text: Optional[str] = None
    position: Tuple[int, int, int, int] = (0, 0, 0, 0)  # x, y, width, height
    confidence: float = 0.0
    selector: Optional[str] = None
    screenshot_path: Optional[str] = None


@dataclass
class TabGroup:
    """タブグループ管理"""
    group_id: str
    tabs: List[Page] = field(default_factory=list)
    active_tab_index: int = 0
    metadata: Dict[str, Any] = field(default_factory=dict)


class EnhancedBrowserAutomationEngine:
    """
    ClaudeCoworkスタイルの強化されたブラウザ自動化エンジン
    
    機能:
    - 視覚的理解（スクリーンショット + OCR + AI分析）
    - マルチタブ操作（タブグループ管理）
    - UI要素の自動認識と操作
    - フォーム自動入力の高度化
    """
    
    def __init__(self, headless: bool = False, browser_type: str = "chromium"):
        self.headless = headless
        self.browser_type = browser_type
        self.browser: Optional[Browser] = None
        self.context: Optional[BrowserContext] = None
        self.tab_groups: Dict[str, TabGroup] = {}
        self.temp_dir = Path(tempfile.gettempdir()) / "cowork_browser"
        self.temp_dir.mkdir(parents=True, exist_ok=True)
        
    async def initialize(self):
        """ブラウザ初期化"""
        playwright = await async_playwright().start()
        
        if self.browser_type == "chromium":
            self.browser = await playwright.chromium.launch(
                headless=self.headless,
                args=['--disable-blink-features=AutomationControlled']
            )
        elif self.browser_type == "firefox":
            self.browser = await playwright.firefox.launch(headless=self.headless)
        elif self.browser_type == "webkit":
            self.browser = await playwright.webkit.launch(headless=self.headless)
        else:
            raise ValueError(f"Unsupported browser type: {self.browser_type}")
        
        self.context = await self.browser.new_context(
            viewport={'width': 1280, 'height': 720},
            user_agent='Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36'
        )
        
        logger.info(f"ブラウザ初期化完了: {self.browser_type} (headless={self.headless})")
    
    async def close(self):
        """ブラウザ終了"""
        if self.context:
            await self.context.close()
        if self.browser:
            await self.browser.close()
        logger.info("ブラウザ終了")
    
    async def create_tab_group(self, group_id: str) -> TabGroup:
        """タブグループ作成"""
        if group_id in self.tab_groups:
            return self.tab_groups[group_id]
        
        group = TabGroup(group_id=group_id)
        self.tab_groups[group_id] = group
        logger.info(f"タブグループ作成: {group_id}")
        return group
    
    async def add_tab_to_group(self, group_id: str, url: Optional[str] = None) -> Page:
        """タブグループにタブ追加"""
        if group_id not in self.tab_groups:
            await self.create_tab_group(group_id)
        
        group = self.tab_groups[group_id]
        page = await self.context.new_page()
        
        if url:
            await page.goto(url, wait_until="networkidle")
        
        group.tabs.append(page)
        logger.info(f"タブ追加: {group_id} (total: {len(group.tabs)})")
        return page
    
    async def switch_tab(self, group_id: str, tab_index: int):
        """タブ切り替え"""
        if group_id not in self.tab_groups:
            raise ValueError(f"Tab group not found: {group_id}")
        
        group = self.tab_groups[group_id]
        if tab_index >= len(group.tabs):
            raise ValueError(f"Tab index out of range: {tab_index}")
        
        group.active_tab_index = tab_index
        # Playwrightでは明示的なタブ切り替えは不要（Pageオブジェクトで管理）
        logger.info(f"タブ切り替え: {group_id} -> tab {tab_index}")
    
    async def capture_screenshot(self, page: Page, element_selector: Optional[str] = None) -> str:
        """スクリーンショット取得"""
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        
        if element_selector:
            element = await page.query_selector(element_selector)
            if element:
                screenshot_path = self.temp_dir / f"screenshot_element_{timestamp}.png"
                await element.screenshot(path=str(screenshot_path))
            else:
                raise ValueError(f"Element not found: {element_selector}")
        else:
            screenshot_path = self.temp_dir / f"screenshot_page_{timestamp}.png"
            await page.screenshot(path=str(screenshot_path), full_page=True)
        
        logger.info(f"スクリーンショット保存: {screenshot_path}")
        return str(screenshot_path)
    
    async def extract_text_from_image(self, image_path: str) -> str:
        """OCRで画像からテキスト抽出"""
        try:
            image = Image.open(image_path)
            text = pytesseract.image_to_string(image, lang='jpn+eng')
            logger.info(f"OCR完了: {len(text)}文字抽出")
            return text
        except Exception as e:
            logger.error(f"OCRエラー: {e}")
            return ""
    
    async def analyze_visual_elements(self, page: Page) -> List[VisualElement]:
        """視覚的にUI要素を分析"""
        # スクリーンショット取得
        screenshot_path = await self.capture_screenshot(page)
        
        # OCRでテキスト抽出
        ocr_text = await self.extract_text_from_image(screenshot_path)
        
        # DOM要素と組み合わせて分析
        elements = await page.query_selector_all("button, input, a, select, textarea")
        
        visual_elements = []
        for element in elements:
            try:
                # 要素の位置とサイズ取得
                box = await element.bounding_box()
                if not box:
                    continue
                
                # 要素のテキスト取得
                text = await element.inner_text()
                
                # 要素タイプ判定
                tag_name = await element.evaluate("el => el.tagName.toLowerCase()")
                
                visual_element = VisualElement(
                    element_type=tag_name,
                    text=text or None,
                    position=(int(box['x']), int(box['y']), int(box['width']), int(box['height'])),
                    confidence=0.9 if text else 0.5,
                    selector=await self._generate_selector(element)
                )
                visual_elements.append(visual_element)
            except Exception as e:
                logger.warning(f"要素分析エラー: {e}")
                continue
        
        logger.info(f"視覚要素分析完了: {len(visual_elements)}要素")
        return visual_elements
    
    async def _generate_selector(self, element) -> str:
        """要素のセレクター生成"""
        try:
            # ID優先
            element_id = await element.get_attribute("id")
            if element_id:
                return f"#{element_id}"
            
            # クラス名
            class_name = await element.get_attribute("class")
            if class_name:
                classes = class_name.split()
                if classes:
                    return f".{classes[0]}"
            
            # タグ名 + テキスト
            tag_name = await element.evaluate("el => el.tagName.toLowerCase()")
            text = await element.inner_text()
            if text:
                return f"{tag_name}:has-text('{text[:20]}')"
            
            return tag_name
        except Exception as e:
            logger.warning(f"セレクター生成エラー: {e}")
            return ""
    
    async def find_element_by_text(self, page: Page, text: str, element_type: Optional[str] = None) -> Optional[VisualElement]:
        """テキストで要素検索"""
        visual_elements = await self.analyze_visual_elements(page)
        
        for element in visual_elements:
            if element.text and text.lower() in element.text.lower():
                if element_type is None or element.element_type == element_type:
                    return element
        
        return None
    
    async def click_element(self, page: Page, selector: str, wait_for_navigation: bool = False):
        """要素クリック"""
        try:
            if wait_for_navigation:
                async with page.expect_navigation():
                    await page.click(selector)
            else:
                await page.click(selector)
            logger.info(f"クリック完了: {selector}")
        except PlaywrightTimeout:
            logger.warning(f"クリックタイムアウト: {selector}")
            raise
        except Exception as e:
            logger.error(f"クリックエラー: {e}")
            raise
    
    async def fill_form(self, page: Page, form_data: Dict[str, str]):
        """フォーム自動入力"""
        for field_name, value in form_data.items():
            try:
                # 複数のセレクター戦略を試行
                selectors = [
                    f"input[name='{field_name}']",
                    f"input[id='{field_name}']",
                    f"input[placeholder*='{field_name}']",
                    f"textarea[name='{field_name}']",
                    f"textarea[id='{field_name}']",
                ]
                
                filled = False
                for selector in selectors:
                    try:
                        element = await page.query_selector(selector)
                        if element:
                            await element.fill(value)
                            filled = True
                            logger.info(f"フォーム入力: {field_name} = {value[:20]}...")
                            break
                    except Exception:
                        continue
                
                if not filled:
                    # テキスト検索で試行
                    visual_element = await self.find_element_by_text(page, field_name, "input")
                    if visual_element and visual_element.selector:
                        await page.fill(visual_element.selector, value)
                        logger.info(f"フォーム入力（視覚検索）: {field_name} = {value[:20]}...")
                    else:
                        logger.warning(f"フォームフィールド見つからず: {field_name}")
            except Exception as e:
                logger.error(f"フォーム入力エラー ({field_name}): {e}")
                continue
    
    async def extract_data(self, page: Page, extraction_rules: Dict[str, str]) -> Dict[str, Any]:
        """データ抽出（セレクタールールベース）"""
        extracted_data = {}
        
        for key, selector in extraction_rules.items():
            try:
                elements = await page.query_selector_all(selector)
                if elements:
                    if len(elements) == 1:
                        extracted_data[key] = await elements[0].inner_text()
                    else:
                        extracted_data[key] = [await el.inner_text() for el in elements]
                else:
                    extracted_data[key] = None
            except Exception as e:
                logger.warning(f"データ抽出エラー ({key}): {e}")
                extracted_data[key] = None
        
        logger.info(f"データ抽出完了: {len(extracted_data)}項目")
        return extracted_data
    
    async def execute_workflow(self, page: Page, workflow: List[Dict[str, Any]]) -> Dict[str, Any]:
        """ワークフロー実行"""
        results = []
        
        for step in workflow:
            step_type = step.get("type")
            step_data = step.get("data", {})
            
            try:
                if step_type == "navigate":
                    url = step_data.get("url")
                    await page.goto(url, wait_until="networkidle")
                    results.append({"step": step_type, "status": "success", "url": url})
                
                elif step_type == "click":
                    selector = step_data.get("selector")
                    await self.click_element(page, selector)
                    results.append({"step": step_type, "status": "success", "selector": selector})
                
                elif step_type == "fill_form":
                    form_data = step_data.get("form_data", {})
                    await self.fill_form(page, form_data)
                    results.append({"step": step_type, "status": "success", "fields": list(form_data.keys())})
                
                elif step_type == "extract":
                    rules = step_data.get("rules", {})
                    data = await self.extract_data(page, rules)
                    results.append({"step": step_type, "status": "success", "data": data})
                
                elif step_type == "screenshot":
                    path = await self.capture_screenshot(page)
                    results.append({"step": step_type, "status": "success", "path": path})
                
                elif step_type == "wait":
                    duration = step_data.get("duration", 1)
                    await asyncio.sleep(duration)
                    results.append({"step": step_type, "status": "success"})
                
                else:
                    logger.warning(f"未知のステップタイプ: {step_type}")
                    results.append({"step": step_type, "status": "skipped"})
            
            except Exception as e:
                logger.error(f"ワークフローステップエラー: {step_type} - {e}")
                results.append({"step": step_type, "status": "error", "error": str(e)})
        
        return {"workflow_results": results, "success_count": sum(1 for r in results if r.get("status") == "success")}


async def main():
    """テスト実行"""
    engine = EnhancedBrowserAutomationEngine(headless=False)
    
    try:
        await engine.initialize()
        
        # タブグループ作成
        group = await engine.create_tab_group("test_group")
        
        # タブ追加
        page = await engine.add_tab_to_group("test_group", "https://example.com")
        
        # 視覚要素分析
        elements = await engine.analyze_visual_elements(page)
        print(f"検出された要素数: {len(elements)}")
        
        # スクリーンショット
        screenshot = await engine.capture_screenshot(page)
        print(f"スクリーンショット: {screenshot}")
        
        # ワークフロー実行例
        workflow = [
            {"type": "navigate", "data": {"url": "https://example.com"}},
            {"type": "screenshot", "data": {}},
            {"type": "extract", "data": {"rules": {"title": "h1"}}}
        ]
        result = await engine.execute_workflow(page, workflow)
        print(f"ワークフロー結果: {json.dumps(result, indent=2, ensure_ascii=False)}")
        
    finally:
        await engine.close()


if __name__ == "__main__":
    asyncio.run(main())
