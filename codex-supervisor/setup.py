#!/usr/bin/env python3

from setuptools import setup, find_packages
import os

# Read the contents of README file
this_directory = os.path.abspath(os.path.dirname(__file__))
with open(os.path.join(this_directory, 'README.md'), encoding='utf-8') as f:
    long_description = f.read()

setup(
    name="codex-supervisor",
    version="1.0.0",
    author="Zapabob",
    author_email="",
    description="Official OpenAI Codex Agents SDK Supervisor - MCP-Centric Multi-Agent Orchestrator",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/zapabob/codex-supervisor",
    packages=find_packages(),
    classifiers=[
        "Development Status :: 4 - Beta",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: Apache Software License",
        "Operating System :: OS Independent",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: 3.12",
        "Topic :: Software Development :: Libraries :: Python Modules",
        "Topic :: Software Development :: Quality Assurance",
        "Topic :: System :: Distributed Computing",
    ],
    keywords="codex openai agents sdk mcp supervisor orchestrator",
    python_requires=">=3.8",
    install_requires=[
        "websockets>=12.0",
        "asyncio-compat>=1.0.0",
        "pathlib>=1.0.1",
        "typing-extensions>=4.0.0",
    ],
    extras_require={
        "dev": [
            "pytest>=7.0.0",
            "pytest-asyncio>=0.21.0",
            "black>=22.0.0",
            "isort>=5.10.0",
            "mypy>=1.0.0",
        ],
        "docs": [
            "sphinx>=5.0.0",
            "sphinx-rtd-theme>=1.2.0",
        ],
    },
    entry_points={
        "console_scripts": [
            "codex-supervisor=supervisor:main",
        ],
    },
    include_package_data=True,
    zip_safe=False,
)