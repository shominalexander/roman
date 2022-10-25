fn main(){
 let mut arabic : u16;
 let mut current: u16;
 let mut maximum: u16;

 loop {
  let mut roman: String = String::new();

  println!("\r\n\r\nroman:"); 

  std::io::stdin().read_line(&mut roman).expect("Input failed");

  roman = roman.replace("\n", "");
  roman = roman.replace("\r", "");

  if &roman[..] == "exit" {
   break;   

  } else {//if &roman[..] == "exit" {
   arabic  = 0u16;
   current = 0u16;
   maximum = 0u16;

   roman = (&roman[..]).to_uppercase().to_string();

   for char in roman.chars().rev() {
    match char {
     'C' => { current = 0100u16 }
     'D' => { current = 0500u16 }
     'I' => { current = 0001u16 }
     'L' => { current = 0050u16 }
     'M' => { current = 1000u16 }
     'V' => { current = 0005u16 }
     'X' => { current = 0010u16 }
     _   => { current = 0000u16 }
    }//match char {

    if current > maximum { maximum = current; }
    if current < maximum { arabic -= current; } else { arabic += current; }
   }//for char in roman.chars().rev() {

   println!("\r\narabic:\r\n{:?}", arabic);
  }//} else {//if &roman[..] == "exit" {
 }//loop {
}//fn main(){
