// SPDX-License-Identifier: Apache-2.0

"use client";

import { useRef, useMemo } from "react";
import { useFrame } from "@react-three/fiber";
import * as THREE from "three";

interface CommitParticlesProps {
  events: Array<{
    sha: string;
    type: string;
  }>;
}

export function CommitParticles({ events }: CommitParticlesProps) {
  return (
    <group>
      {events.slice(0, 10).map((event) => (
        <ParticleStream
          key={event.sha}
          position={[
            (Math.random() - 0.5) * 4,
            Math.random() * 3,
            (Math.random() - 0.5) * 4,
          ]}
          color={event.type === "conflict" ? "#ff0000" : "#00ffff"}
          speed={0.5 + Math.random() * 0.5}
        />
      ))}
    </group>
  );
}

function ParticleStream({
  position,
  color,
  speed,
}: {
  position: [number, number, number];
  color: string;
  speed: number;
}) {
  const meshRef = useRef<THREE.InstancedMesh>(null);
  const particleCount = 50;
  const dummy = useMemo(() => new THREE.Object3D(), []);

  const particles = useMemo(() => {
    return Array.from({ length: particleCount }, () => ({
      offset: Math.random() * Math.PI * 2,
      speed: speed * (0.5 + Math.random() * 0.5),
      radius: 0.3 + Math.random() * 0.2,
    }));
  }, [speed]);

  useFrame((state) => {
    if (!meshRef.current) return;

    const time = state.clock.elapsedTime;

    particles.forEach((particle, i) => {
      const angle = time * particle.speed + particle.offset;
      const x = position[0] + Math.cos(angle) * particle.radius;
      const y =
        position[1] +
        Math.sin(time * speed * 2) * 0.2 +
        (i / particleCount) * 2;
      const z = position[2] + Math.sin(angle) * particle.radius;

      dummy.position.set(x, y, z);
      dummy.rotation.set(time * 0.5, time * 0.3, 0);
      const scale = 0.02 * (0.5 + Math.sin(time * 3 + i) * 0.5);
      dummy.scale.setScalar(scale);
      dummy.updateMatrix();

      meshRef.current!.setMatrixAt(i, dummy.matrix);
    });

    meshRef.current.instanceMatrix.needsUpdate = true;
  });

  return (
    <instancedMesh ref={meshRef} args={[undefined, undefined, particleCount]}>
      <sphereGeometry args={[1, 8, 8]} />
      <meshBasicMaterial
        color={color}
        transparent
        opacity={0.6}
        toneMapped={false}
      />
    </instancedMesh>
  );
}
