fn main() {
    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Silver"), Transmission::Automatic, true);
    println!("Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);
    
    car = car_factory(String::from("Silver"), Transmission::Semiauto, true);
    println!("Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

}
struct Car {
    color : String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    Semiauto,
    Automatic
}

fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    let car = Car {
        color : color,
        transmission: transmission,
        convertible: convertible,
        mileage: 0,  
    };

    return car ;

}