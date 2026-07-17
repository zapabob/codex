use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum XRPlatform {
    OculusQuest2,
    OculusQuest3,
    AppleVisionPro,
    AppleGlass,
    HTCVive,
    SteamVR,
    VirtualDesktop,
    WindowsMixedReality,
    WebXR,
}

#[derive(Debug, Clone)]
pub struct VRController {
    pub platform: XRPlatform,
    pub position: [f32; 3],
    pub rotation: [f32; 4], // Quaternion
    pub buttons: HashMap<String, bool>,
    pub triggers: HashMap<String, f32>,
    pub joysticks: HashMap<String, [f32; 2]>,
}

#[derive(Debug, Clone)]
pub struct HandPose {
    pub hand: HandType,
    pub confidence: f32,
    pub joints: Vec<[f32; 3]>,
    pub gesture: HandGesture,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum HandType {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum HandGesture {
    Open,
    Closed,
    Point,
    ThumbUp,
    Peace,
    Pinch,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Anchor {
    pub id: String,
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
    pub anchor_type: AnchorType,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnchorType {
    Commit,
    Branch,
    Tag,
    TimePoint,
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum VREvent {
    ControllerUpdate(VRController),
    HandPoseUpdate(HandPose),
    AnchorCreated(Anchor),
    AnchorUpdated(Anchor),
    AnchorDeleted(String),
    GestureRecognized(HandGesture, HandType),
    PlatformConnected(XRPlatform),
    PlatformDisconnected(XRPlatform),
}

/// VR interaction commands
#[derive(Debug, Clone)]
pub enum VRInteraction {
    SelectAnchor(String),
    CreateTimeAnchor(String),
    ToggleBranchVisibility,
    ZoomToFit,
    RotateView(f32, f32, f32),    // pitch, yaw, roll
    TranslateView(f32, f32, f32), // x, y, z
    ScaleView(f32),
    Gesture(HandGesture),
    VoiceCommand(String),
}
