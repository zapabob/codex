//! Security module for malware detection and quarantine

pub mod malware_detector;
pub mod quarantine;

pub use malware_detector::{MalwareDetector, MalwareDetectionResult, DetectionMethod};
pub use quarantine::{Quarantine, QuarantineEntry};

