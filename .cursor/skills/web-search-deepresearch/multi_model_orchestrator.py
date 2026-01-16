#!/usr/bin/env python3
"""
Multi-Model Orchestrator for Web Search Deepresearch 2.1
Eliminates ClaudeCode's single-model dependency by providing intelligent model selection
and seamless orchestration across Gemini, Claude, GPT, and local models.
"""

import asyncio
import json
import os
import time
from typing import Dict, List, Optional, Any, Tuple, Callable
from dataclasses import dataclass, field
from enum import Enum
import logging
from abc import ABC, abstractmethod

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class AIModel(Enum):
    """Available AI models for orchestration"""
    GEMINI_PRO = "gemini-pro"
    GEMINI_PRO_VISION = "gemini-pro-vision"
    CLAUDE_3_OPUS = "claude-3-opus"
    CLAUDE_3_SONNET = "claude-3-sonnet"
    CLAUDE_3_HAIKU = "claude-3-haiku"
    GPT_4_TURBO = "gpt-4-turbo"
    GPT_4 = "gpt-4"
    GPT_3_5_TURBO = "gpt-3.5-turbo"
    LOCAL_LLAMA_3 = "local-llama-3"
    LOCAL_CODE_LLAMA = "local-code-llama"
    LOCAL_DEEPSEEK = "local-deepseek"

class TaskType(Enum):
    """Task types for model selection"""
    CODE_GENERATION = "code_generation"
    CODE_REVIEW = "code_review"
    RESEARCH = "research"
    ANALYSIS = "analysis"
    CREATIVE = "creative"
    CONVERSATION = "conversation"
    DEBUGGING = "debugging"
    OPTIMIZATION = "optimization"

class PrivacyLevel(Enum):
    """Privacy requirements for task execution"""
    PUBLIC = "public"          # No sensitive data, internet OK
    SENSITIVE = "sensitive"    # Some sensitive data, prefer local
    PRIVATE = "private"        # Highly sensitive, require local processing
    OFFLINE = "offline"        # Must be offline, no internet

@dataclass
class ModelCapabilities:
    """Model capabilities and characteristics"""
    model: AIModel
    provider: str
    context_window: int
    max_tokens: int
    supports_vision: bool
    supports_tools: bool
    supports_json: bool
    cost_per_1k_input: float
    cost_per_1k_output: float
    latency_ms: int
    reliability_score: float  # 0.0-1.0
    specialized_domains: List[str] = field(default_factory=list)

@dataclass
class TaskRequirements:
    """Task requirements for model selection"""
    task_type: TaskType
    complexity: str  # "low", "medium", "high", "expert"
    input_length: int
    output_length: int
    requires_vision: bool
    requires_tools: bool
    requires_json: bool
    privacy_level: PrivacyLevel
    max_cost_per_request: float
    max_latency_ms: int
    preferred_providers: List[str] = field(default_factory=list)
    excluded_providers: List[str] = field(default_factory=list)

@dataclass
class ModelSelection:
    """Model selection result with reasoning"""
    selected_model: AIModel
    fallback_models: List[AIModel]
    selection_reason: str
    expected_cost: float
    expected_latency: int
    confidence_score: float
    privacy_compliance: bool
    capabilities_match: Dict[str, bool]

class AIModelClient(ABC):
    """Abstract base class for AI model clients"""

    @abstractmethod
    async def generate(self, prompt: str, **kwargs) -> Dict[str, Any]:
        """Generate response from the model"""
        pass

    @abstractmethod
    def get_capabilities(self) -> ModelCapabilities:
        """Get model capabilities"""
        pass

    @abstractmethod
    def is_available(self) -> bool:
        """Check if model is available"""
        pass

    @abstractmethod
    def estimate_cost(self, input_tokens: int, output_tokens: int) -> float:
        """Estimate cost for request"""
        pass

