#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car { color: String, motor: Transmission, roof: bool, age: (Age, u32) }

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission { Manual, SemiAuto, Automatic }

#[derive(PartialEq, Debug)]
// Declare enum for Car age
enum Age { New, Used }

//////////////////////////////////////////////////

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Return tuple with car age ("New" or "Used") and mileage
fn car_quality (miles: u32) -> (Age, u32) {

    //todo!("Add conditional expression: If car has accumulated miles, return tuple for Used car with current mileage");
    let car_age:(Age, u32) = if miles > 0 {
        (Age::Used, miles)
    } else {
        (Age::New, 0)
    };
    //todo!("Return tuple for New car with zero miles");
    return car_age;
}

//////////////////////////////////////////////////

// Build a new "Car" using the values of four input arguments
// - color (String)
// - motor (Transmission enum)
// - roof (boolean, true if the car has a hard top roof)
// - miles (u32)
// Call the car_quality(miles) function to get the car age
// Return an instance of a "Car" struct with the arrow `->` syntax
fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {

    // Show details about car order
    // - Check if order is for Used or New car, then check the roof type 
    // - Print details for New or Used car based on roof type
    //todo!("Add conditional expression: If car is Used age, then check roof type");
    //todo!("Add conditional expression: If roof is a hard top, print details");
    // Call the `println!` macro to show the car order details
    //println!("Prepare a used car: {:?}, {}, Hard top, {} miles\n", motor, color, miles);  
    let roofs:String = if roof {
        String::from("Hard top")
    } else {
        String::from("Convertible")
    };
    if miles > 0 {
        println!("Prepare a used car: {:?}, {}, {}, {} miles\n", motor, color, roofs, miles);
    } else {
        println!("Build a new car: {:?}, {}, {}, {} miles\n", motor, color, roofs, miles);
    }

    // Create a new "Car" instance as requested
    // - Bind first three fields to values of input arguments
    // - Bind "age" to tuple returned from car_quality(miles)
    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles)
    }
}

fn main() {
    // Car order #1: New, Manual, Hard top
    car_factory(String::from("Orange"), Transmission::Manual, true, 0);

    // Car order #2: Used, Semi-automatic, Convertible
    car_factory(String::from("Red"), Transmission::SemiAuto, false, 565);

    // Car order #3: Used, Automatic, Hard top
    car_factory(String::from("White"), Transmission::Automatic, true, 3000);
}
