/**
 * WebXR型定義
 * WebXR APIとエラーの型定義
 */

// WebXRエラー型
export interface XRError extends Error {
  message: string;
  code?: number;
  name?: string;
}

// WebXRエラーのユーティリティ型（Errorオブジェクトまたは文字列）
export type XRErrorLike = XRError | Error | string | { message?: string; [key: string]: unknown };
