fn main() {
#[repr(u8)]
#[derive(Debug)]
enum Vehicle {
  Car = 0,
  Plane = 1,
}

let vehicle: Vehicle = unsafe { std::mem::transmute(3u8) };
println!("{vehicle:?} <- Incorrect! We have violated type safety");

unsafe {
  let ptr: *mut usize = 0xdeadbeef as usize as *mut usize;
  *ptr = 12;
}
}
