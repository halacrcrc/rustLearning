fn main() {
    let color = ("blue","Green","red","silver");
    let mut car :Car;
    let mut engine :Transmission = Transmission::Manual;
    car = car_factory(String::from(color.0), engine, true, 0);
    println!("Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);

    engine = Transmission::Semiauto;
    car = car_factory(String::from(color.1), engine, false, 100);
    println!("Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);

    engine = Transmission::Automatic;
    car = car_factory(String::from(color.2), engine, true, 200);
    println!("Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);
}

#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof:bool,
    age:(Age, u32)
}

#[derive(PartialEq, Debug)]
enum Transmission {Manual,Semiauto, Automatic}

#[derive(PartialEq, Debug)]
enum Age {NewCar, UsedCar}

fn car_quality (miles: u32) -> (Age, u32) {
    let quality:(Age, u32)= (Age::NewCar, miles);
    return quality;
}

fn car_factory(color:String, motor: Transmission, roof: bool, miles: u32) -> Car {
    Car {
        color,
        motor,
        roof,
        age: car_quality(miles)
    }
}





