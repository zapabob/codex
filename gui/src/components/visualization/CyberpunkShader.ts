/**
 * Cyberpunk Shader - Kamui4D超えサイバーパンク風シェーダー実装
 * 
 * Three.js/Babylon.js用のサイバーパンク風シェーダー
 * - Neon効果（グロー）
 * - パーティクルシステム
 * - グリッチエフェクト
 * - ポストプロセスエフェクト
 */

import * as THREE from 'three';

/**
 * Cyberpunk Shader Material for Three.js
 * 
 * サイバーパンク風のネオングロー効果を持つシェーダーマテリアル
 */
export class CyberpunkShaderMaterial extends THREE.ShaderMaterial {
  constructor(options?: {
    color?: THREE.Color | string;
    intensity?: number;
    pulseSpeed?: number;
    glowRadius?: number;
  }) {
    const color = options?.color 
      ? (typeof options.color === 'string' ? new THREE.Color(options.color) : options.color)
      : new THREE.Color(0x00ffff); // Cyan default
    
    const intensity = options?.intensity ?? 1.0;
    const pulseSpeed = options?.pulseSpeed ?? 1.0;
    const glowRadius = options?.glowRadius ?? 2.0;

    super({
      uniforms: {
        uTime: { value: 0 },
        uColor: { value: color },
        uIntensity: { value: intensity },
        uPulseSpeed: { value: pulseSpeed },
        uGlowRadius: { value: glowRadius },
        uResolution: { value: new THREE.Vector2(window.innerWidth, window.innerHeight) },
      },
      vertexShader: `
        varying vec3 vPosition;
        varying vec3 vNormal;
        varying vec2 vUv;
        
        void main() {
          vPosition = position;
          vNormal = normalize(normalMatrix * normal);
          vUv = uv;
          
          gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
        }
      `,
      fragmentShader: `
        uniform float uTime;
        uniform vec3 uColor;
        uniform float uIntensity;
        uniform float uPulseSpeed;
        uniform float uGlowRadius;
        uniform vec2 uResolution;
        
        varying vec3 vPosition;
        varying vec3 vNormal;
        varying vec2 vUv;
        
        void main() {
          // Calculate distance from center for glow effect
          vec2 center = vec2(0.5, 0.5);
          float dist = distance(vUv, center);
          
          // Pulse effect
          float pulse = sin(uTime * uPulseSpeed) * 0.5 + 0.5;
          float glow = 1.0 - smoothstep(0.0, uGlowRadius, dist);
          
          // Neon glow intensity
          float intensity = glow * pulse * uIntensity;
          
          // Color with neon effect
          vec3 neonColor = uColor * intensity;
          
          // Add scanlines effect
          float scanline = sin(vUv.y * uResolution.y * 0.7 + uTime * 2.0) * 0.02 + 0.98;
          
          // Final color
          vec3 finalColor = neonColor * scanline;
          
          gl_FragColor = vec4(finalColor, intensity * 0.8);
        }
      `,
      transparent: true,
      blending: THREE.AdditiveBlending,
    });
  }

  update(time: number) {
    this.uniforms.uTime.value = time;
  }

  setColor(color: THREE.Color | string) {
    const threeColor = typeof color === 'string' ? new THREE.Color(color) : color;
    this.uniforms.uColor.value = threeColor;
  }

  setIntensity(intensity: number) {
    this.uniforms.uIntensity.value = intensity;
  }
}

/**
 * Cyberpunk Post-Process Effect
 * 
 * ポストプロセスエフェクト（グリッチ、ブルーム、カラーパルス）
 */
export class CyberpunkPostProcess {
  private composer: any; // THREE.EffectComposer
  private renderPass: any;
  private bloomPass: any;
  private glitchPass: any;
  private colorPass: any;

