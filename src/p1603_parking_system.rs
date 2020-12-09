#![allow(dead_code)]

// problem
// Design a parking system for a parking lot. The parking lot has three kinds of parking spaces:
// big, medium, and small, with a fixed number of slots for each size.
// Implement the ParkingSystem class:

// ParkingSystem(int big, int medium, int small) Initializes object of the ParkingSystem class.
// The number of slots for each parking space are given as part of the constructor.
// bool addCar(int carType) Checks whether there is a parking space of carType for the car that
// wants to get into the parking lot. carType can be of three kinds:
// big, medium, or small, which are represented by 1, 2, and 3 respectively.
// A car can only park in a parking space of its carType.
// If there is no space available, return false, else park the car in that size space and return true.

struct ParkingSystem {
    slots: Vec<i32>,
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            slots: vec![0, big, medium, small],
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        if self.slots[car_type as usize] > 0 {
            self.slots[car_type as usize] -= 1;
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1603_parking_system_t1() {
        let mut obj = ParkingSystem::new(1, 1, 0);

        assert_eq!(obj.add_car(1), true);
        assert_eq!(obj.add_car(2), true);
        assert_eq!(obj.add_car(3), false);
        assert_eq!(obj.add_car(1), false);
    }
}