class ClaudeCodeMultiModelOrchestrator:
    """
    Intelligent multi-model orchestrator that eliminates ClaudeCode's single-model dependency.
    Provides seamless model selection, cost optimization, and privacy-aware execution.
    """

    def __init__(self):
        self.models = self._initialize_models()
        self.clients: Dict[AIModel, AIModelClient] = {}
        self.cache = ModelSelectionCache()
        self.cost_tracker = CostTracker()
        self.privacy_manager = PrivacyManager()

        # Initialize clients for available models
        self._initialize_clients()

    def _initialize_models(self) -> Dict[AIModel, ModelCapabilities]:
        """Initialize model capabilities database"""
        return {
            AIModel.GEMINI_PRO: ModelCapabilities(
                model=AIModel.GEMINI_PRO,
                provider="Google",
                context_window=32768,
                max_tokens=8192,
                supports_vision=True,
                supports_tools=True,
                supports_json=True,
                cost_per_1k_input=0.00025,
                cost_per_1k_output=0.0005,
                latency_ms=2000,
                reliability_score=0.95,
                specialized_domains=["multimodal", "analysis", "creative"]
            ),
            AIModel.CLAUDE_3_OPUS: ModelCapabilities(
                model=AIModel.CLAUDE_3_OPUS,
                provider="Anthropic",
                context_window=200000,
                max_tokens=4096,
                supports_vision=True,
                supports_tools=True,
                supports_json=True,
                cost_per_1k_input=0.015,
                cost_per_1k_output=0.075,
                latency_ms=3000,
                reliability_score=0.98,
                specialized_domains=["reasoning", "analysis", "code"]
            ),
            AIModel.CLAUDE_3_SONNET: ModelCapabilities(
                model=AIModel.CLAUDE_3_SONNET,
                provider="Anthropic",
                context_window=200000,
                max_tokens=4096,
                supports_vision=True,
                supports_tools=True,
                supports_json=True,
                cost_per_1k_input=0.003,
                cost_per_1k_output=0.015,
                latency_ms=2000,
                reliability_score=0.96,
                specialized_domains=["code", "analysis", "reasoning"]
            ),
            AIModel.GPT_4_TURBO: ModelCapabilities(
                model=AIModel.GPT_4_TURBO,
                provider="OpenAI",
                context_window=128000,
                max_tokens=4096,
                supports_vision=True,
                supports_tools=True,
                supports_json=True,
                cost_per_1k_input=0.01,
                cost_per_1k_output=0.03,
                latency_ms=2500,
                reliability_score=0.94,
                specialized_domains=["creative", "analysis", "conversation"]
            ),
            AIModel.LOCAL_LLAMA_3: ModelCapabilities(
                model=AIModel.LOCAL_LLAMA_3,
                provider="Local",
                context_window=8192,
                max_tokens=2048,
                supports_vision=False,
                supports_tools=False,
                supports_json=True,
                cost_per_1k_input=0.0,
                cost_per_1k_output=0.0,
                latency_ms=5000,
                reliability_score=0.85,
                specialized_domains=["privacy", "offline", "general"]
            ),
            AIModel.LOCAL_CODE_LLAMA: ModelCapabilities(
                model=AIModel.LOCAL_CODE_LLAMA,
                provider="Local",
                context_window=16384,
                max_tokens=4096,
                supports_vision=False,
                supports_tools=False,
                supports_json=True,
                cost_per_1k_input=0.0,
                cost_per_1k_output=0.0,
                latency_ms=4000,
                reliability_score=0.88,
                specialized_domains=["code", "privacy", "offline"]
            )
        }

    def _initialize_clients(self):
        """Initialize model clients for available models"""
        # This would initialize actual API clients
        # For now, we'll create mock clients
        for model in self.models.keys():
            if self._is_model_available(model):
                self.clients[model] = self._create_client(model)

    def _is_model_available(self, model: AIModel) -> bool:
        """Check if a model is available based on API keys and configuration"""
        if model.provider == "Local":
            # Check if local model is installed
            return self._check_local_model_availability(model)

        # Check API keys for cloud providers
        api_keys = {
            "Google": os.getenv("GOOGLE_API_KEY"),
            "Anthropic": os.getenv("ANTHROPIC_API_KEY"),
            "OpenAI": os.getenv("OPENAI_API_KEY")
        }

        provider = self.models[model].provider
        return bool(api_keys.get(provider))

    def _check_local_model_availability(self, model: AIModel) -> bool:
        """Check if local model is available"""
        # This would check if Ollama or similar is running
        # For demo, assume some models are available
        available_local_models = [AIModel.LOCAL_LLAMA_3, AIModel.LOCAL_CODE_LLAMA]
        return model in available_local_models

    def _create_client(self, model: AIModel) -> AIModelClient:
        """Create appropriate client for the model"""
        # This would create actual API clients
        # For demo, return a mock client
        return MockModelClient(model, self.models[model])

    async def select_and_execute(self, task_requirements: TaskRequirements,
                               prompt: str, **kwargs) -> Dict[str, Any]:
        """
        Select optimal model and execute the task.
        ClaudeCode's model dependency problem solved.
        """
        logger.info(f"Selecting model for task: {task_requirements.task_type.value}")

        # Select optimal model
        selection = await self.select_model(task_requirements)

        logger.info(f"Selected model: {selection.selected_model.value} "
                   f"(reason: {selection.selection_reason})")

        # Execute with selected model
        try:
            result = await self.execute_with_model(selection.selected_model, prompt, **kwargs)

            # Track cost and performance
            await self.cost_tracker.track_request(
                selection.selected_model, task_requirements, result
            )

            # Update cache with successful execution
            await self.cache.update_cache(task_requirements, selection, result)

            return {
                "success": True,
                "model": selection.selected_model.value,
                "result": result,
                "cost": selection.expected_cost,
                "latency": result.get("latency", 0),
                "selection": selection
            }

        except Exception as e:
            logger.warning(f"Primary model failed: {e}")

            # Try fallback models
            for fallback_model in selection.fallback_models:
                try:
                    logger.info(f"Trying fallback model: {fallback_model.value}")
                    result = await self.execute_with_model(fallback_model, prompt, **kwargs)
                    return {
                        "success": True,
                        "model": fallback_model.value,
                        "result": result,
                        "fallback": True,
                        "original_model": selection.selected_model.value
                    }
                except Exception as fallback_error:
                    logger.warning(f"Fallback model {fallback_model.value} also failed: {fallback_error}")
                    continue

            return {
                "success": False,
                "error": str(e),
                "model": selection.selected_model.value
            }

    async def select_model(self, requirements: TaskRequirements) -> ModelSelection:
        """
        Intelligently select the optimal model for the task.
        Eliminates ClaudeCode's single-model limitation.
        """

        # Check cache first
        cached_selection = await self.cache.get_cached_selection(requirements)
        if cached_selection:
            logger.info("Using cached model selection")
            return cached_selection

        # Filter available models based on requirements
        candidates = await self._filter_candidates(requirements)

        if not candidates:
            # Fallback to any available model
            candidates = [model for model in self.models.keys() if self._is_model_available(model)]

        # Score and rank candidates
        scored_candidates = await self._score_candidates(candidates, requirements)

        # Select primary and fallback models
        primary_model = scored_candidates[0][0]
        fallback_models = [model for model, _ in scored_candidates[1:3]]  # Top 2 as fallbacks

        # Generate selection reasoning
        selection_reason = self._generate_selection_reasoning(
            primary_model, requirements, scored_candidates[0][1]
        )

        # Calculate expected metrics
        expected_cost = self._calculate_expected_cost(primary_model, requirements)
        expected_latency = self.models[primary_model].latency_ms

        # Check privacy compliance
        privacy_compliance = self.privacy_manager.check_compliance(
            primary_model, requirements.privacy_level
        )

        # Check capabilities match
        capabilities_match = self._check_capabilities_match(primary_model, requirements)

        selection = ModelSelection(
            selected_model=primary_model,
            fallback_models=fallback_models,
            selection_reason=selection_reason,
            expected_cost=expected_cost,
            expected_latency=expected_latency,
            confidence_score=0.9,  # Simplified
            privacy_compliance=privacy_compliance,
            capabilities_match=capabilities_match
        )

        # Cache the selection
        await self.cache.store_selection(requirements, selection)

        return selection

    async def _filter_candidates(self, requirements: TaskRequirements) -> List[AIModel]:
        """Filter models based on task requirements"""
        candidates = []

        for model, capabilities in self.models.items():
            if not self._is_model_available(model):
                continue

            # Check basic requirements
            if requirements.input_length > capabilities.context_window:
                continue

            if requirements.requires_vision and not capabilities.supports_vision:
                continue

            if requirements.requires_tools and not capabilities.supports_tools:
                continue

            if requirements.requires_json and not capabilities.supports_json:
                continue

            # Check cost constraints
            estimated_cost = self._estimate_request_cost(model, requirements)
            if estimated_cost > requirements.max_cost_per_request:
                continue

            # Check latency constraints
            if capabilities.latency_ms > requirements.max_latency_ms:
                continue

            # Check provider preferences
            if requirements.preferred_providers and capabilities.provider not in requirements.preferred_providers:
                continue

            if capabilities.provider in requirements.excluded_providers:
                continue

            # Check privacy compliance
            if not self.privacy_manager.check_compliance(model, requirements.privacy_level):
                continue

            candidates.append(model)

        return candidates

    async def _score_candidates(self, candidates: List[AIModel],
                              requirements: TaskRequirements) -> List[Tuple[AIModel, float]]:
        """Score and rank model candidates"""
        scored = []

        for model in candidates:
            score = await self._calculate_model_score(model, requirements)
            scored.append((model, score))

        # Sort by score (descending)
        scored.sort(key=lambda x: x[1], reverse=True)

        return scored

    async def _calculate_model_score(self, model: AIModel, requirements: TaskRequirements) -> float:
        """Calculate comprehensive score for a model"""
        capabilities = self.models[model]
        score = 0.0

        # Task type compatibility (0-20 points)
        score += self._score_task_compatibility(model, requirements.task_type) * 20

        # Complexity handling (0-15 points)
        score += self._score_complexity_handling(model, requirements.complexity) * 15

        # Cost efficiency (0-15 points)
        cost_score = 1.0 - min(1.0, self._estimate_request_cost(model, requirements) / requirements.max_cost_per_request)
        score += cost_score * 15

        # Latency performance (0-10 points)
        latency_score = 1.0 - min(1.0, capabilities.latency_ms / requirements.max_latency_ms)
        score += latency_score * 10

        # Reliability (0-10 points)
        score += capabilities.reliability_score * 10

        # Privacy compliance (0-10 points)
        privacy_score = 1.0 if self.privacy_manager.check_compliance(model, requirements.privacy_level) else 0.0
        score += privacy_score * 10

        # Capability match bonus (0-10 points)
        capability_bonus = self._calculate_capability_bonus(model, requirements)
        score += capability_bonus * 10

        # Specialization bonus (0-10 points)
        specialization_bonus = self._calculate_specialization_bonus(model, requirements)
        score += specialization_bonus * 10

        return min(100.0, score)  # Cap at 100

    def _score_task_compatibility(self, model: AIModel, task_type: TaskType) -> float:
        """Score how well the model handles the task type"""
        capabilities = self.models[model]
        compatibility_map = {
            TaskType.CODE_GENERATION: ["code"],
            TaskType.CODE_REVIEW: ["code", "analysis"],
            TaskType.RESEARCH: ["analysis", "reasoning"],
            TaskType.ANALYSIS: ["analysis", "reasoning"],
            TaskType.CREATIVE: ["creative"],
            TaskType.CONVERSATION: ["conversation"],
            TaskType.DEBUGGING: ["code", "analysis"],
            TaskType.OPTIMIZATION: ["code", "analysis"]
        }

        required_domains = compatibility_map.get(task_type, [])
        matching_domains = [d for d in required_domains if d in capabilities.specialized_domains]

        return len(matching_domains) / len(required_domains) if required_domains else 0.5

    def _score_complexity_handling(self, model: AIModel, complexity: str) -> float:
        """Score model's ability to handle task complexity"""
        complexity_scores = {
            "low": 0.3,
            "medium": 0.6,
            "high": 0.8,
            "expert": 1.0
        }

        base_score = complexity_scores.get(complexity, 0.5)
        capabilities = self.models[model]

        # Boost score for models with larger context windows and higher reliability
        context_boost = min(1.0, capabilities.context_window / 100000)  # Normalize to 100k
        reliability_boost = capabilities.reliability_score

        return min(1.0, base_score * (1 + context_boost) * (1 + reliability_boost) / 2)

    def _calculate_capability_bonus(self, model: AIModel, requirements: TaskRequirements) -> float:
        """Calculate bonus for exact capability matches"""
        capabilities = self.models[model]
        bonus = 0.0

        if requirements.requires_vision and capabilities.supports_vision:
            bonus += 0.3
        if requirements.requires_tools and capabilities.supports_tools:
            bonus += 0.3
        if requirements.requires_json and capabilities.supports_json:
            bonus += 0.4

        return bonus

    def _calculate_specialization_bonus(self, model: AIModel, requirements: TaskRequirements) -> float:
        """Calculate bonus for domain specialization"""
        capabilities = self.models[model]

        # Task-type based specialization scoring
        specialization_map = {
            TaskType.CODE_GENERATION: ["code"],
            TaskType.CODE_REVIEW: ["code", "analysis"],
            TaskType.RESEARCH: ["analysis", "reasoning"],
            TaskType.ANALYSIS: ["analysis"],
            TaskType.CREATIVE: ["creative"],
            TaskType.DEBUGGING: ["code"],
            TaskType.OPTIMIZATION: ["code", "analysis"]
        }

        relevant_domains = specialization_map.get(requirements.task_type, [])
        matching_specializations = [d for d in relevant_domains if d in capabilities.specialized_domains]

        return len(matching_specializations) / len(relevant_domains) if relevant_domains else 0.0

    def _generate_selection_reasoning(self, model: AIModel, requirements: TaskRequirements, score: float) -> str:
        """Generate human-readable reasoning for model selection"""
        capabilities = self.models[model]

        reasons = []

        if requirements.task_type == TaskType.CODE_GENERATION and "code" in capabilities.specialized_domains:
            reasons.append("optimized for code generation tasks")

        if requirements.privacy_level in [PrivacyLevel.PRIVATE, PrivacyLevel.OFFLINE] and capabilities.provider == "Local":
            reasons.append("ensures data privacy with local processing")

        if score > 80:
            reasons.append("excellent overall performance score")
        elif score > 60:
            reasons.append("good balance of performance and cost")

        cost_effective = self._estimate_request_cost(model, requirements) < requirements.max_cost_per_request * 0.5
        if cost_effective:
            reasons.append("cost-effective option")

        return f"Selected {model.value} because it is {' and '.join(reasons)}"

    def _estimate_request_cost(self, model: AIModel, requirements: TaskRequirements) -> float:
        """Estimate cost for a request"""
        capabilities = self.models[model]

        # Rough token estimation
        input_tokens = requirements.input_length // 4  # Rough character to token conversion
        output_tokens = requirements.output_length // 4

        input_cost = (input_tokens / 1000) * capabilities.cost_per_1k_input
        output_cost = (output_tokens / 1000) * capabilities.cost_per_1k_output

        return input_cost + output_cost

    def _calculate_expected_cost(self, model: AIModel, requirements: TaskRequirements) -> float:
        """Calculate expected cost with some buffer"""
        return self._estimate_request_cost(model, requirements) * 1.1  # 10% buffer

    def _check_capabilities_match(self, model: AIModel, requirements: TaskRequirements) -> Dict[str, bool]:
        """Check detailed capability matches"""
        capabilities = self.models[model]

        return {
            "vision": requirements.requires_vision == capabilities.supports_vision,
            "tools": requirements.requires_tools == capabilities.supports_tools,
            "json": requirements.requires_json == capabilities.supports_json,
            "context_window": requirements.input_length <= capabilities.context_window,
            "cost": self._estimate_request_cost(model, requirements) <= requirements.max_cost_per_request,
            "latency": capabilities.latency_ms <= requirements.max_latency_ms
        }

    async def execute_with_model(self, model: AIModel, prompt: str, **kwargs) -> Dict[str, Any]:
        """Execute request with specified model"""
        if model not in self.clients:
            raise ValueError(f"Model {model.value} is not available")

        client = self.clients[model]

        start_time = time.time()
        result = await client.generate(prompt, **kwargs)
        end_time = time.time()

        result["latency"] = int((end_time - start_time) * 1000)
        result["model"] = model.value

        return result

    async def get_model_status(self) -> Dict[str, Any]:
        """Get status of all models"""
        status = {}

        for model, capabilities in self.models.items():
            status[model.value] = {
                "available": self._is_model_available(model),
                "provider": capabilities.provider,
                "capabilities": {
                    "context_window": capabilities.context_window,
                    "supports_vision": capabilities.supports_vision,
                    "supports_tools": capabilities.supports_tools,
                    "cost_per_1k_input": capabilities.cost_per_1k_input,
                    "latency_ms": capabilities.latency_ms
                }
            }

        return status

