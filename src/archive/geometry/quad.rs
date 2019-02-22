use ::types::{Point};

pub struct Quad {
    a: Point,
    b: Point,
    abt: Point
}

impl Quad {
    pub fn new(a: Point, b: Point, width: f32) -> Quad {
        // Make AB vector of length half the width. Then use AB vector to make AB perpendicular
    //    GPoint AB = Utility::UnitVector(A, B);
    //    float rad = width / 2.0f;
    //    AB.fX *= rad;
    //    AB.fY *= rad;
    //    GPoint ABT = GPoint::Make(-AB.fY, AB.fX);
       //
    //    GQuad Shell = {A, B, ABT};
       //
    //    return Shell;
    }

    pub fn get_points() -> Vec<Point> {
        unimplemented!();

    //     std::vector<GPoint> Points;
    // Points.emplace_back(GPoint{A.fX + ABT.fX, A.fY + ABT.fY});
    // Points.emplace_back(GPoint{A.fX - ABT.fX, A.fY - ABT.fY});
    // Points.emplace_back(GPoint{B.fX + ABT.fX, B.fY + ABT.fY});
    // Points.emplace_back(GPoint{B.fX - ABT.fX, B.fY - ABT.fY});
    //
    // return Points;
    }
}
