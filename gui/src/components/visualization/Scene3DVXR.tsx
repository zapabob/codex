'use client'

import { useRef, useMemo, useState } from 'react'
import { Canvas, useFrame } from '@react-three/fiber'
import { VRButton, XR, Controllers, Hands, useXR, useController } from '@react-three/xr'
import { OrbitControls, PerspectiveCamera } from '@react-three/drei'
import * as THREE from 'three'

interface Commit3D {
  sha: string
  message: string
  author: string
  author_email: string
  timestamp: string
  branch: string
  parents: string[]
  x: number
  y: number
  z: number
  color: string
}

interface Scene3DVXRProps {
  commits: Commit3D[]
  onCommitClick?: (commit: Commit3D) => void
  selectedCommitSha?: string
}

/**
 * VR-enabled Commit Nodes with Hand Tracking
 */
function CommitNodesVR({
  commits,
  onCommitClick,
  selectedCommitSha,
}: Scene3DVXRProps) {
  const meshRef = useRef<THREE.InstancedMesh>(null)
  const [hoveredIndex, setHoveredIndex] = useState<number | null>(null)
  const { isPresenting, player } = useXR()
  
  // Left and right controllers
  const leftController = useController('left')
  const rightController = useController('right')

  // Prepare instanced mesh data
  const { matrices, colors } = useMemo(() => {
    const matrices: THREE.Matrix4[] = []
    const colors: number[] = []

    commits.forEach((commit) => {
      const matrix = new THREE.Matrix4()
      const position = new THREE.Vector3(commit.x, commit.y / 1000000, commit.z)
      const scale = isPresenting ? 0.5 : 1.0  // Smaller in VR for better visibility

      matrix.compose(
        position,
        new THREE.Quaternion(),
        new THREE.Vector3(scale, scale, scale)
      )

      matrices.push(matrix)

      // Parse color
      const color = new THREE.Color(commit.color)
      colors.push(color.r, color.g, color.b)
    })

    return { matrices, colors }
  }, [commits, isPresenting])

  // Apply matrices and colors to instanced mesh
  useMemo(() => {
    if (meshRef.current) {
      matrices.forEach((matrix, i) => {
        meshRef.current!.setMatrixAt(i, matrix)
      })

      const colorAttribute = new THREE.InstancedBufferAttribute(
        new Float32Array(colors),
        3
      )
      meshRef.current.geometry.setAttribute('color', colorAttribute)

      meshRef.current.instanceMatrix.needsUpdate = true
    }
  }, [matrices, colors])

  // VR Controller raycasting
  useFrame(() => {
    if (!meshRef.current || !isPresenting) return

    // Handle left controller
    if (leftController?.controller) {
      const tempMatrix = new THREE.Matrix4()
      tempMatrix.identity().extractRotation(leftController.controller.matrixWorld)

      const raycaster = new THREE.Raycaster()
      raycaster.setFromCamera(new THREE.Vector2(0, 0), player.children[0] as THREE.Camera)

      const intersects = raycaster.intersectObject(meshRef.current)
      if (intersects.length > 0 && intersects[0].instanceId !== undefined) {
        setHoveredIndex(intersects[0].instanceId)
        
        // Trigger haptic feedback
        if (leftController.inputSource?.gamepad?.hapticActuators?.[0]) {
          leftController.inputSource.gamepad.hapticActuators[0].pulse(0.5, 100)
        }
      }
    }

    // Handle right controller
    if (rightController?.controller) {
      const tempMatrix = new THREE.Matrix4()
      tempMatrix.identity().extractRotation(rightController.controller.matrixWorld)

      const raycaster = new THREE.Raycaster()
      raycaster.setFromCamera(new THREE.Vector2(0, 0), player.children[0] as THREE.Camera)

      const intersects = raycaster.intersectObject(meshRef.current)
      if (intersects.length > 0 && intersects[0].instanceId !== undefined) {
        const index = intersects[0].instanceId
        
        // Check for trigger press
        if (rightController.inputSource?.gamepad?.buttons[0]?.pressed) {
          onCommitClick?.(commits[index])
          
          // Stronger haptic feedback on click
          if (rightController.inputSource.gamepad.hapticActuators?.[0]) {
            rightController.inputSource.gamepad.hapticActuators[0].pulse(1.0, 200)
          }
        }
      }
    }
  })

  return (
    <instancedMesh
      ref={meshRef}
      args={[undefined, undefined, commits.length]}
      frustumCulled
    >
      <sphereGeometry args={[1, 16, 16]} />
      <meshStandardMaterial vertexColors />
    </instancedMesh>
  )
}

/**
 * VR Interface - In-world UI panels
 */
function VRScene({ commits, onCommitClick, selectedCommitSha }: Scene3DVXRProps) {
  return (
    <>
      {/* Lighting */}
      <ambientLight intensity={0.4} />
      <pointLight position={[10, 10, 10]} intensity={1} />
      <pointLight position={[-10, -10, -10]} intensity={0.5} color="#764ba2" />

      {/* Commit nodes */}
      <CommitNodesVR
        commits={commits}
        onCommitClick={onCommitClick}
        selectedCommitSha={selectedCommitSha}
      />

      {/* Grid helper */}
      <gridHelper args={[100, 100, '#ffffff', '#333333']} />

      {/* Axes helper */}
      <axesHelper args={[50]} />
    </>
  )
}

/**
 * Main VR Scene Component
 */
export default function Scene3DVXR({
  commits,
  onCommitClick,
  selectedCommitSha,
}: Scene3DVXRProps) {
  return (
    <div className="w-full h-full">
      <Canvas>
        <XR>
          <VRButton />
          <Controllers />
          <Hands />
          
          <PerspectiveCamera makeDefault position={[0, 1.6, 5]} />
          <OrbitControls
            enableDamping
            dampingFactor={0.05}
            minDistance={10}
            maxDistance={200}
          />

          <VRScene
            commits={commits}
            onCommitClick={onCommitClick}
            selectedCommitSha={selectedCommitSha}
          />
        </XR>
      </Canvas>
    </div>
  )
}
