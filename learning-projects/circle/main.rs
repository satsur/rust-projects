//  Make a programme that allows the user to input either the radius, diameter, or area of the circle.
// The programme should then calculate the other 2 based on the input.
use std::f64;
use std::io;

#[derive(Debug, Clone)]
pub struct Circle {
    radius: Option<f64>,
    diameter: Option<f64>, 
    area: Option<f64>,
}

impl Circle {
    fn builder(base_circle: Option<&Circle>) -> CircleBuilder {
        CircleBuilder::new(base_circle)
    }
    
    fn find_radius(&self) -> Option<f64> {
        if self.radius != None {
            return Some(self.radius.unwrap());
        } else if self.diameter != None {
            return Some(self.diameter.unwrap() / 2.0);
        } else if self.area != None {
            return Some(f64::sqrt(&self.area.unwrap() / f64::consts::PI));
        }
        None
    }
    
    fn find_diameter(&self) -> Option<f64> {
        if self.diameter != None {
            return Some(self.diameter.unwrap());
        } else if self.radius != None {
            return Some(self.radius.unwrap() * 2.0);
        } else if self.area != None { // A = pi * r^2 = pi * d^2 / 4
            return Some(f64::sqrt(self.area.unwrap() / f64::consts::PI * 4.0));
        }
        None
    }
    
    fn find_area(&self) -> Option<f64> {
        if self.area != None {
            return Some(self.area.unwrap());
        } else if self.radius != None {
            return Some(f64::consts::PI * f64::powf(self.radius.unwrap(), 2.0));
        } else if self.diameter != None {
            return Some(f64::consts::PI * f64::powf(self.diameter.unwrap() / 2.0, 2.0));
        }
        None
    }
    
    fn build_with_missing_values(&self) -> Result<Self, &str> {
        let mut new_circle = Self::builder(Some(self));
        if self.radius != None {
            new_circle = new_circle
                        .diameter(self.find_diameter())
                        .area(self.find_area());
        } else if self.diameter != None {
            new_circle = new_circle
                        .radius(self.find_radius())
                        .area(self.find_area());
        } else if self.area != None {
            new_circle = new_circle
                .radius(self.find_radius())
                .diameter(self.find_diameter());
        }
        if new_circle.radius == None || new_circle.diameter == None || new_circle.area == None {
            return Err("Not all missing values were inferred.")
        }
        Ok(new_circle.build())
    }
}

struct CircleBuilder {
    radius: Option<f64>,
    diameter: Option<f64>,
    area: Option<f64>,
}

impl CircleBuilder {
    fn new(base_circle: Option<&Circle>) -> Self {
        if base_circle.is_some() {
            let circle = base_circle.unwrap();
            return Self {
                radius: circle.radius,
                diameter: circle.diameter,
                area: circle.area,
            }
        }
        return Self {
            radius: None,
            diameter: None,
            area: None,
        }
    }
    
    fn radius(mut self, radius: Option<f64>) -> Self {
        self.radius = radius;
        self
    }
    
    fn diameter(mut self, diameter: Option<f64>) -> Self {
        self.diameter = diameter;
        self
    }
    
    fn area(mut self, area: Option<f64>) -> Self {
        self.area = area;
        self
    }
    
    fn build(self) -> Circle {
        Circle {
            radius: self.radius,
            diameter: self.diameter,
            area: self.area,
        }
    }
}

fn main() {
    println!("Would you like to enter the radius (1), diameter (2), or area (3)?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let choice = input.trim().parse::<u8>().expect("Input was not a valid integer.");
    
    input.clear();
    println!("Enter the number: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number = input.trim().parse::<f64>().expect("Input was not a valid floating-point number.");
    
    let partial_circle = 
        Circle::builder(None)
        .radius(if choice == 1 { Some(number) } else { None })
        .diameter(if choice == 2 { Some(number) } else { None })
        .area(if choice == 3 { Some(number) } else { None })
        .build();
    
    let complete_circle = match partial_circle.build_with_missing_values() {
        Ok(t) => t,
        Err(e) => { 
            println!("{}: {:?}", e, partial_circle);
            partial_circle
        },
    };
    
    println!("{:?}", complete_circle);
}