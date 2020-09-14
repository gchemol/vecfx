// [[file:~/Workspace/Programming/gchemol-rs/vecfx/vecfx.note::*imports][imports:1]]
use crate::{VecFloatExt, Vector3fVec};

use nalgebra as na;
// imports:1 ends here

// [[file:~/Workspace/Programming/gchemol-rs/vecfx/vecfx.note::*tests][tests:1]]
use crate::*;

use na::geometry::Rotation3;
use na::Unit;
use na::Vector3;

#[test]
fn test_rotation() {
    use approx::*;

    // C2H6
    let coords = vec![
        [-2.1524, 0.6849, 0.0000],
        [-1.7958, -0.3239, 0.0000],
        [-1.7957, 1.1893, -0.8737],
        [-3.2224, 0.6849, 0.0000],
        [-1.6391, 1.4108, 1.2574],
        [-1.9941, 2.4202, 1.2564],
        [-1.9973, 0.9075, 2.1311],
        [-0.5691, 1.4091, 1.2584],
    ];

    // 沿2-1-5角, 转动原子2, PI/4, 原子的新坐标为:
    // -1.5435571	0.0976385	0.6552523
    let coords_a2 = [-1.5435571, 0.0976385, 0.6552523];

    let p1 = Vector3::from(coords[0]);
    let p2 = Vector3::from(coords[1]);
    let p5 = Vector3::from(coords[4]);
    let p6 = Vector3::from(coords[5]);

    let v12 = p2 - &p1;
    let v15 = p5 - &p1;
    let normal_215 = v12.cross(&v15).normalize();

    // 求1-2-5角度
    let angle_215 = v12.angle(&v15);
    dbg!(angle_215/3.1415 * 180.0);

    // let axis = Vector3::y_axis();
    let angle = std::f64::consts::FRAC_PI_4;
    let rot = Rotation3::from_axis_angle(&Unit::new_normalize(normal_215), angle);
    let p1_prime = rot * v12 + p1;
    assert_relative_eq!(p1_prime, Vector3::from(coords_a2), epsilon = 1e-4);

    // 旋转5,6,7,8基团 PI/4
    let normal_215 = v15.cross(&v12).normalize();
    let rot = Rotation3::from_axis_angle(&Unit::new_normalize(normal_215), angle);
    let p6_prime = rot * (p6 - &p1) + p1;
    dbg!(p6_prime);



    // Point and vector being transformed in the tests.
    let n = Vector3::new(1.0, 0.1, 0.0);
    let t = Vector3::new(0.1, 1.0, 0.0);
    let normal = n.cross(&t).normalize();
    dbg!(normal);

    let rot = Rotation3::from_axis_angle(&Unit::new_normalize(normal), angle);

    // let rot = Rotation3::from_axis_angle(&vec.normalize(), angle);

    dbg!(rot * n);
}
// tests:1 ends here
