
struct Car {
  color:    String,
  transmission:      Transmission,
  convertible:        bool,
  mileage:            u32,
}

#[derive(PartialEq, Debug)]
enum Transmission {
  Manual,
  SemiAuto,
  Automatic,
}

fn car_factory(color: String, transmission: Transmission, convertible: bool ) -> Car {
  let car: Car = Car {
    color,
    transmission,
    convertible,
    mileage: 0
  };
  assert_eq!(car.mileage, 0);

  if car.convertible {
    println!("New car = {}, {:?}, Convertible", car.color, car.transmission);
  } else {
    println!("New car = {}, {:?}, Hardtop", car.color, car.transmission);
  }

  car
}

pub fn comm() {
  let client_request_1 = car_factory(String::from("Red"),
          Transmission::Manual, true);
  assert_eq!(client_request_1.color, "Red");
  assert_eq!(client_request_1.transmission, Transmission::Manual);
  // assert_eq!(client_request_1.convertible, false);
}








