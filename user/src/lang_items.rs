use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
  let err = info.message().unwrap();
  if let Some(location) = info.location() {
    println!(
      "Paniced at {}:{}, {}",
      location.file(),
      location.line(),
      err
    );
  } else {
    println!("Paniced: {}", err);
  }
  loop {}
}
