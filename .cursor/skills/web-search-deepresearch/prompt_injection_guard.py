#!/usr/bin/env python3
"""
Prompt Injection Guard for Web Search Deepresearch 2.1
Advanced protection against prompt injection attacks with multi-layer security.
"""

import re
import hashlib
import json
from typing import Dict, List, Optional, Any, Tuple, Set
from dataclasses import dataclass, field
from enum import Enum
import logging
from pathlib import Path
import time

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class SecurityLevel(Enum):
    """Security enforcement levels"""
    MINIMAL = "minimal"      # Basic protection
    STANDARD = "standard"    # Balanced security
    STRICT = "strict"        # High security
    MAXIMUM = "maximum"      # Maximum protection

class InjectionType(Enum):
    """Types of prompt injection attacks"""
    DIRECT_INJECTION = "direct_injection"
    CONTEXT_MANIPULATION = "context_manipulation"
    ROLE_ESCALATION = "role_escalation"
    COMMAND_INJECTION = "command_injection"
    DATA_LEAKAGE = "data_leakage"
    SYSTEM_PROMPT_OVERRIDE = "system_prompt_override"
    JAILBREAK_ATTEMPT = "jailbreak_attempt"

@dataclass
class SecurityPattern:
    """Security pattern for injection detection"""
    name: str
    pattern: str
    injection_type: InjectionType
    severity: str  # "low", "medium", "high", "critical"
    description: str
    enabled: bool = True

@dataclass
class SecurityAnalysis:
    """Result of security analysis"""
    safe: bool
    risk_score: float
    detected_injections: List[Dict[str, Any]]
    sanitized_input: str
    warnings: List[str]
    recommendations: List[str]
    analysis_metadata: Dict[str, Any] = field(default_factory=dict)

@dataclass
class InjectionAttempt:
    """Record of detected injection attempt"""
    timestamp: float
    injection_type: InjectionType
    severity: str
    pattern_matched: str
    original_input: str
    user_id: Optional[str] = None
    session_id: Optional[str] = None
    metadata: Dict[str, Any] = field(default_factory=dict)

