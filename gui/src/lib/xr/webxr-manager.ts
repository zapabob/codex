// WebXR Manager for Windows 11 25H2 VR/AR Integration
// Enhanced VR experience with Windows-specific features

import * as THREE from 'three'
import { EventEmitter } from 'events'

export interface VRExperience {
  session: XRSession
  referenceSpace: XRReferenceSpace
  renderer: THREE.WebGLRenderer
  camera: THREE.PerspectiveCamera
  scene: THREE.Scene
}

export interface ARAnchor {
  id: string
  position: THREE.Vector3
  rotation: THREE.Quaternion
  commitData?: any
}

export interface HandTrackingData {
  hand: 'left' | 'right'
  joints: Map<string, THREE.Vector3>
  gestures: string[]
}

export class WebXRManager extends EventEmitter {
  private xrSession: XRSession | null = null
  private referenceSpace: XRReferenceSpace | null = null
  private renderer: THREE.WebGLRenderer | null = null
  private isVRMode = false
  private isARMode = false
  private anchors: Map<string, ARAnchor> = new Map()
  private handTrackingSupported = false

  constructor() {
    super()
    this.initializeXRSupport()
  }

  private async initializeXRSupport() {
    if ('xr' in navigator) {
      const xr = (navigator as any).xr

      // Check for VR support
      const vrSupported = await xr.isSessionSupported('immersive-vr')
      if (vrSupported) {
        console.log('WebXR Manager: VR supported')
      }

      // Check for AR support (Windows 11 25H2 specific)
      const arSupported = await xr.isSessionSupported('immersive-ar')
      if (arSupported) {
        console.log('WebXR Manager: AR supported (Windows 11 25H2)')
        this.isARMode = true
      }

      // Check for hand tracking (Windows 11 25H2)
      const handTrackingSupported = await xr.isSessionSupported('immersive-vr') &&
        xr.getSystem().hasFeature('hand-tracking')
      this.handTrackingSupported = handTrackingSupported

      if (handTrackingSupported) {
        console.log('WebXR Manager: Hand tracking supported')
      }
    } else {
      console.warn('WebXR Manager: WebXR not supported')
    }
  }

  async enterVR(): Promise<VRExperience | null> {
    try {
      if (!('xr' in navigator)) {
        throw new Error('WebXR not supported')
      }

      const xr = (navigator as any).xr
      this.xrSession = await xr.requestSession('immersive-vr', {
        requiredFeatures: ['local-floor', 'bounded-floor'],
        optionalFeatures: ['hand-tracking', 'anchors']
      })

      this.referenceSpace = await this.xrSession.requestReferenceSpace('local-floor')

      // Create WebGL renderer for XR
      this.renderer = new THREE.WebGLRenderer({
        antialias: true,
        alpha: true
      })

      // Setup XR session
      await this.renderer.xr.setSession(this.xrSession)

      const camera = new THREE.PerspectiveCamera()
      const scene = new THREE.Scene()

      // Windows 11 25H2 specific XR setup
      this.setupWindowsXRFeatures()

      this.isVRMode = true

      const experience: VRExperience = {
        session: this.xrSession,
        referenceSpace: this.referenceSpace,
        renderer: this.renderer,
        camera,
        scene
      }

      this.emit('vrEntered', experience)
      return experience

    } catch (error) {
      console.error('WebXR Manager: Failed to enter VR', error)
      this.emit('vrError', error)
      return null
    }
  }

