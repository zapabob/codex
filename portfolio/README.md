## Portfolio Guide（採用担当者向けの“見る順番”）

この`portfolio/`は、既存ファイルを動かさずに「見せ方」だけ整理した導線です。

### まず5分で全体像（面接官が最初に見たい順）

1. **Plan Modeが本当に再現できるか**  
   - `docs/plan/README.md`
2. **定量的な裏付け（速度/品質）**  
   - `docs/benchmarks/README.md`
   - `docs/benchmarks/subagents.md`
   - `docs/benchmarks/cuda.md`
3. **アーキテクチャ俯瞰（技術の射程）**  
   - `ARCHITECTURE.md`
4. **実務っぽい成果物（サンプルで語れる）**  
   - `examples/README.md`
5. **GUIで“画面デモ”**（Plan運用 + 可視化の入口）  
   - `gui/README.md` / `extensions/codex-viz-web/README.md`

### 深掘りしたい人向け（フォルダ別の入口）

- **Overview**: `portfolio/01_overview/README.md`
- **System / Components**: `portfolio/02_system/README.md`
- **Evidence（ベンチ・再現・品質）**: `portfolio/03_evidence/README.md`
- **Run / Demo（ローカルで見せる）**: `portfolio/04_run/README.md`

