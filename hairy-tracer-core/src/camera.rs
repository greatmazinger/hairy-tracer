use glam::{DVec3, DQuat, DMat3};

#[derive(Debug, Clone, Copy)]
pub struct CameraOrientation(pub DQuat);

impl CameraOrientation {
    pub fn from_look_at(origin: DVec3, look_at: DVec3, up: DVec3) -> Self {
        let w = origin - look_at;
        let w_norm = w.length();
        let w = if w_norm == 0.0 {
            DVec3::new(0.0, 0.0, 1.0)
        } else {
            w / w_norm
        };

        let u = up.cross(w);
        let u_norm = u.length();
        let u = if u_norm == 0.0 {
            DVec3::new(1.0, 0.0, 0.0)
        } else {
            u / u_norm
        };

        let v = w.cross(u);

        // Construct a rotation matrix from basis vectors.
        // The columns of the matrix are the basis vectors of the new coordinate frame.
        // Since u, v, w map to the X, Y, Z axes of the camera frame, the rotation matrix
        // converting from camera space to world space is simply [u, v, w].
        let mat = DMat3::from_cols(u, v, w);
        CameraOrientation(DQuat::from_mat3(&mat).normalize())
    }

    pub fn from_axis_angle(axis: DVec3, angle: f64) -> Self {
        CameraOrientation(DQuat::from_axis_angle(axis.normalize(), angle))
    }

    pub fn slerp(&self, other: &Self, t: f64) -> Self {
        CameraOrientation(self.0.slerp(other.0, t))
    }

    pub fn basis_vectors(&self) -> (DVec3, DVec3, DVec3) {
        let mat = DMat3::from_quat(self.0);
        (mat.col(0), mat.col(1), mat.col(2)) // u, v, w
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::DVec3;

    fn assert_vec_eq(a: DVec3, b: DVec3) {
        assert!((a - b).length() < 1e-6, "Vectors not equal: {:?} vs {:?}", a, b);
    }

    #[test]
    fn test_round_trip() {
        let origin = DVec3::new(0.0, 200.0, 0.0);
        let look_at = DVec3::new(1000.0, 200.0, 0.0);
        let up = DVec3::new(0.0, 1.0, 0.0);

        let orient = CameraOrientation::from_look_at(origin, look_at, up);
        let (u, v, w) = orient.basis_vectors();

        // Manual basis calculation to compare
        let w_true = (origin - look_at).normalize();
        let u_true = up.cross(w_true).normalize();
        let v_true = w_true.cross(u_true);

        assert_vec_eq(u, u_true);
        assert_vec_eq(v, v_true);
        assert_vec_eq(w, w_true);
    }

    #[test]
    fn test_axis_angle() {
        let orient = CameraOrientation::from_axis_angle(DVec3::Y, std::f64::consts::PI / 2.0);
        let (u, v, w) = orient.basis_vectors();
        
        // 90 degrees around Y axis:
        // Original basis: u=(1,0,0), v=(0,1,0), w=(0,0,1)
        // Rotated: X axis (u) moves to -Z (0,0,-1)
        // Y axis (v) stays Y (0,1,0)
        // Z axis (w) moves to X (1,0,0)
        assert_vec_eq(u, DVec3::new(0.0, 0.0, -1.0));
        assert_vec_eq(v, DVec3::new(0.0, 1.0, 0.0));
        assert_vec_eq(w, DVec3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_slerp() {
        let orient1 = CameraOrientation::from_axis_angle(DVec3::Y, 0.0);
        let orient2 = CameraOrientation::from_axis_angle(DVec3::Y, std::f64::consts::PI / 2.0);

        let t0 = orient1.slerp(&orient2, 0.0);
        let t1 = orient1.slerp(&orient2, 1.0);
        let t05 = orient1.slerp(&orient2, 0.5);

        assert_vec_eq(t0.basis_vectors().0, DVec3::new(1.0, 0.0, 0.0));
        assert_vec_eq(t1.basis_vectors().0, DVec3::new(0.0, 0.0, -1.0));
        
        // 45 degrees around Y: X moves to (cos 45, 0, -sin 45)
        let root2_over_2 = (std::f64::consts::PI / 4.0).cos();
        assert_vec_eq(t05.basis_vectors().0, DVec3::new(root2_over_2, 0.0, -root2_over_2));
    }
}
