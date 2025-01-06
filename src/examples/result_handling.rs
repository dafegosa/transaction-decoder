// using match

fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
  if b == 0 {
      Err("Cannot divide by zero")
  } else {
      Ok(a / b)
  }
}

fn main() {
  let result = divide(10, 0);

  match result {
      Ok(value) => println!("Division result: {}", value),
      Err(err) => println!("Error: {}", err),
  }
}

// // using ? operator

// fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
//   if b == 0 {
//       Err("Cannot divide by zero")
//   } else {
//       Ok(a / b)
//   }
// }

// fn calculate() -> Result<i32, &'static str> {
//   let result = divide(10, 2)?; // propagate error
//   Ok(result + 5) // Si es Ok, seguimos con el cálculo.
// }

// fn main() {
//   match calculate() {
//       Ok(value) => println!("Final result: {}", value),
//       Err(err) => println!("Error: {}", err),
//   }
// }


// // using thiserror crate

// use thiserror::Error;

// #[derive(Debug, Error)]
// enum MathError {
//     #[error("Cannot divide by zero")]
//     DivideByZero,
//     #[error("Other math error")]
//     Other,
// }

// fn divide(a: i32, b: i32) -> Result<i32, MathError> {
//     if b == 0 {
//         Err(MathError::DivideByZero)
//     } else {
//         Ok(a / b)
//     }
// }

// fn calculate() -> Result<i32, MathError> {
//     let result = divide(10, 0)?; // propagate error
//     Ok(result + 5)
// }

// fn main() {
//     match calculate() {
//         Ok(value) => println!("Final result: {}", value),
//         Err(err) => println!("Error: {}", err),
//     }
// }

// // using anyhow crate

// use anyhow::{Result, Context};

// fn divide(a: i32, b: i32) -> Result<i32> {
//     if b == 0 {
//         Err(anyhow::anyhow!("Cannot divide by zero")) // Crea un error fácilmente.
//     } else {
//         Ok(a / b)
//     }
// }

// fn calculate() -> Result<i32> {
//     let result = divide(10, 0).context("Failed to calculate division")?; // Añade contexto al error.
//     Ok(result + 5)
// }

// fn main() {
//     match calculate() {
//         Ok(value) => println!("Final result: {}", value),
//         Err(err) => println!("Error: {}", err),
//     }
// }