  constructor(renderer: THREE.WebGLRenderer, scene: THREE.Scene, camera: THREE.Camera) {
    // Note: Requires THREE.EffectComposer and passes
    // This is a conceptual implementation
    // In production, you'd use postprocessing library or implement custom passes
    
    // Initialize post-processing passes
    // this.composer = new EffectComposer(renderer);
    // this.renderPass = new RenderPass(scene, camera);
    // this.bloomPass = new BloomPass({ ... });
    // this.glitchPass = new GlitchPass({ ... });
    // this.colorPass = new ColorCorrectionPass({ ... });
  }

  render() {
    // this.composer.render();
  }

  resize(width: number, height: number) {
    // this.composer.setSize(width, height);
  }
}

/**
 * Cyberpunk Particle System
 * 
 * サイバーパンク風のパーティクルシステム（データストリーム、マトリックスレイン風）
 */
export class CyberpunkParticleSystem {
  private particles: THREE.Points;
  private geometry: THREE.BufferGeometry;
  private material: THREE.PointsMaterial;
  private particleCount: number;

  constructor(count: number = 10000, color: THREE.Color | string = 0x00ffff) {
    this.particleCount = count;
    const threeColor = typeof color === 'string' ? new THREE.Color(color) : color;

    // Create geometry
    this.geometry = new THREE.BufferGeometry();
    const positions = new Float32Array(count * 3);
    const colors = new Float32Array(count * 3);
    const sizes = new Float32Array(count);

    // Initialize particles
    for (let i = 0; i < count; i++) {
      const i3 = i * 3;
      
      // Random positions
      positions[i3] = (Math.random() - 0.5) * 200;
      positions[i3 + 1] = Math.random() * 200;
      positions[i3 + 2] = (Math.random() - 0.5) * 200;
      
      // Color variation
      const hue = Math.random();
      const particleColor = new THREE.Color().setHSL(hue, 1.0, 0.5);
      colors[i3] = particleColor.r;
      colors[i3 + 1] = particleColor.g;
      colors[i3 + 2] = particleColor.b;
      
      // Size variation
      sizes[i] = Math.random() * 3 + 1;
    }

    this.geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3));
    this.geometry.setAttribute('color', new THREE.BufferAttribute(colors, 3));
    this.geometry.setAttribute('size', new THREE.BufferAttribute(sizes, 1));

    // Create material with neon glow
    this.material = new THREE.PointsMaterial({
      size: 2,
      vertexColors: true,
      transparent: true,
      opacity: 0.8,
      blending: THREE.AdditiveBlending,
      sizeAttenuation: true,
    });

    this.particles = new THREE.Points(this.geometry, this.material);
  }

  update(time: number) {
    const positions = this.geometry.attributes.position.array as Float32Array;
    
    for (let i = 0; i < this.particleCount; i++) {
      const i3 = i * 3;
      
      // Animate particles (matrix rain effect)
      positions[i3 + 1] -= 0.5 + Math.random() * 0.5;
      
      // Reset when out of bounds
      if (positions[i3 + 1] < 0) {
        positions[i3 + 1] = 200;
        positions[i3] = (Math.random() - 0.5) * 200;
        positions[i3 + 2] = (Math.random() - 0.5) * 200;
      }
    }
    
    this.geometry.attributes.position.needsUpdate = true;
  }

  getMesh(): THREE.Points {
    return this.particles;
  }

  dispose() {
    this.geometry.dispose();
    this.material.dispose();
  }
}

/**
 * Cyberpunk Glitch Effect
 * 
 * グリッチエフェクト（画面歪み、色ずれ）
 */
export class CyberpunkGlitchEffect {
  private shaderMaterial: THREE.ShaderMaterial;

  constructor() {
    this.shaderMaterial = new THREE.ShaderMaterial({
      uniforms: {
        uTime: { value: 0 },
        uIntensity: { value: 0.0 },
        uTexture: { value: null },
      },
      vertexShader: `
        varying vec2 vUv;
        void main() {
          vUv = uv;
          gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
        }
      `,
      fragmentShader: `
        uniform float uTime;
        uniform float uIntensity;
        uniform sampler2D uTexture;
        
        varying vec2 vUv;
        
        void main() {
          vec2 uv = vUv;
          
          // Horizontal glitch
          float glitch = sin(uTime * 10.0) * uIntensity;
          uv.x += glitch * (0.01 * sin(uTime * 50.0));
          
          // Color channel separation
          vec2 offset = vec2(glitch * 0.02, 0.0);
          float r = texture2D(uTexture, uv + offset).r;
          float g = texture2D(uTexture, uv).g;
          float b = texture2D(uTexture, uv - offset).b;
          
          vec3 color = vec3(r, g, b);
          
          // Scanline effect
          float scanline = sin(uv.y * 800.0 + uTime * 5.0) * 0.02 + 0.98;
          color *= scanline;
          
          gl_FragColor = vec4(color, 1.0);
        }
      `,
    });
  }

