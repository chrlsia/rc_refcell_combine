fn main(){
   //into_iter

   let v=vec![1,2,3];
   for number in v.into_iter(){
    println!("numbers is {number}");
   }

   // v has been consumed in the for loop
   // println!("v now in {v:?}");

//    let s=String::from("Chris Χρήστος");
//    for c in s.bytes(){
//     println!("c is {:?}",c);
   }

}