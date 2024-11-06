use coordinate_vector::vector_2d::{CartesianCoordinatesVector, Vector2D};

fn main() {
    let v = Vector2D::new(1.2, -8.7);

    println!("v = {}, norm = {}, norm^2 = {}", v, v.norm(), v.norm_sqr());
}
