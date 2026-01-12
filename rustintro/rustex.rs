fn main() {
#[repr(u8)]
#[derive(Debug)]
enum Vehicle {
  Car = 0,
  Plane = 1,
}

let vehicle: Vehicle = unsafe { std::mem::transmute(3u8) };
println!("{vehicle:?}")
}
