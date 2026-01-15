#!/usr/bin/env python3
"""
Architect Agent - System Architecture Analysis and ADR Generation
"""

import os
import sys
import json
from pathlib import Path
from typing import Dict, List, Any
from datetime import datetime

def analyze_project_structure(project_root: Path) -> Dict[str, Any]:
    """Analyze project structure and dependencies"""

    analysis = {
        "languages": {},
        "frameworks": [],
        "architecture_patterns": [],
        "entry_points": [],
        "config_files": [],
        "test_coverage": "unknown"
    }

    # Language detection
    extensions = {}
    for file_path in project_root.rglob('*'):
        if file_path.is_file():
            ext = file_path.suffix.lower()
            extensions[ext] = extensions.get(ext, 0) + 1

    # Primary languages
    if '.rs' in extensions:
        analysis["languages"]["Rust"] = extensions.get('.rs', 0)
    if '.py' in extensions:
        analysis["languages"]["Python"] = extensions.get('.py', 0)
    if '.js' in extensions or '.ts' in extensions:
        analysis["languages"]["JavaScript/TypeScript"] = extensions.get('.js', 0) + extensions.get('.ts', 0)
    if '.go' in extensions:
        analysis["languages"]["Go"] = extensions.get('.go', 0)

    # Framework detection
    cargo_toml = project_root / "Cargo.toml"
    if cargo_toml.exists():
        analysis["frameworks"].append("Rust/Cargo")

    package_json = project_root / "package.json"
    if package_json.exists():
        analysis["frameworks"].append("Node.js/npm")

    # Architecture patterns
    if (project_root / "src").exists():
        analysis["architecture_patterns"].append("Standard src/ layout")
    if (project_root / "internal").exists():
        analysis["architecture_patterns"].append("Go-style internal packages")
    if (project_root / "pkg").exists():
        analysis["architecture_patterns"].append("Go-style pkg packages")

    # Entry points
    main_rs = project_root / "src" / "main.rs"
    lib_rs = project_root / "src" / "lib.rs"
    if main_rs.exists():
        analysis["entry_points"].append("src/main.rs (binary)")
    if lib_rs.exists():
        analysis["entry_points"].append("src/lib.rs (library)")

    return analysis

def detect_design_patterns(project_root: Path) -> List[str]:
    """Detect common design patterns in the codebase"""

    patterns = []

    # Search for common patterns
    for rs_file in project_root.rglob('*.rs'):
        try:
            with open(rs_file, 'r', encoding='utf-8') as f:
                content = f.read()

                # Singleton pattern
                if 'lazy_static' in content or 'OnceCell' in content:
                    patterns.append("Singleton pattern")

                # Factory pattern
                if 'factory' in content.lower() or 'Factory' in content:
                    patterns.append("Factory pattern")

                # Builder pattern
                if 'builder' in content.lower() or 'Builder' in content:
                    patterns.append("Builder pattern")

                # Observer pattern
                if 'observer' in content.lower() or 'notify' in content and 'subscribe' in content:
                    patterns.append("Observer pattern")

                # Strategy pattern
                if 'strategy' in content.lower() or 'Strategy' in content:
                    patterns.append("Strategy pattern")

        except UnicodeDecodeError:
            continue

    return list(set(patterns))  # Remove duplicates

def generate_adr_template(title: str, context: str = "") -> str:
    """Generate an Architectural Decision Record template"""

    date = datetime.now().strftime("%Y-%m-%d")

    adr = f"""# ADR: {title}

## Status
Proposed | Accepted | Rejected | Deprecated | Superseded by [ADR-XXXX](XXXX-adr-title.md)

## Context
{context}

## Decision
[Brief description of the decision]

## Consequences
### Positive
- [List positive consequences]

### Negative
- [List negative consequences]

### Risks
- [List potential risks]

## Alternatives Considered
- [Alternative 1]
  - Pros: [pros]
  - Cons: [cons]

- [Alternative 2]
  - Pros: [pros]
  - Cons: [cons]

## References
- [Link to relevant documentation]
- [Link to related ADRs]

---

*Date: {date}*
*Status: Proposed*
"""

    return adr