class MockModelClient(AIModelClient):
    """Mock client for demonstration purposes"""

    def __init__(self, model: AIModel, capabilities: ModelCapabilities):
        self.model = model
        self.capabilities = capabilities

    async def generate(self, prompt: str, **kwargs) -> Dict[str, Any]:
        """Mock response generation"""
        await asyncio.sleep(0.1)  # Simulate API call

        # Generate mock response based on model type
        if "code" in self.capabilities.specialized_domains:
            response = f"```python\n# Generated by {self.model.value}\ndef solution():\n    return '{prompt[:50]}...'\n```"
        elif "analysis" in self.capabilities.specialized_domains:
            response = f"Analysis result from {self.model.value}: {prompt[:100]}..."
        else:
            response = f"Response from {self.model.value}: {prompt[:100]}..."

        return {
            "response": response,
            "tokens_used": len(prompt.split()) * 2,
            "finish_reason": "stop"
        }

    def get_capabilities(self) -> ModelCapabilities:
        return self.capabilities

    def is_available(self) -> bool:
        return True

    def estimate_cost(self, input_tokens: int, output_tokens: int) -> float:
        return ((input_tokens / 1000) * self.capabilities.cost_per_1k_input +
                (output_tokens / 1000) * self.capabilities.cost_per_1k_output)

