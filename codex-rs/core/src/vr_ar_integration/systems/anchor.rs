use crate::vr_ar_integration::types::Anchor;
use crate::vr_ar_integration::types::VREvent;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Mutex;

/// Anchor management system
pub struct AnchorSystem {
    anchors: Mutex<HashMap<String, Anchor>>,
}

impl Default for AnchorSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl AnchorSystem {
    pub fn new() -> Self {
        Self {
            anchors: Mutex::new(HashMap::new()),
        }
    }

    pub async fn add_anchor(&self, anchor: Anchor) -> Result<()> {
        let mut anchors = self
            .anchors
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to lock anchors: {e}"))?;
        anchors.insert(anchor.id.clone(), anchor);
        Ok(())
    }

    pub async fn find_nearest_anchor(
        &self,
        position: [f32; 3],
        max_distance: f32,
    ) -> Result<Option<Anchor>> {
        let anchors = self
            .anchors
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to lock anchors: {e}"))?;

        let mut nearest: Option<(&String, &Anchor, f32)> = None;

        for (id, anchor) in anchors.iter() {
            let distance = ((anchor.position[0] - position[0]).powi(2)
                + (anchor.position[1] - position[1]).powi(2)
                + (anchor.position[2] - position[2]).powi(2))
            .sqrt();

            if distance <= max_distance {
                match nearest {
                    Some((_, _, min_dist)) if distance < min_dist => {
                        nearest = Some((id, anchor, distance));
                    }
                    None => {
                        nearest = Some((id, anchor, distance));
                    }
                    _ => {}
                }
            }
        }

        Ok(nearest.map(|(_, anchor, _)| anchor.clone()))
    }

    pub async fn update(&self) -> Result<Vec<VREvent>> {
        // Mock anchor updates
        Ok(Vec::new())
    }
}
