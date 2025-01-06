enum Point {
  Nothing,
  TuplePoint(i32, i32),
  StructPoint {
      x: i32,
      y: i32
  }
}

fn get_point(n: u8) -> Point {
  match n {
      1 => Point::TuplePoint(-1, 1),
      2 => Point::StructPoint {
          x: -1,
          y: 1
      },
      _ => Point::Nothing
  }
}

fn main() {
  let p = get_point(2);
  match p {
      Point::Nothing => println!("no point"),
      Point::TuplePoint(x, y) => println!("x is {} and y is {}", x, y),
      Point::StructPoint{x, y} => println!("x is {} and y is {}", x, y),
  }
}

// #[allow(unused)]
// enum Car <T, E, F>{
//     Tesla(T),
//     Ford(E),
//     Toyota(F),
// }

// fn main() {
//     let my_car: Car<String, i32, u32> = Car::Tesla("Model S".to_string());
//     match my_car {
//         Car::Tesla(adj) => println!("Tesla model {}", adj),
//         Car::Ford(adj) => println!("Ford model {}", adj),
//         Car::Toyota(adj) => println!("Toyota model {}", adj),
//     }
// }