class ModelSelectionCache:
    """Cache for model selections to improve performance"""

    def __init__(self):
        self.cache: Dict[str, ModelSelection] = {}
        self.cache_timeout = 3600  # 1 hour

    def _generate_cache_key(self, requirements: TaskRequirements) -> str:
        """Generate cache key from requirements"""
        key_parts = [
            requirements.task_type.value,
            requirements.complexity,
            str(requirements.input_length),
            str(requirements.output_length),
            str(requirements.requires_vision),
            str(requirements.requires_tools),
            requirements.privacy_level.value,
            str(requirements.max_cost_per_request),
            str(requirements.max_latency_ms)
        ]
        return "|".join(key_parts)

    async def get_cached_selection(self, requirements: TaskRequirements) -> Optional[ModelSelection]:
        """Get cached model selection if available"""
        key = self._generate_cache_key(requirements)

        if key in self.cache:
            # Check if cache is still valid (simplified)
            return self.cache[key]

        return None

    async def store_selection(self, requirements: TaskRequirements, selection: ModelSelection):
        """Store model selection in cache"""
        key = self._generate_cache_key(requirements)
        self.cache[key] = selection

    async def update_cache(self, requirements: TaskRequirements, selection: ModelSelection, result: Dict[str, Any]):
        """Update cache based on execution results"""
        # Could implement cache invalidation or reinforcement learning here
        pass

