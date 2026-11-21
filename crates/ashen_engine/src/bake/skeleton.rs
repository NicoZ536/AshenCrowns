//! Starre Skelette und Keyframe-Posen für animierte Bake-Modelle.

use glam::{Mat4, Quat, Vec3};

#[derive(Clone, Debug)]
pub struct Bone {
    pub name: &'static str,
    pub parent: Option<usize>,
    /// Ruheposition relativ zum Elternknochen.
    pub offset: Vec3,
}

#[derive(Clone, Debug, Default)]
pub struct Skeleton {
    pub bones: Vec<Bone>,
}

/// Lokale Rotation und zusätzliche Verschiebung je Knochen.
#[derive(Clone, Debug)]
pub struct Pose {
    pub rot: Vec<Quat>,
    pub shift: Vec<Vec3>,
    pub scale: Vec<f32>,
}

impl Skeleton {
    pub fn add(&mut self, name: &'static str, parent: Option<usize>, offset: Vec3) -> usize {
        self.bones.push(Bone { name, parent, offset });
        self.bones.len() - 1
    }

    pub fn find(&self, name: &str) -> Option<usize> {
        self.bones.iter().position(|b| b.name == name)
    }

    pub fn rest(&self) -> Pose {
        let n = self.bones.len();
        Pose { rot: vec![Quat::IDENTITY; n], shift: vec![Vec3::ZERO; n], scale: vec![1.0; n] }
    }

    /// Weltmatrizen aller Knochen (Eltern stehen vor Kindern).
    pub fn world(&self, pose: &Pose) -> Vec<Mat4> {
        let mut out: Vec<Mat4> = Vec::with_capacity(self.bones.len());
        for (i, b) in self.bones.iter().enumerate() {
            let local = Mat4::from_scale_rotation_translation(Vec3::splat(pose.scale[i]), pose.rot[i], b.offset + pose.shift[i]);
            let m = match b.parent {
                Some(p) => out[p] * local,
                None => local,
            };
            out.push(m);
        }
        out
    }
}

impl Pose {
    pub fn rotate(&mut self, bone: usize, q: Quat) {
        self.rot[bone] = q * self.rot[bone];
    }

    /// Rotation um X (vor/zurück schwingen), Y (drehen), Z (seitlich) in Grad.
    pub fn euler(&mut self, bone: usize, x: f32, y: f32, z: f32) {
        self.rotate(bone, Quat::from_euler(glam::EulerRot::YXZ, y.to_radians(), x.to_radians(), z.to_radians()));
    }

    pub fn shift(&mut self, bone: usize, v: Vec3) {
        self.shift[bone] += v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hierarchy_composes() {
        let mut s = Skeleton::default();
        let root = s.add("root", None, Vec3::new(0.0, 1.0, 0.0));
        let arm = s.add("arm", Some(root), Vec3::new(1.0, 0.0, 0.0));
        let mut p = s.rest();
        p.euler(root, 0.0, 90.0, 0.0);
        let w = s.world(&p);
        let tip = w[arm].transform_point3(Vec3::ZERO);
        assert!((tip - Vec3::new(0.0, 1.0, -1.0)).length() < 1e-4, "{tip:?}");
    }
}
