// SPDX-License-Identifier: Apache-2.0

"use client";

import { useRef } from "react";
import { useFrame } from "@react-three/fiber";
import { Text, Line } from "@react-three/drei";
import * as THREE from "three";
import type { CommitData } from "@/lib/github";

interface GitTimelineProps {
  events: Array<{
    sha: string;
    type: string;
    author: string;
    timestamp: string;
  }>;
  onCommitSelect?: (sha: string) => void;
  selectedCommit?: string | null;
}

export function GitTimeline({
  events,
  onCommitSelect,
  selectedCommit,
}: GitTimelineProps) {
  const groupRef = useRef<THREE.Group>(null);

  useFrame((state) => {
    if (groupRef.current) {
      groupRef.current.rotation.y =
        Math.sin(state.clock.elapsedTime * 0.05) * 0.05;
    }
  });

  const sortedEvents = [...events].sort((a, b) => {
    const timeA = new Date(a.timestamp).getTime();
    const timeB = new Date(b.timestamp).getTime();
    return timeB - timeA;
  });

  return (
    <group ref={groupRef} position={[0, 1, -3]}>
      <Line
        points={[
          [0, 3, 0],
          [0, -2, 0],
        ]}
        color="#333366"
        transparent
        opacity={0.5}
        lineWidth={2}
      />

      {sortedEvents.map((event, index) => {
        const isConflict = event.type === "conflict";
        const isNew = index < 3;
        const timeOffset = index * 0.3;
        const helixRadius = 2;
        const height = 2 - timeOffset;

        const position: [number, number, number] = [
          Math.sin(timeOffset * 0.5) * helixRadius,
          height,
          Math.cos(timeOffset * 0.5) * helixRadius,
        ];

        const commitData: CommitData = {
          sha: event.sha,
          message: event.type.toUpperCase(),
          author: {
            name: event.author,
            email: "",
            date: event.timestamp,
          },
          url: "",
        };

        return (
          <CommitNode
            key={event.sha}
            position={position}
            commit={commitData}
            isSelected={selectedCommit === event.sha}
            isConflict={isConflict}
            isNew={isNew}
            onClick={() => onCommitSelect?.(event.sha)}
          />
        );
      })}

      <Text
        position={[0, 3.5, 0]}
        fontSize={0.1}
        color="#00ffff"
        anchorX="center"
        anchorY="bottom"
      >
        Git Timeline
      </Text>
    </group>
  );
}

function CommitNode({
  position,
  commit,
  isSelected = false,
  isConflict = false,
  isNew = false,
  onClick,
}: {
  position: [number, number, number];
  commit: CommitData;
  isSelected?: boolean;
  isConflict?: boolean;
  isNew?: boolean;
  onClick?: () => void;
}) {
  const meshRef = useRef<THREE.Mesh>(null);
  const glowRef = useRef<THREE.Mesh>(null);
  const textRef = useRef<THREE.Group>(null);

  const sha = commit.sha;
  const message = commit.message;
  const author = commit.author.name;

  const baseColor = isConflict ? "#ff0000" : isNew ? "#00ff00" : "#00ffff";
  const emissiveColor = isConflict ? "#ff0000" : "#00ffff";

  useFrame((state) => {
    if (meshRef.current && isConflict) {
      const pulse = Math.sin(state.clock.elapsedTime * 8) * 0.5 + 1;
      (
        meshRef.current.material as THREE.MeshStandardMaterial
      ).emissiveIntensity = 2 + pulse;
    }

    if (meshRef.current && isNew) {
      const scale = 1 + Math.sin(state.clock.elapsedTime * 4) * 0.1;
      meshRef.current.scale.setScalar(scale);
    }

    if (textRef.current) {
      textRef.current.lookAt(state.camera.position);
    }
  });

  return (
    <group position={position}>
      <mesh ref={meshRef} onClick={onClick}>
        <sphereGeometry args={[0.08, 32, 32]} />
        <meshStandardMaterial
          color={isSelected ? "#ffffff" : "#1a1a2e"}
          emissive={emissiveColor}
          emissiveIntensity={isConflict ? 4 : isSelected ? 3 : 2}
          toneMapped={false}
          roughness={0.3}
          metalness={0.8}
        />
      </mesh>

      <mesh ref={glowRef} scale={1.5}>
        <sphereGeometry args={[0.08, 16, 16]} />
        <meshBasicMaterial
          color={baseColor}
          transparent
          opacity={0.15}
          side={THREE.BackSide}
          blending={THREE.AdditiveBlending}
          depthWrite={false}
        />
      </mesh>

      {isSelected && (
        <mesh rotation={[Math.PI / 2, 0, 0]}>
          <ringGeometry args={[0.12, 0.15, 32]} />
          <meshBasicMaterial color="#ffffff" transparent opacity={0.8} />
        </mesh>
      )}

      <group ref={textRef} position={[0.2, 0.1, 0]}>
        <Text
          fontSize={0.05}
          color={baseColor}
          anchorX="left"
          anchorY="bottom"
          maxWidth={0.5}
        >
          {message.length > 30 ? message.slice(0, 30) + "..." : message}
        </Text>
        <Text
          position={[0, -0.06, 0]}
          fontSize={0.03}
          color="#888888"
          anchorX="left"
          anchorY="top"
        >
          {sha.slice(0, 7)} • {author}
        </Text>
      </group>

      <Line
        points={[
          [0, 0, 0],
          [0, 0.3, 0],
        ]}
        color={baseColor}
        transparent
        opacity={0.5}
        lineWidth={1}
      />
    </group>
  );
}