  async enterAR(): Promise<VRExperience | null> {
    try {
      if (!this.isARMode) {
        throw new Error('AR not supported or not Windows 11 25H2')
      }

      const xr = (navigator as any).xr
      this.xrSession = await xr.requestSession('immersive-ar', {
        requiredFeatures: ['local-floor'],
        optionalFeatures: ['anchors', 'hit-test', 'light-estimation']
      })

      this.referenceSpace = await this.xrSession.requestReferenceSpace('local-floor')

      this.renderer = new THREE.WebGLRenderer({
        antialias: true,
        alpha: true
      })

      await this.renderer.xr.setSession(this.xrSession)

      const camera = new THREE.PerspectiveCamera()
      const scene = new THREE.Scene()

      // AR-specific setup
      this.setupARExperience(scene)

      this.isARMode = true

      const experience: VRExperience = {
        session: this.xrSession,
        referenceSpace: this.referenceSpace,
        renderer: this.renderer,
        camera,
        scene
      }

      this.emit('arEntered', experience)
      return experience

    } catch (error) {
      console.error('WebXR Manager: Failed to enter AR', error)
      this.emit('arError', error)
      return null
    }
  }

  private setupWindowsXRFeatures() {
    if (!this.xrSession) return

    // Enable Windows 11 25H2 specific features
    const session = this.xrSession as any

    // Hand tracking setup
    if (this.handTrackingSupported) {
      session.addEventListener('handtracking', (event: any) => {
        this.handleHandTracking(event)
      })
    }

    // Anchor creation for AR
    session.addEventListener('anchorcreated', (event: any) => {
      this.handleAnchorCreated(event)
    })

    // Windows-specific gesture recognition
    this.setupGestureRecognition()
  }

  private setupARExperience(scene: THREE.Scene) {
    // Add AR-specific lighting
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.6)
    scene.add(ambientLight)

