use std::fs;
use std::env;

#[derive(Debug)]
#[derive(Clone)]
struct Point (f32, f32);

#[derive(Debug)]
struct Segment {
    start: Point,
    end: Point
}

impl Segment {
    fn length(&self) -> f32 {
        return((self.start.0 - self.end.0) + (self.start.1 - self.end.1)).abs()
    }

    //https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#In_two_dimensions
    fn intersects(&self, segment: &Segment) -> Option<Point> {
        if (self.start.0 == 0.0 && self.start.1 == 0.0) {
            return None;
        }
        if (self.end.0 == 0.0 && self.end.1 == 0.0) {
            return None;
        }
        if (segment.start.0 == 0.0 && segment.start.1 == 0.0) {
            return None;
        }
        if (segment.end.0 == 0.0 && segment.end.1 == 0.0) {
            return None;
        }
        //println!("self.start.0: {} self.end.0: {}", self.start.0, self.end.0);
        //println!("self.start.1: {} self.end.1: {}", self.start.1, self.end.1);
        //println!("segment.start.0: {} segment.end.0: {}", segment.start.0, segment.end.0);
        //println!("segment.start.1: {} segment.end.1: {}", segment.start.1, segment.end.1);
    
        let x_1 = segment.start.0;
        let y_1 = segment.start.1;

        let x_2 = segment.end.0;
        let y_2 = segment.end.1;

        let x_3 = self.start.0;
        let y_3 = self.start.1;

        let x_4 = self.end.0;
        let y_4 = self.end.1;

        if (x_1-x_2)*(y_3-y_4) - (y_1-y_2)*(x_3-x_4) == 0.0 {
            return None;
        }

        let t_top = (x_1-x_3)*(y_3-y_4) - (y_1-y_3)*(x_3-x_4);
        let t_bottom = (x_1-x_2)*(y_3-y_4) - (y_1-y_2)*(x_3-x_4);
        

        let t = t_top / t_bottom;

        let u_top = (x_1-x_2)*(y_1-y_3) - (y_1-y_2)*(x_1-x_3);
        let u_bottom = (x_1-x_2)*(y_3-y_4) - (y_1-y_2)*(x_3-x_4);

        let u = -1.0 * (u_top / u_bottom);
        if t > 1.0 || t < 0.0 || u > 1.0 || u < 0.0 {
            return None;
        }

        //println!("t: {} u: {}", t, u);
        let x = x_1 + t*(x_2-x_1);
        let y = y_1 + t*(y_2-y_1);

        Some(Point(x, y))
    }
}


fn calculate_manhattan_distance(p: Point) -> i32 {
    (p.1.abs() + p.0.abs()) as i32
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let contents = fs::read_to_string(file_name).expect("Ruh roh");

    let routes: Vec<Vec<&str>> = contents.split("\n").map(|s| s.split(",").map(|s| s.trim()).collect()).collect();
    
    let route_one = &routes[0];
    let route_two = &routes[1];
    let map_one = create_map(route_one.to_vec());
    let map_two = create_map(route_two.to_vec());

    //println!("{:?}", routes);
    println!("{:?}", map_one);
    println!("{:?}", map_two);

    //print_map(map_one);
    find_intersections(map_one, map_two);
}

fn create_map(route: Vec<&str>) -> Vec<Point> {
    let mut map: Vec<Point> = vec![];
    let mut x = 0.0;
    let mut y = 0.0;
    let origin = Point(0.0, 0.0);
    map.push(origin);

    for instruction in route {
        let characters: Vec<char> = instruction.chars().collect();
        let direction = characters[0];
        let velocity = characters[1..characters.len()].into_iter().collect::<String>().parse::<i32>().unwrap() as f32;
    

        match direction {
            'R' => {
                x += velocity;
            }
            'L' => {
                x -= velocity;
            }
            'U' => {
                y += velocity;
            }
            'D' => {
                y -= velocity;
            }
            _ => break
        }

        let point = Point(x, y);
        map.push(point);
    }
    map
}

fn generate_segments(points: Vec<Point>) -> Vec<Segment> {

    let mut segments: Vec<Segment> = vec![];
    let mut point = &points[0];
    for i in 1..points.len() {
        let end = &points[i];
        let segment = Segment{start: point.clone(), end: end.clone()}; 
        point = end;
        segments.push(segment);
    }
    segments
}


fn find_intersections(map_one: Vec<Point>, map_two: Vec<Point>) {
    let map_one_segments = generate_segments(map_one);
    let map_two_segments = generate_segments(map_two);

    let mut least = 999999.0;
    

    let mut intersected_routes: Vec<Vec<Segment>> = vec![];

    let mut map_1_total = 0.0;

    for segment in map_one_segments {
        let mut map_2_total = 0.0;
        for segment2 in map_two_segments.iter() {
            let mut path_two: Vec<Segment> = vec![];
            let intersection =  segment.intersects(segment2);
            match intersection {
                Some(i) => {
                     let new_segment = Segment{start: segment.start.clone(), end: i.clone()};
                     let new_segment2 = Segment{start: segment2.start.clone(), end: i.clone()};
                     let total = map_2_total + map_1_total + new_segment.length() + new_segment2.length();
                     if least > total {
                         least = total
                     }
                     map_2_total += segment2.length();
                     break
                }
                _ => {
                    map_2_total += segment2.length();
                }
            }
        }
        map_1_total += segment.length();
    }
    println!("{}", least);
}




