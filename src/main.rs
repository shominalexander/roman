fn request() -> String {
 let mut value: String = String::new();

 std::io::stdin().read_line(&mut value).expect("Input failed");

 value = value.trim().to_string();
 value = value.replace("\n", "") ;
 value = value.replace("\r", "") ;

 value
}//fn request() -> String {

fn main(){
 loop {
  println!("\r\n\r\nroman:"); 

  let mut roman: String = request();

  if &roman[..] == "exit" {
   break;   

  } else {//if &roman[..] == "exit" {
   let mut arabic : u16 = 0u16;
   let mut maximum: u16 = 0u16;

   roman = (&roman[..]).to_uppercase().to_string();

   for char in roman.chars().rev() {
    let current: u16;

    match char {
     'C' => { current = 0100u16 }
     'D' => { current = 0500u16 }
     'I' => { current = 0001u16 }
     'L' => { current = 0050u16 }
     'M' => { current = 1000u16 }
     'V' => { current = 0005u16 }
     'X' => { current = 0010u16 }
      _  => { current = 0000u16 }
    }//match char {

    if current > maximum { maximum = current; }
    if current < maximum { arabic -= current; } else { arabic += current; }
   }//for char in roman.chars().rev() {

   println!("\r\narabic:\r\n{:?}", arabic);
  }//} else {//if &roman[..] == "exit" {
 }//loop {
}//fn main(){