def assess_scalability(project_root: Path) -> Dict[str, Any]:
    """Assess scalability characteristics"""

    assessment = {
        "concurrency_model": "unknown",
        "data_layer": "unknown",
        "caching_strategy": "unknown",
        "bottlenecks": [],
        "recommendations": []
    }

    # Check for async patterns
    async_found = False
    tokio_found = False
    thread_found = False

    for rs_file in project_root.rglob('*.rs'):
        try:
            with open(rs_file, 'r', encoding='utf-8') as f:
                content = f.read()

                if 'async' in content or 'await' in content:
                    async_found = True
                if 'tokio' in content:
                    tokio_found = True
                if 'std::thread' in content or 'spawn' in content:
                    thread_found = True

        except UnicodeDecodeError:
            continue

    if tokio_found:
        assessment["concurrency_model"] = "Tokio async runtime"
    elif async_found:
        assessment["concurrency_model"] = "Async/Await patterns"
    elif thread_found:
        assessment["concurrency_model"] = "Multi-threading"
    else:
        assessment["concurrency_model"] = "Single-threaded"

    # Database detection
    if any((project_root / f).exists() for f in ["diesel.toml", "sea-orm.toml"]):
        assessment["data_layer"] = "ORM detected"
    elif "sql" in " ".join(str(f) for f in project_root.rglob('*')).lower():
        assessment["data_layer"] = "SQL usage detected"

    # Recommendations
    if assessment["concurrency_model"] == "Single-threaded":
        assessment["recommendations"].append("Consider adopting async patterns for better concurrency")

    if not assessment["data_layer"]:
        assessment["recommendations"].append("Consider implementing proper data persistence layer")

    return assessment

def run_architect_analysis():
    """Run comprehensive architect analysis"""

    print("[SEARCH] Architect Agent: Analyzing system architecture...")

    # Get project root (assuming script is run from project root)
    project_root = Path.cwd()

    # Create artifacts directory
    artifacts_dir = project_root / "artifacts"
    artifacts_dir.mkdir(exist_ok=True)

    # Perform analyses
    print("  [INFO] Analyzing project structure...")
    structure = analyze_project_structure(project_root)

    print("  [INFO] Detecting design patterns...")
    patterns = detect_design_patterns(project_root)

    print("  [INFO] Assessing scalability...")
    scalability = assess_scalability(project_root)

    # Generate comprehensive report
    report_path = artifacts_dir / "architecture_analysis.md"
    print(f"  [SAVE] Generating report: {report_path}")

    with open(report_path, 'w', encoding='utf-8') as f:
        f.write("# Architecture Analysis Report\n\n")
        f.write(f"Generated by Architect Skill on {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n\n")

        f.write("## Project Overview\n\n")
        f.write(f"- **Languages**: {', '.join(structure['languages'].keys())}\n")
        f.write(f"- **Frameworks**: {', '.join(structure['frameworks'])}\n")
        f.write(f"- **Entry Points**: {', '.join(structure['entry_points'])}\n\n")

        f.write("## Architecture Patterns\n\n")
        for pattern in structure['architecture_patterns']:
            f.write(f"- {pattern}\n")
        f.write("\n")

        f.write("## Design Patterns Detected\n\n")
        if patterns:
            for pattern in patterns:
                f.write(f"- {pattern}\n")
        else:
            f.write("No specific design patterns detected in the current analysis.\n")
        f.write("\n")

        f.write("## Scalability Assessment\n\n")
        f.write(f"- **Concurrency Model**: {scalability['concurrency_model']}\n")
        f.write(f"- **Data Layer**: {scalability['data_layer']}\n")
        f.write(f"- **Caching Strategy**: {scalability['caching_strategy']}\n\n")

        if scalability['recommendations']:
            f.write("### Recommendations\n\n")
            for rec in scalability['recommendations']:
                f.write(f"- {rec}\n")
            f.write("\n")

        f.write("## Generated Files\n\n")
        f.write("- This architecture analysis report\n")
        f.write("- ADR template (see below)\n\n")

        f.write("---\n\n")

        # Include ADR template
        f.write(generate_adr_template("System Architecture Decisions", "Initial architecture assessment"))

    print("[OK] Architecture analysis completed")
    print(f"[DIR] Report saved to: {report_path}")
    print("[INFO] ADR template included in report")

if __name__ == "__main__":
    run_architect_analysis()
