#!/usr/bin/env python3
"""
Mermaid図をSVG/PNG形式に変換するスクリプト
"""

import argparse
import subprocess
import sys
from pathlib import Path


def convert_mermaid_to_images(mermaid_file: Path, output_dir: Path = None):
    """MermaidファイルをSVGとPNGに変換"""

    if not mermaid_file.exists():
        print(f"❌ Error: File not found: {mermaid_file}")
        return False

    # 出力ディレクトリ
    if output_dir is None:
        output_dir = mermaid_file.parent
    output_dir.mkdir(parents=True, exist_ok=True)

    # 出力ファイル名
    base_name = mermaid_file.stem
    svg_output = output_dir / f"{base_name}.svg"
    png_output = output_dir / f"{base_name}.png"

    print(f"🔄 Converting {mermaid_file.name}...")

    # mermaid.inkを使用してSVG生成
    try:
        cmd = ["npx", "-y", "mermaid.ink", str(mermaid_file), "-o", str(svg_output)]
        result = subprocess.run(cmd, capture_output=True, text=True)

        if result.returncode == 0:
            print(f"✅ SVG created: {svg_output}")
        else:
            print(f"⚠️ mermaid.ink failed, trying alternative method...")
            # 代替: mmdc
            cmd_alt = [
                "docker",
                "run",
                "--rm",
                "-v",
                f"{mermaid_file.parent.absolute()}:/data",
                "minlag/mermaid-cli",
                "-i",
                f"/data/{mermaid_file.name}",
                "-o",
                f"/data/{svg_output.name}",
            ]
            result_alt = subprocess.run(cmd_alt, capture_output=True, text=True)

            if result_alt.returncode != 0:
                print(f"❌ SVG conversion failed")
                return False

            print(f"✅ SVG created (via Docker): {svg_output}")
    except Exception as e:
        print(f"❌ Error during SVG conversion: {e}")
        return False

    # SVGをPNGに変換（ImageMagickまたはinkscape）
    if svg_output.exists():
        try:
            # Try ImageMagick first
            cmd_png = [
                "magick",
                "convert",
                "-density",
                "300",
                "-background",
                "white",
                "-alpha",
                "remove",
                str(svg_output),
                str(png_output),
            ]
            result = subprocess.run(cmd_png, capture_output=True, text=True)

            if result.returncode == 0:
                print(f"✅ PNG created: {png_output}")
                return True
            else:
                print(f"⚠️ ImageMagick not available, using Python PIL...")
                # 代替: cairosvg
                import cairosvg

                cairosvg.svg2png(
                    url=str(svg_output),
                    write_to=str(png_output),
                    output_width=2400,
                    output_height=1800,
                )
                print(f"✅ PNG created (via cairosvg): {png_output}")
                return True

        except Exception as e:
            print(f"⚠️ PNG conversion failed: {e}")
            print(f"ℹ️ SVG is available at: {svg_output}")
            return True  # SVGは成功したのでTrueを返す

    return False


def main():
    parser = argparse.ArgumentParser(description="Convert Mermaid diagrams to SVG/PNG")
    parser.add_argument("input", type=Path, help="Input .mmd file")
    parser.add_argument(
        "--output-dir",
        "-o",
        type=Path,
        help="Output directory (default: same as input)",
    )

    args = parser.parse_args()

    success = convert_mermaid_to_images(args.input, args.output_dir)
    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()
