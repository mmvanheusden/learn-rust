pub fn main() {
    // Now what if we implement methods on structs and enums and use generic types in their definitions.
    let rubix = Cube {width: 15.77, height:13};
    println!("rubix width = {}", rubix.width);
    
    let weird_cube = Cube {width: 5.543543543 as f32, height: 2.6784 as f32};
    
    println!("The volume of weird cube is approximately {} cm^2", weird_cube.getVolumeAfgerond())
}

struct Cube<T, U> {
    width: T,
    height: U,
}

impl<T, U> Cube<T, U> {
    fn getWidth(&self) -> &T {
        &self.width
    }
    fn getHeight(&self) -> &U {
        &self.height
    }
}


impl Cube<f32, f32> {
    fn getVolumeAfgerond(&self) -> i32 {
        (self.width * self.height).round() as i32
    }
}