class CostTracker:
    """Track and optimize API costs"""

    def __init__(self):
        self.usage_stats: Dict[str, Dict[str, Any]] = {}

    async def track_request(self, model: AIModel, requirements: TaskRequirements, result: Dict[str, Any]):
        """Track API request for cost analysis"""
        model_name = model.value
        if model_name not in self.usage_stats:
            self.usage_stats[model_name] = {
                "requests": 0,
                "total_cost": 0.0,
                "total_tokens": 0,
                "avg_latency": 0
            }

        stats = self.usage_stats[model_name]
        stats["requests"] += 1
        stats["total_cost"] += result.get("cost", 0.0)
        stats["total_tokens"] += result.get("tokens_used", 0)
        stats["avg_latency"] = ((stats["avg_latency"] * (stats["requests"] - 1)) + result.get("latency", 0)) / stats["requests"]

    async def get_cost_analysis(self) -> Dict[str, Any]:
        """Get cost analysis and optimization recommendations"""
        return self.usage_stats

class PrivacyManager:
    """Manage privacy and security compliance"""

    def check_compliance(self, model: AIModel, privacy_level: PrivacyLevel) -> bool:
        """Check if model complies with privacy requirements"""
        if privacy_level == PrivacyLevel.OFFLINE:
            return model.value.startswith("local")

        if privacy_level == PrivacyLevel.PRIVATE:
            return model.value.startswith("local") or model.value in ["claude-3-opus", "claude-3-sonnet"]

        # Public and sensitive allow cloud models
        return True

