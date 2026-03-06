'use client'

import { useEffect, useMemo, useRef } from 'react'
import { Canvas, useFrame, useThree } from '@react-three/fiber'
import { VRButton, XR, createXRStore, useXR, useXRControllerState } from '@react-three/xr'
import { OrbitControls, PerspectiveCamera } from '@react-three/drei'
import * as THREE from 'three'

const xrStore = createXRStore()

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

function CommitNodesVR({ commits, onCommitClick }: Scene3DVXRProps) {
  const meshRef = useRef<THREE.InstancedMesh>(null)
  const { isPresenting } = useXR()
  const { camera } = useThree()

  const leftController = useXRControllerState('left')
  const rightController = useXRControllerState('right')

  const { matrices, colors } = useMemo(() => {
    const nextMatrices: THREE.Matrix4[] = []
    const nextColors: number[] = []

    commits.forEach((commit) => {
      const matrix = new THREE.Matrix4()
      const position = new THREE.Vector3(commit.x, commit.y / 1_000_000, commit.z)
      const scale = isPresenting ? 0.5 : 1.0

      matrix.compose(
        position,
        new THREE.Quaternion(),
        new THREE.Vector3(scale, scale, scale)
      )

      nextMatrices.push(matrix)

      const color = new THREE.Color(commit.color)
      nextColors.push(color.r, color.g, color.b)
    })

    return { matrices: nextMatrices, colors: nextColors }
  }, [commits, isPresenting])

  useEffect(() => {
    if (!meshRef.current) {
      return
    }

    matrices.forEach((matrix, index) => {
      meshRef.current?.setMatrixAt(index, matrix)
    })

    meshRef.current.geometry.setAttribute(
      'color',
      new THREE.InstancedBufferAttribute(new Float32Array(colors), 3)
    )
    meshRef.current.instanceMatrix.needsUpdate = true
  }, [colors, matrices])

  useFrame(() => {
    if (!meshRef.current || !isPresenting) {
      return
    }

    const raycaster = new THREE.Raycaster()
    raycaster.setFromCamera(new THREE.Vector2(0, 0), camera)
    const intersects = raycaster.intersectObject(meshRef.current)

    if (leftController?.controller && intersects.length > 0) {
      const actuator = leftController.inputSource?.gamepad?.hapticActuators?.[0]
      if (actuator) {
        void actuator.pulse(0.5, 100)
      }
    }

    if (rightController?.controller && intersects.length > 0 && intersects[0].instanceId !== undefined) {
      const index = intersects[0].instanceId
      const commit = commits[index]
      if (!commit) {
        return
      }

      if (rightController.inputSource?.gamepad?.buttons[0]?.pressed) {
        onCommitClick?.(commit)

        const actuator = rightController.inputSource.gamepad.hapticActuators?.[0]
        if (actuator) {
          void actuator.pulse(1.0, 200)
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

function VRScene({ commits, onCommitClick, selectedCommitSha }: Scene3DVXRProps) {
  return (
    <>
      <ambientLight intensity={0.4} />
      <pointLight position={[10, 10, 10]} intensity={1} />
      <pointLight position={[-10, -10, -10]} intensity={0.5} color="#764ba2" />

      <CommitNodesVR
        commits={commits}
        onCommitClick={onCommitClick}
        selectedCommitSha={selectedCommitSha}
      />

      <gridHelper args={[100, 100, '#ffffff', '#333333']} />
      <axesHelper args={[50]} />
    </>
  )
}

export default function Scene3DVXR({
  commits,
  onCommitClick,
  selectedCommitSha,
}: Scene3DVXRProps) {
  return (
    <div className="w-full h-full">
      <Canvas>
        <XR store={xrStore}>
          <VRButton />

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