  update(time: number, intensity: number = 0.0) {
    this.shaderMaterial.uniforms.uTime.value = time;
    this.shaderMaterial.uniforms.uIntensity.value = intensity;
  }

  getMaterial(): THREE.ShaderMaterial {
    return this.shaderMaterial;
  }
}

/**
 * Cyberpunk Color Palette
 * 
 * サイバーパンク風カラーパレット（シアン/マゼンタ/イエロー/ブラック）
 */
export const CyberpunkColors = {
  cyan: new THREE.Color(0x00ffff),
  magenta: new THREE.Color(0xff00ff),
  yellow: new THREE.Color(0xffff00),
  black: new THREE.Color(0x000000),
  green: new THREE.Color(0x00ff41), // Matrix green
  pink: new THREE.Color(0xff0080), // Hot pink
  blue: new THREE.Color(0x00d4ff), // Electric blue
  purple: new THREE.Color(0xb84fff), // Neon purple
};

/**
 * Utility function to create cyberpunk scene
 */
export function createCyberpunkScene(renderer: THREE.WebGLRenderer): {
  scene: THREE.Scene;
  camera: THREE.PerspectiveCamera;
  particleSystem: CyberpunkParticleSystem;
  glitchEffect: CyberpunkGlitchEffect;
} {
  const scene = new THREE.Scene();
  scene.background = new THREE.Color(0x0a0a0f);
  scene.fog = new THREE.Fog(0x0a0a0f, 50, 200);

  const camera = new THREE.PerspectiveCamera(
    75,
    window.innerWidth / window.innerHeight,
    0.1,
    1000
  );
  camera.position.set(0, 50, 100);

  // Add ambient light with cyberpunk colors
  const ambientLight = new THREE.AmbientLight(0x404040, 0.3);
  scene.add(ambientLight);

  // Add colored lights
  const cyanLight = new THREE.PointLight(CyberpunkColors.cyan, 1, 100);
  cyanLight.position.set(50, 50, 50);
  scene.add(cyanLight);

  const magentaLight = new THREE.PointLight(CyberpunkColors.magenta, 1, 100);
  magentaLight.position.set(-50, 50, -50);
  scene.add(magentaLight);

  // Create particle system
  const particleSystem = new CyberpunkParticleSystem(10000, CyberpunkColors.cyan);
  scene.add(particleSystem.getMesh());

  // Create glitch effect
  const glitchEffect = new CyberpunkGlitchEffect();

  return {
    scene,
    camera,
    particleSystem,
    glitchEffect,
  };
}

/**
 * Animation loop for cyberpunk effects
 */
export function animateCyberpunkScene(
  renderer: THREE.WebGLRenderer,
  scene: THREE.Scene,
  camera: THREE.Camera,
  particleSystem: CyberpunkParticleSystem,
  glitchEffect: CyberpunkGlitchEffect,
  time: number
) {
  // Update particle system
  particleSystem.update(time);

  // Update glitch effect (random intensity)
  const glitchIntensity = Math.random() > 0.95 ? Math.random() * 0.3 : 0.0;
  glitchEffect.update(time, glitchIntensity);

  // Render
  renderer.render(scene, camera);
}

export default {
  CyberpunkShaderMaterial,
  CyberpunkPostProcess,
  CyberpunkParticleSystem,
  CyberpunkGlitchEffect,
  CyberpunkColors,
  createCyberpunkScene,
  animateCyberpunkScene,
};

