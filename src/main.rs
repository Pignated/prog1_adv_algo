use std::cmp::PartialEq;
use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;
use std::ops::Range;
use rand::Rng;
use serde::{Deserialize, Serialize};

fn main() {
    let t1:Test = Test{
        polygon:vec![Point::new(1.0, 9.0), Point::new(8.0, 3.0), Point::new(0.0, -6.0), Point::new(-4.0, 2.0)],
        point:Point::new(2.0,2.0),
        inside:true,
    };
    assert!(check_point(&Point::new(2.0, 2.0), &vec![Point::new(1.0, 9.0), Point::new(8.0, 3.0), Point::new(0.0, -6.0), Point::new(-4.0, 2.0)]));
    // assert!(!check_point(&Point::new(2.0, 2.0), &vec![Point::new(1.0, 9.0), Point::new(8.0, 3.0), Point::new(0.0, 6.0), Point::new(-4.0, 2.0)]));
    // assert!(check_point(&Point::new(2.0, 2.0), &vec![Point::new(1.5, 3.0), Point::new(1.0, 9.0), Point::new(8.0, 3.0), Point::new(0.0, -6.0), Point::new(-4.0, 2.0), Point::new(-2.0, 1.0)]));
    // assert!(check_point(&Point::new(0.0,0.0),&vec![Point::new(1.0,0.0),Point::new(0.0,1.0),Point::new(-1.0,0.0),Point::new(0.0,-1.0)]));
    let t1_str = serde_json::to_string(&t1).expect("Couldn't serialize test data");
    let mut file = File::create("testfile").expect("lol");
    file.write_all(t1_str.as_bytes()).expect("TODO: panic message");
    println!("Tests Passed!!!");
}



fn check_point(point: &Point, polygon: &[Point]) -> bool {
    let mut left_intersections = 0;
    for i in 0..polygon.len()  {

        let p0 = polygon[i];
        let p1 = polygon[(i+1).rem_euclid(polygon.len())];
        if p0.y == p1.y {
            print!("1{:?}",polygon[i]);
            continue;
        }
        //If a vertex is on the line, add one intersection
        if p0.y == point.y && p0.x <= point.x {
            left_intersections += 1;
            print!("2{:?}",polygon[i]);
            //If both the vertex before and after that vertex are on the same side of the line, add another (as it enters than exits. One could argue it never entered but it gives the same end result)
            if (polygon[((i) as isize -1).rem_euclid(polygon.len() as isize) as usize].y - point.y)*(p1.y-point.y) > 0.0 {
                print!("3{:?}",polygon[i]);
                left_intersections += 1;
            }
        }
        if p0.y.min(p1.y) < point.y && p0.y.max(p1.y) > point.y {
            let x_interesect = ((point.y - p0.y)*(p0.x - p1.x)/(p0.y-p1.y))+p0.x;
            if x_interesect < point.x {
                print!("4{:?}",polygon[i]);
                left_intersections += 1;
            }
        }
    }
    return left_intersections % 2 == 1;
}

fn generate_random_polygon() ->Vec<Point>{
    let mut rng = rand::thread_rng();
    let num_sides = rng.gen_range(3..11);
    let mut unsorted_points : Vec<(Point,f64)> = Vec::with_capacity(num_sides);
    let mut points : Vec<Point> = Vec::with_capacity(num_sides);
    for _ in 0..num_sides {
        let point = Point::random_point(-10.0..10.0, -10.0..10.0);
        let angle = point.angle();
        unsorted_points.push((point,angle));
    }
     unsorted_points.sort_by(|a,b| {
         let val = a.1.total_cmp(&b.1);
         return val;
     });
    for i in 0..unsorted_points.len(){
        let a = unsorted_points[i];
        points.push(unsorted_points[i].0);
    }


    points
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
struct Point{
    x:f64,
    y:f64
}




impl Point{
    fn new(x:f64,y:f64)->Point{
        Point{x,y}
    }
    fn angle(&self)->f64{
        let mut angle = (self.y/self.magnitude()).acos();
        if self.x < 0.0 {
            angle = 2.0*PI - angle;
        }
        angle
    }
    fn magnitude(&self)->f64{
        return (self.x * self.x + self.y * self.y).sqrt();
    }
    fn random_point(x_range: Range<f64>, y_range:Range<f64>) -> Point{
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(x_range);
        let y = rng.gen_range(y_range);
        Point::new(x,y)
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
#[derive(Serialize,Deserialize,Debug,Clone)]
struct Test {
    polygon: Vec<Point>,
    point: Point,
    inside:bool
}