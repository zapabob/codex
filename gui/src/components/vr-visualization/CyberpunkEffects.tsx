// SPDX-License-Identifier: Apache-2.0

"use client";

import {
  EffectComposer,
  Bloom,
  ChromaticAberration,
  Noise,
  Vignette,
} from "@react-three/postprocessing";
import { BlendFunction } from "postprocessing";
import { Vector2 } from "three";

interface CyberpunkEffectsProps {
  bloomIntensity?: number;
}

export function CyberpunkEffects({
  bloomIntensity = 2.5,
}: CyberpunkEffectsProps) {
  return (
    <EffectComposer>
      <Bloom
        intensity={bloomIntensity}
        luminanceThreshold={0.6}
        luminanceSmoothing={0.9}
        kernelSize={5}
        mipmapBlur
        blendFunction={BlendFunction.ADD}
      />
      <ChromaticAberration
        offset={new Vector2(0.002, 0.002)}
        radialModulation={false}
        modulationOffset={0}
      />
      <Noise opacity={0.05} blendFunction={BlendFunction.OVERLAY} />
      <Vignette darkness={0.6} eskil={false} offset={0.1} />
    </EffectComposer>
  );
}
