/**
 * Three.js型定義
 * Three.js関連の型定義（postprocessing等）
 */



// EffectComposer型（postprocessingライブラリ使用時）
export interface EffectComposer {
  render: (deltaTime?: number) => void;
  setSize: (width: number, height: number) => void;
  [key: string]: unknown;
}

// RenderPass型
export interface RenderPass {
  [key: string]: unknown;
}

// BloomPass型
export interface BloomPass {
  [key: string]: unknown;
}

// GlitchPass型
export interface GlitchPass {
  [key: string]: unknown;
}

// ColorPass型
export interface ColorPass {
  [key: string]: unknown;
}

// Git4Dコミットデータ型（visualization用）
export interface Git4DCommitData {
  id: string;
  author: string;
  message: string;
  timestamp: number;
  x: number;
  y: number;
  z: number;
  filesChanged: number;
  insertions: number;
  deletions: number;
  branch: number;
  parents: string[];
}
