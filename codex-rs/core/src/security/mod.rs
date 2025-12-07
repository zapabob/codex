//! Security module for malware detection and quarantine

pub mod malware_detector;
pub mod quarantine;

pub use malware_detector::DetectionMethod;
pub use malware_detector::MalwareDetectionResult;
pub use malware_detector::MalwareDetector;
pub use quarantine::Quarantine;
pub use quarantine::QuarantineEntry;
