use crate::vr_ar_integration::types::{HandGesture, HandPose};

/// Gesture recognition system
pub struct GestureRecognizer;

impl GestureRecognizer {
    pub fn new() -> Self {
        Self
    }

    pub fn recognize_gesture(&self, pose: &HandPose) -> Option<HandGesture> {
        // Simple gesture recognition based on joint positions
        // In real implementation, this would use ML models

        if pose.joints.len() < 21 {
            return Some(HandGesture::Unknown);
        }

        // Check if thumb and index finger are close (pinch gesture)
        let thumb_tip = pose.joints[4];
        let index_tip = pose.joints[8];
        let distance = ((thumb_tip[0] - index_tip[0]).powi(2)
            + (thumb_tip[1] - index_tip[1]).powi(2)
            + (thumb_tip[2] - index_tip[2]).powi(2))
        .sqrt();

        if distance < 0.05 {
            return Some(HandGesture::Pinch);
        }

        // Check if index finger is extended and others are closed (point gesture)
        let index_extended = pose.joints[8][1] > pose.joints[6][1]; // Simplified check
        let middle_closed = pose.joints[12][1] < pose.joints[10][1];
        let ring_closed = pose.joints[16][1] < pose.joints[14][1];
        let pinky_closed = pose.joints[20][1] < pose.joints[18][1];

        if index_extended && middle_closed && ring_closed && pinky_closed {
            return Some(HandGesture::Point);
        }

        Some(HandGesture::Open)
    }
}
