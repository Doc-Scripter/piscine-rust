
pub fn area_fit(
    x: usize,
    y: usize,
    objects:GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    let container_area = rectangle_area(x, y);
    let object_area = match objects {
        GeometricalShapes::Square => square_area(a),
        GeometricalShapes::Circle => circle_area(a) as usize,
        GeometricalShapes::Rectangle => rectangle_area(a, b),
        GeometricalShapes::Triangle => triangle_area(a, b) as usize,
    };

    if object_area == 0 {
        return false;
    }

    container_area >= object_area * times
}

pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    let container_volume = parallelepiped_volume(x, y, z);
    let object_volume = match objects {
        GeometricalVolumes::Cube => cube_volume(a),
        GeometricalVolumes::Sphere => sphere_volume(a) as usize,
        GeometricalVolumes::Cone => cone_volume(a, b) as usize,
        GeometricalVolumes::Pyramid => {
            let base = triangle_area(a, a); // Using a as both base and height for triangle
            triangular_pyramid_volume(base, b) as usize
        }
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c),
    };

    if object_volume == 0 {
        return false;
    }

    container_volume >= object_volume * times
}

pub enum GeometricalShapes {
    Square,
    Circle,
    Rectangle,
    Triangle,
}

pub enum GeometricalVolumes {
    Cube,
    Sphere,
    Cone,
    Pyramid,
    Parallelepiped,
}

pub fn square_area(side: usize) -> usize {
    side.pow(2)
}

pub fn triangle_area(base: usize, height: usize) -> f64 {
    (base as f64 * height as f64) / 2.0
}

pub fn circle_area(radius: usize) -> f64 {
    std::f64::consts::PI * (radius.pow(2) as f64)
}

pub fn rectangle_area(side_a: usize, side_b: usize) -> usize {
    side_a * side_b
}

pub fn cube_volume(side: usize) -> usize {
    side.pow(3)
}

pub fn sphere_volume(radius: usize) -> f64 {
    (4.0 / 3.0) * std::f64::consts::PI * (radius.pow(3) as f64)
}

pub fn triangular_pyramid_volume(base_area: f64, height: usize) -> f64 {
    (base_area * height as f64) / 3.0
}

pub fn parallelepiped_volume(side_a: usize, side_b: usize, side_c: usize) -> usize {
    side_a * side_b * side_c
}

pub fn cone_volume(base_radius: usize, height: usize) -> f64 {
    (1.0 / 3.0) * std::f64::consts::PI * base_radius.pow(2) as f64 * height as f64
}