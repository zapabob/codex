//! Security module for malware detection and quarantine

pub mod malware_detector;
pub mod quarantine;

pub use malware_detector::{DetectionMethod, MalwareDetectionResult, MalwareDetector};
pub use quarantine::{Quarantine, QuarantineEntry};
