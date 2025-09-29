use std::cmp::PartialEq;
use std::fs;
// use rand::Rng;
use serde::{Deserialize, Serialize};

fn main() {

}



fn check_point(point: &Point, polygon: &[Point]) -> bool {
    let mut left_intersections = 0;
    for i in 0..polygon.len()  {

        let p0 = polygon[i];
        let p1 = polygon[(i+1).rem_euclid(polygon.len())];
        if p0.y == p1.y {
            // print!("1{:?}",polygon[i]);
            continue;
        }
        //If a vertex is on the line, add one intersection
        if p0.y == point.y && p0.x <= point.x {
            left_intersections += 1;
            // print!("2{:?}",polygon[i]);
            //If both the vertex before and after that vertex are on the same side of the line, add another (as it enters than exits. One could argue it never entered but it gives the same end result)
            if (polygon[((i) as isize - 1).rem_euclid(polygon.len() as isize) as usize].y - point.y)*(p1.y-point.y) >= 0.0 {
                // print!("3{:?}",polygon[i]);
                left_intersections += 1;
            }
        }
        //Check if p0 and p1 are on different sides of the line
        if p0.y.min(p1.y) < point.y && p0.y.max(p1.y) > point.y {
            let x_interesect = ((point.y - p0.y)*(p0.x - p1.x)/(p0.y-p1.y))+p0.x;
            if x_interesect < point.x {
                // print!("4{:?}",polygon[i]);
                left_intersections += 1;
            }
        }
    }
    return left_intersections % 2 == 1;
}

//Likely can delete this function vv

// fn generate_random_polygon() ->Vec<Point>{
//     let mut rng = rand::thread_rng();
//     let num_sides = rng.gen_range(3..11);
//     let mut unsorted_points : Vec<(Point,f64)> = Vec::with_capacity(num_sides);
//     let mut points : Vec<Point> = Vec::with_capacity(num_sides);
//     for _ in 0..num_sides {
//         let point = Point::random_point(-10.0..10.0, -10.0..10.0);
//         let angle = point.angle();
//         unsorted_points.push((point,angle));
//     }
//      unsorted_points.sort_by(|a,b| {
//          let val = a.1.total_cmp(&b.1);
//          return val;
//      });
//     for i in 0..unsorted_points.len(){
//         let a = unsorted_points[i];
//         points.push(unsorted_points[i].0);
//     }
//
//
//     points
// }

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
struct Point{
    x:f64,
    y:f64
}




impl Point{
    fn new(x:f64,y:f64)->Point{
        Point{x,y}
    }
    //Likely can delete this method vv
    // fn random_point(x_range: Range<f64>, y_range:Range<f64>) -> Point{
    //     let mut rng = rand::thread_rng();
    //     let x = rng.gen_range(x_range);
    //     let y = rng.gen_range(y_range);
    //     Point::new(x,y)
    // }
}
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
struct SerializedPoint {
    coord: (f64,f64)
}
impl SerializedPoint{
    fn to_point(self) -> Point{
        Point::new(self.coord.0, self.coord.1)
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
#[derive(Serialize,Deserialize,Debug,Clone)]
struct Test {
    polygon: Vec<SerializedPoint>,
    point: SerializedPoint,
    inside:bool
}

impl Test {
    fn check(&self) {
        let polygon_converted:Vec<Point> = self.polygon.iter().map(|p| p.to_point()).collect();
        let point_converted = self.point.to_point();
        let check_value = check_point(&point_converted, &polygon_converted);
        if check_value != self.inside {
            print!("Polygon: {:?}",self.polygon );
        }
        assert_eq!(check_value, self.inside);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_algo() {
        let json_file_text = fs::read_to_string("testfile.json");
        let test_file : Vec<Test> = serde_json::from_str(&json_file_text.expect("")).unwrap();
        for test in test_file {
            test.check();
        }
    }

}