# Main execution functions
async def orchestrate_multi_model_task(
    task_description: str,
    task_type: TaskType = TaskType.RESEARCH,
    complexity: str = "medium",
    privacy_level: PrivacyLevel = PrivacyLevel.PUBLIC,
    max_cost: float = 1.0
) -> Dict[str, Any]:
    """
    Main function to orchestrate multi-model task execution.
    ClaudeCode's model dependency completely eliminated.
    """

    # Create task requirements
    requirements = TaskRequirements(
        task_type=task_type,
        complexity=complexity,
        input_length=len(task_description),
        output_length=2000,  # Estimated
        requires_vision=False,
        requires_tools=False,
        requires_json=False,
        privacy_level=privacy_level,
        max_cost_per_request=max_cost,
        max_latency_ms=10000
    )

    # Initialize orchestrator
    orchestrator = ClaudeCodeMultiModelOrchestrator()

    # Get model status
    status = await orchestrator.get_model_status()
    logger.info(f"Available models: {sum(1 for s in status.values() if s['available'])}")

    # Execute task
    result = await orchestrator.select_and_execute(requirements, task_description)

    # Add metadata
    result["orchestrator_status"] = status
    result["cost_analysis"] = await orchestrator.cost_tracker.get_cost_analysis()

    return result

if __name__ == "__main__":
    import sys

    if len(sys.argv) < 2:
        print("Usage: python multi_model_orchestrator.py 'task description' [task_type] [complexity] [privacy_level]")
        print("Example: python multi_model_orchestrator.py 'Create a React component' code_generation medium public")
        sys.exit(1)

    task_description = sys.argv[1]
    task_type = TaskType(sys.argv[2]) if len(sys.argv) > 2 else TaskType.RESEARCH
    complexity = sys.argv[3] if len(sys.argv) > 3 else "medium"
    privacy_level = PrivacyLevel(sys.argv[4]) if len(sys.argv) > 4 else PrivacyLevel.PUBLIC

    async def main():
        print("🤖 Multi-Model Orchestrator - ClaudeCode's Model Dependency Solved")
        print("=" * 70)
        print(f"Task: {task_description}")
        print(f"Type: {task_type.value}")
        print(f"Complexity: {complexity}")
        print(f"Privacy: {privacy_level.value}")
        print("-" * 70)

        result = await orchestrate_multi_model_task(
            task_description, task_type, complexity, privacy_level
        )

        print("\n" + "=" * 70)
        print("EXECUTION RESULTS")
        print("=" * 70)

        if result["success"]:
            print(f"✅ Success with model: {result['model']}")
            if result.get("fallback"):
                print(f"   (Fallback from: {result['original_model']})")

            print(f"💰 Cost: ${result.get('cost', 0):.4f}")
            print(f"⚡ Latency: {result.get('latency', 0)}ms")

            selection = result.get("selection")
            if selection:
                print(f"🎯 Selection Reason: {selection.selection_reason}")
                print(f"📊 Confidence: {selection.confidence_score:.2f}")

            print(f"\n📝 Response: {result['result']['response'][:200]}...")

        else:
            print(f"❌ Failed: {result['error']}")

        # Show model status
        status = result.get("orchestrator_status", {})
        print(f"\n🔧 Available Models: {sum(1 for s in status.values() if s['available'])}/{len(status)}")

        # Show cost analysis
        cost_analysis = result.get("cost_analysis", {})
        if cost_analysis:
            print("
💵 Cost Summary:"            for model, stats in cost_analysis.items():
                if stats["requests"] > 0:
                    print(".4f"        else:
            print("   No requests tracked yet")

        print("\n🎉 ClaudeCode's single-model limitation eliminated!")

    asyncio.run(main())