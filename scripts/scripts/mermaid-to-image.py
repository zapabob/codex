#!/usr/bin/env python3
"""
Mermaid図をSVG/PNG形式に変換（mermaid.ink API使用）
"""

import argparse
import base64
import zlib
import requests
from pathlib import Path


def convert_mermaid_to_svg(mermaid_file: Path, output_svg: Path):
    """Mermaid図をSVG形式に変換"""

    # Mermaidファイル読み込み
    with open(mermaid_file, "r", encoding="utf-8") as f:
        content = f.read()

    # ```mermaid 除去
    content = content.replace("```mermaid", "").replace("```", "").strip()

    # mermaid.ink APIでSVG生成
    # Method 1: kroki API (推奨)
    try:
        print(f"🔄 Converting {mermaid_file.name} to SVG...")

        # zlibで圧縮してbase64エンコード
        compressed = zlib.compress(content.encode("utf-8"), level=9)
        encoded = base64.urlsafe_b64encode(compressed).decode("utf-8")

        # kroki API URL
        url = f"https://kroki.io/mermaid/svg/{encoded}"

        # SVG取得
        response = requests.get(url, timeout=30)
        response.raise_for_status()

        # SVG保存
        with open(output_svg, "wb") as f:
            f.write(response.content)

        print(f"✅ SVG created: {output_svg}")
        return True

    except Exception as e:
        print(f"⚠️ kroki API failed: {e}")

        # Method 2: mermaid.ink API (fallback)
        try:
            print("🔄 Trying mermaid.ink API...")
            import urllib.parse

            encoded = urllib.parse.quote(content)
            url = f"https://mermaid.ink/svg/{encoded}"

            response = requests.get(url, timeout=30)
            response.raise_for_status()

            with open(output_svg, "wb") as f:
                f.write(response.content)

            print(f"✅ SVG created (mermaid.ink): {output_svg}")
            return True

        except Exception as e2:
            print(f"❌ All methods failed: {e2}")
            return False


def convert_svg_to_png(svg_file: Path, png_file: Path, width: int = 2400):
    """SVGをPNG形式に変換"""

    try:
        from PIL import Image
        import cairosvg

        print(f"🔄 Converting SVG to PNG...")

        # SVG → PNG (cairosvg)
        cairosvg.svg2png(url=str(svg_file), write_to=str(png_file), output_width=width)

        print(f"✅ PNG created: {png_file}")
        return True

    except ImportError:
        print("⚠️ cairosvg not installed. Install: pip install cairosvg")
        print(f"ℹ️ SVG is available at: {svg_file}")
        print("You can convert it manually using:")
        print("  - Online: https://cloudconvert.com/svg-to-png")
        print("  - ImageMagick: magick convert -density 300 input.svg output.png")
        return False
    except Exception as e:
        print(f"⚠️ PNG conversion failed: {e}")
        print(f"ℹ️ SVG is available at: {svg_file}")
        return False


def main():
    parser = argparse.ArgumentParser(description="Convert Mermaid to SVG/PNG")
    parser.add_argument("input", type=Path, help="Input .mmd file")
    parser.add_argument("--output-dir", "-o", type=Path, help="Output directory")
    parser.add_argument(
        "--width", "-w", type=int, default=2400, help="PNG width (default: 2400)"
    )
    parser.add_argument(
        "--svg-only", action="store_true", help="Generate SVG only (skip PNG)"
    )

    args = parser.parse_args()

    # 出力ディレクトリ
    output_dir = args.output_dir if args.output_dir else args.input.parent

    # ファイル名
    base_name = args.input.stem
    svg_file = output_dir / f"{base_name}.svg"
    png_file = output_dir / f"{base_name}.png"

    # SVG変換
    if not convert_mermaid_to_svg(args.input, svg_file):
        sys.exit(1)

    # PNG変換（オプション）
    if not args.svg_only:
        convert_svg_to_png(svg_file, png_file, args.width)

    print("")
    print("🎉 Conversion complete!")
    print(f"📁 Output directory: {output_dir}")


if __name__ == "__main__":
    main()
