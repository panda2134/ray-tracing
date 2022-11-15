use std::{io::BufReader, sync::Arc};

use bvh::aabb::Bounded;
use glm::{DVec4, DMat4};
use itertools::Itertools;
use nalgebra_glm::DVec3;
use obj::{Obj, Vertex};
use serde::{Serialize, Deserialize};

use crate::{
    hit::{HitRecord, Ray},
    material::Material,
};
use nalgebra_glm as glm;

use super::Shape;

#[derive(Clone, Serialize, Deserialize)]
pub struct Triangle {
    pub points: [DVec3; 3],
    pub material: Arc<dyn Material>,
}

impl Bounded for Triangle {
    fn aabb(&self) -> bvh::aabb::AABB {
        let min = bvh::Point3::new(
            self.points
                .iter()
                .map(|p| p.x)
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap() as f32,
            self.points
                .iter()
                .map(|p| p.y)
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap() as f32,
            self.points
                .iter()
                .map(|p| p.z)
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap() as f32,
        );
        let max = bvh::Point3::new(
            self.points
                .iter()
                .map(|p| p.x)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap() as f32,
            self.points
                .iter()
                .map(|p| p.y)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap() as f32,
            self.points
                .iter()
                .map(|p| p.z)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap() as f32,
        );
        bvh::aabb::AABB { min, max }
    }
}

impl Triangle {
    fn v1(&self) -> DVec3 {
        self.points[1] - self.points[0]
    }
    fn v2(&self) -> DVec3 {
        self.points[2] - self.points[0]
    }
}

#[typetag::serde]
impl Shape for Triangle {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        // p0 + k1v1 + k2v2 = o + k*d
        /*
            [v1 v2 -d] * [k1 k2 k]^T = o - p0
            [k1 k2 k]^T = [v1 v2 -d]^{-1} * (o - p0)
        */
        let m = glm::DMat3::from_columns(&[self.v1(), self.v2(), -ray.direction]);
        let m_inv = match m.try_inverse() {
            Some(h) => h,
            None => return None,
        };
        let res = m_inv * (ray.origin - self.points[0]);
        let (k1, k2, toi) = (res.x, res.y, res.z);
        if k1 < 0.0 || k1 > 1.0 {
            return None;
        }
        if k2 < 0.0 || k2 > 1.0 {
            return None;
        }
        if k1 + k2 > 1.0 {
            return None;
        }
        let point = ray.origin + toi * ray.direction;
        let normal = self.v1().cross(&self.v2()).normalize();
        
        Some(HitRecord { toi, point, normal })
    }

    fn material(&self, _hit: &HitRecord) -> Arc<dyn Material> {
        self.material.clone()
    }
}

pub fn load_triangle(buffer: &[u8], model_matrix: &DMat4, material: Arc<dyn Material>) -> anyhow::Result<Vec<Triangle>> {
    let obj_contents: Obj<Vertex, usize> = obj::load_obj(BufReader::new(buffer))?;
    assert!(obj_contents.indices.len() % 3 == 0);
    let triangle_cnt = obj_contents.indices.len() / 3;
    let get_point = |i: usize| -> DVec3 {
        let t = (obj_contents.vertices[i] as Vertex).position;
        (model_matrix * DVec4::new(t[0] as f64, t[1] as f64, t[2] as f64, 1.0)).xyz()
    };
    Ok((0..triangle_cnt)
        .into_iter()
        .map(|i| Triangle {
            material: material.clone(),
            points: [
                get_point(obj_contents.indices[3 * i]), 
                get_point(obj_contents.indices[3 * i + 1]), 
                get_point(obj_contents.indices[3 * i + 2])
            ],
        })
        .collect_vec())
}

pub fn draw_rect(points: &[DVec3; 4], material: &Arc<dyn Material>) -> [Arc<dyn Shape>; 2] {
    [
        Arc::new(Triangle {
            points: [points[0], points[1], points[2]],
            material: material.clone()
        }),
        Arc::new(Triangle {
            points: [points[0], points[2], points[3]],
            material: material.clone() 
        }),
    ]
}

pub fn draw_cube(transform: &DMat4, material: &Arc<dyn Material>) -> Vec<Arc<dyn Shape>> {
    let h = 0.5; // default to unit cube

    let t = |v: &DVec3| -> DVec3 {
        (transform * DVec4::new(v.x, v.y, v.z, 1.0)).xyz()
    };

    let front = draw_rect(&[
        t(&DVec3::new(-h, h, h)), t(&DVec3::new(-h, -h, h)), t(&DVec3::new(h, -h, h)), t(&DVec3::new(h, h, h)),
    ], material).into_iter();
    let back = draw_rect(&[
        t(&DVec3::new(-h, h, -h)), t(&DVec3::new(h, h, -h)), t(&DVec3::new(h, -h, -h)), t(&DVec3::new(-h, -h, -h)), 
    ], material).into_iter();
    let right = draw_rect(&[
        t(&DVec3::new(h, h, h)), t(&DVec3::new(h, -h, h)), t(&DVec3::new(h, -h, -h)), t(&DVec3::new(h, h, -h))
    ], material).into_iter();
    let left = draw_rect(&[
        t(&DVec3::new(-h, h, h)), t(&DVec3::new(-h, h, -h)), t(&DVec3::new(-h, -h, -h)), t(&DVec3::new(-h, -h, h))
    ], material).into_iter();
    let top = draw_rect(&[
        t(&DVec3::new(-h, h, -h)), t(&DVec3::new(-h, h, h)), t(&DVec3::new(h, h, h)), t(&DVec3::new(h, h, -h))
    ], material).into_iter();
    let bottom = draw_rect(&[
        t(&DVec3::new(-h, -h, -h)), t(&DVec3::new(h, -h, -h)), t(&DVec3::new(h, -h, h)), t(&DVec3::new(-h, -h, h))
    ], material).into_iter();

    back.chain(front).chain(left).chain(right).chain(bottom).chain(top).collect_vec()
}