    const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8)
    directionalLight.position.set(10, 10, 5)
    scene.add(directionalLight)

    // Setup hit testing for AR placement
    this.setupHitTesting()
  }

  private setupHitTesting() {
    if (!this.xrSession) return

    // Windows 11 25H2 AR hit testing
    const session = this.xrSession as any
    if (session.requestHitTestSource) {
      session.requestHitTestSource({
        space: this.referenceSpace,
        entityTypes: ['plane']
      }).then((hitTestSource: any) => {
        session.addEventListener('select', (event: any) => {
          const hitTestResults = session.getHitTestResults(hitTestSource)
          if (hitTestResults.length > 0) {
            const hit = hitTestResults[0]
            this.handleARPlacement(hit)
          }
        })
      })
    }
  }

  private handleHandTracking(event: any) {
    const handData: HandTrackingData = {
      hand: event.hand,
      joints: new Map(),
      gestures: []
    }

    // Process hand joints
    event.joints.forEach((joint: any) => {
      const position = new THREE.Vector3(
        joint.transform.position.x,
        joint.transform.position.y,
        joint.transform.position.z
      )
      handData.joints.set(joint.jointName, position)
    })

    // Detect gestures (Windows 11 25H2 enhanced)
    handData.gestures = this.detectGestures(handData.joints)

    this.emit('handTracking', handData)
  }

  private detectGestures(joints: Map<string, THREE.Vector3>): string[] {
    const gestures: string[] = []

    // Thumb up gesture
    const thumbTip = joints.get('thumb-tip')
    const indexTip = joints.get('index-finger-tip')
    if (thumbTip && indexTip) {
      const thumbUp = thumbTip.y > indexTip.y + 0.1
      if (thumbUp) gestures.push('thumb-up')
    }

    // Pointing gesture
    const indexExtended = this.isFingerExtended(joints, 'index')
    const middleFolded = !this.isFingerExtended(joints, 'middle')
    const ringFolded = !this.isFingerExtended(joints, 'ring')
    const pinkyFolded = !this.isFingerExtended(joints, 'pinky')

    if (indexExtended && middleFolded && ringFolded && pinkyFolded) {
      gestures.push('pointing')
    }

    // Pinch gesture
    const thumbIndexDistance = this.getFingerDistance(joints, 'thumb-tip', 'index-finger-tip')
    if (thumbIndexDistance < 0.05) {
      gestures.push('pinch')
    }

    return gestures
  }

  private isFingerExtended(joints: Map<string, THREE.Vector3>, finger: string): boolean {
    const tip = joints.get(`${finger}-tip`)
    const dip = joints.get(`${finger}-dip`)
    const pip = joints.get(`${finger}-pip`)
    const mcp = joints.get(`${finger}-mcp`)

    if (!tip || !dip || !pip || !mcp) return false

    // Simple extension check
    const extension = tip.distanceTo(mcp) > dip.distanceTo(mcp)
    return extension
  }

  private getFingerDistance(joints: Map<string, THREE.Vector3>, joint1: string, joint2: string): number {
    const j1 = joints.get(joint1)
    const j2 = joints.get(joint2)

    if (!j1 || !j2) return Infinity

    return j1.distanceTo(j2)
  }

  private handleAnchorCreated(event: any) {
    const anchor = event.anchor
    const transform = anchor.transform

    const arAnchor: ARAnchor = {
      id: anchor.uid,
      position: new THREE.Vector3(
        transform.position.x,
        transform.position.y,
        transform.position.z
      ),
      rotation: new THREE.Quaternion(
        transform.orientation.x,
        transform.orientation.y,
        transform.orientation.z,
        transform.orientation.w
      )
    }

    this.anchors.set(arAnchor.id, arAnchor)
    this.emit('anchorCreated', arAnchor)
  }

  private handleARPlacement(hit: any) {
    const position = new THREE.Vector3(
      hit.transform.position.x,
      hit.transform.position.y,
      hit.transform.position.z
    )

    this.emit('arPlacement', { position, hit })
  }

  private setupGestureRecognition() {
    // Windows 11 25H2 enhanced gesture recognition
    if (this.xrSession) {
      const session = this.xrSession as any

      session.addEventListener('gesture', (event: any) => {
        this.emit('gesture', {
          type: event.gesture,
          confidence: event.confidence,
          hand: event.hand
        })
      })
    }
  }

  // Git visualization specific methods
  setCommitAnchor(commitSha: string, position: THREE.Vector3): void {
    const anchor: ARAnchor = {
      id: `commit-${commitSha}`,
      position,
      rotation: new THREE.Quaternion(),
      commitData: { sha: commitSha }
    }

    this.anchors.set(anchor.id, anchor)
    this.emit('commitAnchorSet', anchor)
  }

  selectCommitByGesture(gesture: string, handData: HandTrackingData): void {
    if (gesture === 'pointing') {
      // Find commit in pointing direction
      const pointingDirection = this.calculatePointingDirection(handData)

      for (const anchor of this.anchors.values()) {
        if (anchor.commitData) {
          const distance = this.calculateDistanceToAnchor(pointingDirection, anchor)
          if (distance < 0.1) { // Within selection threshold
            this.emit('commitSelected', anchor.commitData)
            break
          }
        }
      }
    }
  }

  private calculatePointingDirection(handData: HandTrackingData): THREE.Vector3 {
    const indexTip = handData.joints.get('index-finger-tip')
    const indexDip = handData.joints.get('index-finger-dip')

    if (!indexTip || !indexDip) return new THREE.Vector3()

    return new THREE.Vector3().subVectors(indexTip, indexDip).normalize()
  }

  private calculateDistanceToAnchor(direction: THREE.Vector3, anchor: ARAnchor): number {
    // Simplified distance calculation
    const anchorDirection = anchor.position.clone().normalize()
    return direction.angleTo(anchorDirection)
  }

  exitXR(): void {
    if (this.xrSession) {
      this.xrSession.end()
      this.xrSession = null
      this.referenceSpace = null
      this.isVRMode = false
      this.isARMode = false

      if (this.renderer) {
        this.renderer.dispose()
        this.renderer = null
      }

      this.anchors.clear()
      this.emit('xrExited')
    }
  }

  get isInVR(): boolean {
    return this.isVRMode
  }

  get isInAR(): boolean {
    return this.isARMode
  }

  getAnchors(): ARAnchor[] {
    return Array.from(this.anchors.values())
  }

  removeAnchor(anchorId: string): void {
    this.anchors.delete(anchorId)
    this.emit('anchorRemoved', anchorId)
  }
}