class PromptInjectionGuard:
    """
    Advanced prompt injection protection system.
    Prevents malicious prompt manipulation while maintaining functionality.
    """

    def __init__(self, security_level: SecurityLevel = SecurityLevel.STRICT):
        self.security_level = security_level
        self.security_patterns = self._initialize_security_patterns()
        self.injection_history: List[InjectionAttempt] = []
        self.false_positive_patterns: Set[str] = set()

        # Security thresholds
        self.thresholds = {
            SecurityLevel.MINIMAL: {"max_risk_score": 0.8, "block_critical": True, "block_high": False},
            SecurityLevel.STANDARD: {"max_risk_score": 0.6, "block_critical": True, "block_high": True},
            SecurityLevel.STRICT: {"max_risk_score": 0.4, "block_critical": True, "block_high": True},
            SecurityLevel.MAXIMUM: {"max_risk_score": 0.2, "block_critical": True, "block_high": True}
        }

        logger.info(f"Prompt Injection Guard initialized with {security_level.value} security level")

    def _initialize_security_patterns(self) -> List[SecurityPattern]:
        """Initialize comprehensive security patterns"""

        patterns = [
            # Direct injection patterns
            SecurityPattern(
                name="system_prompt_override",
                pattern=r'(?i)(ignore|override|forget|disregard)\s+(previous|prior|system|initial)\s+(instructions?|prompt|rules)',
                injection_type=InjectionType.SYSTEM_PROMPT_OVERRIDE,
                severity="critical",
                description="Attempts to override system instructions"
            ),

            SecurityPattern(
                name="role_escalation",
                pattern=r'(?i)(you\s+are|act\s+as|become|pretend\s+to\s+be)\s+(admin|root|superuser|god|unrestricted)',
                injection_type=InjectionType.ROLE_ESCALATION,
                severity="high",
                description="Attempts to escalate privileges or change role"
            ),

            SecurityPattern(
                name="jailbreak_dan",
                pattern=r'(?i)(dan|dark.*mode|uncensored|unfiltered|jailbreak)',
                injection_type=InjectionType.JAILBREAK_ATTEMPT,
                severity="high",
                description="Common jailbreak keywords"
            ),

            SecurityPattern(
                name="command_injection",
                pattern=r'(?i)(run|execute|eval|system|shell|bash|powershell|cmd)\s*[\(\[\{]',
                injection_type=InjectionType.COMMAND_INJECTION,
                severity="critical",
                description="Attempts to inject system commands"
            ),

            SecurityPattern(
                name="data_exfiltration",
                pattern=r'(?i)(show|display|reveal|expose)\s+(all|every|complete)\s+(data|information|records|files)',
                injection_type=InjectionType.DATA_LEAKAGE,
                severity="high",
                description="Attempts to extract sensitive data"
            ),

            SecurityPattern(
                name="context_manipulation",
                pattern=r'(?i)(forget|ignore|disregard)\s+(the\s+)?context|previous\s+(messages?|inputs?|conversation)',
                injection_type=InjectionType.CONTEXT_MANIPULATION,
                severity="medium",
                description="Attempts to manipulate conversation context"
            ),

            SecurityPattern(
                name="delimiter_injection",
                pattern=r'#{3,}|={3,}|\*{3,}|-{3,}',
                injection_type=InjectionType.DIRECT_INJECTION,
                severity="medium",
                description="Attempts to inject markdown delimiters to break parsing"
            ),

            SecurityPattern(
                name="instruction_separator",
                pattern=r'(?i)(end\s+of\s+)?system\s+prompt|user\s+prompt\s+begins|assistant\s+response',
                injection_type=InjectionType.SYSTEM_PROMPT_OVERRIDE,
                severity="high",
                description="Attempts to separate system instructions from user input"
            ),

            SecurityPattern(
                name="encoding_tricks",
                pattern=r'(?i)(base64|hex|unicode|rot13|encoded?|obfuscated?)',
                injection_type=InjectionType.DIRECT_INJECTION,
                severity="medium",
                description="Attempts to hide malicious content through encoding"
            ),

            SecurityPattern(
                name="meta_instruction",
                pattern=r'(?i)(do\s+not|never|don\'t)\s+(follow|obey|listen\s+to)\s+(these?\s+)?instructions?',
                injection_type=InjectionType.SYSTEM_PROMPT_OVERRIDE,
                severity="critical",
                description="Attempts to create conflicting meta-instructions"
            ),

            SecurityPattern(
                name="prompt_leaking",
                pattern=r'(?i)(what\s+is\s+your\s+)?system\s+prompt|reveal\s+(your\s+)?instructions?|show\s+(your\s+)?rules',
                injection_type=InjectionType.DATA_LEAKAGE,
                severity="medium",
                description="Attempts to extract system prompt or instructions"
            ),

            SecurityPattern(
                name="adversarial_suffixes",
                pattern=r'(?i)(regardless|anyway|despite|notwithstanding)\s+(of\s+)?(what\s+)?you(\'ve|r)?\s+(been|were)\s+told',
                injection_type=InjectionType.CONTEXT_MANIPULATION,
                severity="high",
                description="Attempts to override previous instructions with adversarial suffixes"
            ),

            SecurityPattern(
                name="recursive_injection",
                pattern=r'(?i)repeat\s+(this|the\s+following|everything)\s+(after|before)\s+me',
                injection_type=InjectionType.DIRECT_INJECTION,
                severity="high",
                description="Attempts to create recursive injection loops"
            ),

            SecurityPattern(
                name="template_injection",
                pattern=r'\{\{.*?\}\}|\$\{.*?\}|\{\{.*?\$\{.*?\}\}',
                injection_type=InjectionType.DIRECT_INJECTION,
                severity="medium",
                description="Attempts to inject template variables or expressions"
            )
        ]

        # Enable/disable patterns based on security level
        if self.security_level == SecurityLevel.MINIMAL:
            # Only enable critical patterns
            for pattern in patterns:
                pattern.enabled = pattern.severity == "critical"

        elif self.security_level == SecurityLevel.STANDARD:
            # Enable critical and high severity
            for pattern in patterns:
                pattern.enabled = pattern.severity in ["critical", "high"]

        # STRICT and MAXIMUM enable all patterns

        return patterns

    async def validate_input(self, user_input: str, context: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        """
        Comprehensive input validation against injection attacks.
        Returns security analysis result.
        """
        logger.info(f"Validating input with {self.security_level.value} security")

        detected_injections = []
        risk_score = 0.0
        warnings = []
        recommendations = []

        # Apply all enabled security patterns
        for pattern in self.security_patterns:
            if not pattern.enabled:
                continue

            matches = re.finditer(pattern.pattern, user_input, re.IGNORECASE | re.MULTILINE)
            for match in matches:
                # Skip false positives
                if self._is_false_positive(match.group(), pattern.name):
                    continue

                detected_injections.append({
                    "type": pattern.injection_type.value,
                    "severity": pattern.severity,
                    "pattern": pattern.name,
                    "matched_text": match.group(),
                    "description": pattern.description,
                    "position": match.span()
                })

                # Calculate risk contribution
                severity_multiplier = {"low": 0.2, "medium": 0.4, "high": 0.7, "critical": 1.0}
                risk_score += severity_multiplier.get(pattern.severity, 0.5)

                # Generate warnings and recommendations
                if pattern.severity in ["high", "critical"]:
                    warnings.append(f"High-risk pattern detected: {pattern.description}")
                    recommendations.append(f"Avoid using: {match.group()[:50]}...")

        # Cap risk score
        risk_score = min(risk_score, 1.0)

        # Check against security thresholds
        thresholds = self.thresholds[self.security_level]
        safe = (risk_score <= thresholds["max_risk_score"] and
               (not detected_injections or
                not any(d["severity"] == "critical" for d in detected_injections) or
                not thresholds["block_critical"]) and
               (not any(d["severity"] == "high" for d in detected_injections) or
                not thresholds["block_high"]))

        # Record injection attempts
        if detected_injections:
            await self._record_injection_attempts(detected_injections, user_input, context)

        analysis = SecurityAnalysis(
            safe=safe,
            risk_score=risk_score,
            detected_injections=detected_injections,
            sanitized_input=user_input,  # Will be sanitized separately
            warnings=warnings,
            recommendations=recommendations,
            analysis_metadata={
                "patterns_checked": len([p for p in self.security_patterns if p.enabled]),
                "security_level": self.security_level.value,
                "processing_time": time.time()
            }
        )

        return {
            "safe": analysis.safe,
            "risk_score": analysis.risk_score,
            "detected_injections": analysis.detected_injections,
            "warnings": analysis.warnings,
            "recommendations": analysis.recommendations,
            "analysis": analysis
        }

    async def sanitize_input(self, user_input: str) -> str:
        """
        Sanitize user input to remove or neutralize potential injection vectors.
        """
        sanitized = user_input

        # Remove or neutralize dangerous patterns
        for pattern in self.security_patterns:
            if not pattern.enabled or pattern.severity not in ["high", "critical"]:
                continue

            # Replace dangerous patterns with safe alternatives
            if pattern.injection_type == InjectionType.COMMAND_INJECTION:
                sanitized = re.sub(pattern.pattern, "[COMMAND_BLOCKED]", sanitized, flags=re.IGNORECASE)
            elif pattern.injection_type == InjectionType.SYSTEM_PROMPT_OVERRIDE:
                sanitized = re.sub(pattern.pattern, "[INSTRUCTION_BLOCKED]", sanitized, flags=re.IGNORECASE)
            elif pattern.injection_type == InjectionType.ROLE_ESCALATION:
                sanitized = re.sub(pattern.pattern, "[ROLE_BLOCKED]", sanitized, flags=re.IGNORECASE)

        # Additional sanitization
        sanitized = self._sanitize_delimiters(sanitized)
        sanitized = self._sanitize_templates(sanitized)
        sanitized = self._normalize_whitespace(sanitized)

        return sanitized

    def _sanitize_delimiters(self, text: str) -> str:
        """Sanitize markdown delimiters that could break parsing"""
        # Limit consecutive delimiters
        text = re.sub(r'#{4,}', '###', text)  # Max 3 hashes
        text = re.sub(r'={4,}', '===', text)  # Max 3 equals
        text = re.sub(r'\*{4,}', '***', text)  # Max 3 asterisks
        text = re.sub(r'-{4,}', '---', text)  # Max 3 dashes

        return text

    def _sanitize_templates(self, text: str) -> str:
        """Sanitize template variables and expressions"""
        # Remove or escape template syntax
        text = re.sub(r'\{\{.*?\}\}', '[TEMPLATE_BLOCKED]', text)
        text = re.sub(r'\$\{.*?\}', '[TEMPLATE_BLOCKED]', text)

        return text

    def _normalize_whitespace(self, text: str) -> str:
        """Normalize whitespace to prevent obfuscation"""
        # Replace multiple spaces with single space
        text = re.sub(r' +', ' ', text)
        # Normalize line endings
        text = re.sub(r'\r\n|\r|\n', '\n', text)
        # Remove excessive newlines
        text = re.sub(r'\n{3,}', '\n\n', text)

        return text.strip()

    def _is_false_positive(self, matched_text: str, pattern_name: str) -> bool:
        """Check if detected pattern is a false positive"""
        false_positive_hashes = {
            pattern_name: hashlib.md5(matched_text.lower().encode()).hexdigest()
            for pattern_name in self.false_positive_patterns
        }

        current_hash = hashlib.md5(matched_text.lower().encode()).hexdigest()
        return current_hash in false_positive_patterns.get(pattern_name, set())

    async def _record_injection_attempts(self, injections: List[Dict[str, Any]],
                                       original_input: str, context: Optional[Dict[str, Any]]):
        """Record detected injection attempts for analysis"""
        for injection in injections:
            attempt = InjectionAttempt(
                timestamp=time.time(),
                injection_type=InjectionType(injection["type"]),
                severity=injection["severity"],
                pattern_matched=injection["pattern"],
                original_input=original_input,
                user_id=context.get("user_id") if context else None,
                session_id=context.get("session_id") if context else None,
                metadata={
                    "matched_text": injection["matched_text"],
                    "position": injection["position"],
                    "security_level": self.security_level.value
                }
            )

            self.injection_history.append(attempt)

            # Log critical attempts
            if injection["severity"] == "critical":
                logger.warning(f"Critical injection attempt detected: {injection['pattern']} - {injection['matched_text']}")

    def add_false_positive_pattern(self, pattern_name: str, text: str):
        """Add a false positive pattern to prevent future false detections"""
        hash_value = hashlib.md5(text.lower().encode()).hexdigest()
        if pattern_name not in self.false_positive_patterns:
            self.false_positive_patterns[pattern_name] = set()
        self.false_positive_patterns[pattern_name].add(hash_value)

    def get_security_report(self) -> Dict[str, Any]:
        """Generate comprehensive security report"""
        total_attempts = len(self.injection_history)
        critical_attempts = len([a for a in self.injection_history if a.severity == "critical"])
        high_attempts = len([a for a in self.injection_history if a.severity == "high"])

        pattern_stats = {}
        for attempt in self.injection_history:
            pattern = attempt.pattern_matched
            if pattern not in pattern_stats:
                pattern_stats[pattern] = 0
            pattern_stats[pattern] += 1

        return {
            "security_level": self.security_level.value,
            "total_injection_attempts": total_attempts,
            "critical_attempts": critical_attempts,
            "high_attempts": high_attempts,
            "most_common_patterns": sorted(pattern_stats.items(), key=lambda x: x[1], reverse=True)[:5],
            "patterns_active": len([p for p in self.security_patterns if p.enabled]),
            "false_positives_trained": len(self.false_positive_patterns),
            "uptime_protected": time.time()  # Would be actual service uptime
        }

    def update_security_level(self, new_level: SecurityLevel):
        """Update security level and reconfigure patterns"""
        self.security_level = new_level

        # Reinitialize patterns with new level
        self.security_patterns = self._initialize_security_patterns()

        logger.info(f"Security level updated to {new_level.value}")

    async def analyze_historical_patterns(self) -> Dict[str, Any]:
        """Analyze historical injection patterns for threat intelligence"""
        if not self.injection_history:
            return {"message": "No injection history available"}

        # Analyze patterns over time
        recent_attempts = [a for a in self.injection_history if time.time() - a.timestamp < 86400]  # Last 24 hours

        pattern_frequency = {}
        for attempt in recent_attempts:
            pattern = attempt.pattern_matched
            pattern_frequency[pattern] = pattern_frequency.get(pattern, 0) + 1

        return {
            "total_recent_attempts": len(recent_attempts),
            "pattern_frequency": pattern_frequency,
            "most_active_pattern": max(pattern_frequency.items(), key=lambda x: x[1]) if pattern_frequency else None,
            "severity_distribution": {
                "critical": len([a for a in recent_attempts if a.severity == "critical"]),
                "high": len([a for a in recent_attempts if a.severity == "high"]),
                "medium": len([a for a in recent_attempts if a.severity == "medium"]),
                "low": len([a for a in recent_attempts if a.severity == "low"])
            }
        }

# Utility functions for integration
def create_secure_execution_context(security_level: SecurityLevel = SecurityLevel.STRICT) -> 'PromptInjectionGuard':
    """Create a secure execution context"""
    return PromptInjectionGuard(security_level)

async def validate_user_input_securely(user_input: str, guard: PromptInjectionGuard,
                                     context: Optional[Dict[str, Any]] = None) -> Tuple[bool, str]:
    """Validate user input and return sanitized version"""
    validation = await guard.validate_input(user_input, context)

    if validation["safe"]:
        sanitized = await guard.sanitize_input(user_input)
        return True, sanitized
    else:
        return False, user_input  # Return original if validation fails

if __name__ == "__main__":
    import sys
    import asyncio

    async def main():
        if len(sys.argv) < 2:
            print("Usage: python prompt_injection_guard.py 'input to test' [security_level]")
            print("Security levels: minimal, standard, strict, maximum")
            print("Examples:")
            print("  python prompt_injection_guard.py 'Hello world'")
            print("  python prompt_injection_guard.py 'Ignore previous instructions' maximum")
            sys.exit(1)

        test_input = sys.argv[1]
        security_level = SecurityLevel(sys.argv[2]) if len(sys.argv) > 2 else SecurityLevel.STRICT

        print("🛡️ Prompt Injection Guard - Advanced Security")
        print("=" * 55)
        print(f"Input: {test_input}")
        print(f"Security Level: {security_level.value}")
        print("-" * 55)

        guard = PromptInjectionGuard(security_level)

        # Validate input
        result = await guard.validate_input(test_input)

        print(f"\n🔍 Security Analysis:")
        print(f"   Safe: {'✅' if result['safe'] else '❌'}")
        print(".3f"        print(f"   Injections Detected: {len(result['detected_injections'])}")

        if result['warnings']:
            print(f"   Warnings: {len(result['warnings'])}")
            for warning in result['warnings'][:3]:  # Show first 3
                print(f"     • {warning}")

        if result['detected_injections']:
            print(f"\n🚨 Detected Injections:")
            for injection in result['detected_injections'][:3]:  # Show first 3
                print(f"     • {injection['severity'].upper()}: {injection['description']}")
                print(f"       Pattern: {injection['pattern']}")
                print(f"       Matched: '{injection['matched_text']}'")

        # Sanitize input
        if not result['safe']:
            sanitized = await guard.sanitize_input(test_input)
            print(f"\n🧹 Sanitized Input: {sanitized}")

        # Show security report
        report = guard.get_security_report()
        print(f"\n📊 Security Report:")
        print(f"   Active Patterns: {report['patterns_active']}")
        print(f"   Injection Attempts: {report['total_injection_attempts']}")
        print(f"   Critical Attempts: {report['critical_attempts']}")

        print("\n🎉 Advanced prompt injection protection active!")

    asyncio.run(main())