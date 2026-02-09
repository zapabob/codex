// SPDX-License-Identifier: Apache-2.0

"use client";

import { useRef } from "react";
import { useFrame } from "@react-three/fiber";
import { Text, Line, Sphere } from "@react-three/drei";
import * as THREE from "three";

interface ConflictAlertsProps {
  events: Array<{
    sha: string;
    type: string;
  }>;
}

export function ConflictAlerts({ events }: ConflictAlertsProps) {
  const conflicts = events.filter((e) => e.type === "conflict");

  return (
    <group>
      {conflicts.map((conflict, index) => (
        <ConflictPulse
          key={conflict.sha}
          position={[-2, 1.5 + index * 0.5, -2]}
          sha={conflict.sha}
        />
      ))}
    </group>
  );
}

function ConflictPulse({
  position,
  sha,
}: {
  position: [number, number, number];
  sha: string;
}) {
  const meshRef = useRef<THREE.Mesh>(null);
  const innerRef = useRef<THREE.Mesh>(null);

  useFrame((state) => {
    if (meshRef.current) {
      const pulse = Math.sin(state.clock.elapsedTime * 8) * 0.5 + 1;
      const scale = 0.15 + pulse * 0.1;
      meshRef.current.scale.setScalar(scale);
      (meshRef.current.material as THREE.MeshBasicMaterial).opacity =
        0.2 + pulse * 0.2;
    }

    if (innerRef.current) {
      const fastPulse = Math.sin(state.clock.elapsedTime * 16) * 0.5 + 0.5;
      innerRef.current.scale.setScalar(0.5 + fastPulse * 0.3);
    }
  });

  return (
    <group position={position}>
      <Sphere ref={meshRef} args={[0.2, 32, 32]}>
        <meshBasicMaterial
          color="#ff0000"
          transparent
          opacity={0.3}
          side={THREE.BackSide}
          blending={THREE.AdditiveBlending}
        />
      </Sphere>

      <Sphere ref={innerRef} args={[0.1, 16, 16]}>
        <meshBasicMaterial color="#ff0000" />
      </Sphere>

      <Text
        position={[0.4, 0, 0]}
        fontSize={0.08}
        color="#ff0000"
        anchorX="left"
        anchorY="middle"
      >
        CONFLICT
      </Text>

      <Text
        position={[0.4, -0.12, 0]}
        fontSize={0.04}
        color="#ff6666"
        anchorX="left"
        anchorY="top"
      >
        {sha.slice(0, 8)}
      </Text>

      <Line
        points={[
          [-0.2, 0, 0],
          [2, -0.5, 2],
        ]}
        color="#ff0000"
        transparent
        opacity={0.3}
        lineWidth={1}
      />
    </group>
  );
}
