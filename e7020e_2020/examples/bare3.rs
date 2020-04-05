//! bare3.rs
//!
//! String types in Rust
//!
//! What it covers:
//! - Types, str, arrays ([u8; usize]), slices (&[u8])
//! - Iteration, copy
//! - Semihosting (tracing using `hprintln`

#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::{hprint, hprintln};

#[entry]
fn main() -> ! {
    hprintln!("bare3").unwrap();
  //  let s = "ABCD"; 
 //   let bs = s.as_bytes(); // Returns a byte slice (pedazo) of the string 
   // Como las letras estan en codigo ASCII cada una es un byte (u8), por tanto bs será
   // un array con los cuatro pedazos (tiene el valor del caracter en la tabla ASCII)
   // Lo que hace es devolver el string en forma de array con su codificacion ASCII (valor numerico)
   let s: &str = "ABCD";
   let bs: &[u8] = s.as_bytes();
    hprintln!("s = {}", s).unwrap();
    hprintln!("bs = {:?}", bs).unwrap();

    hprintln!("iterate over slice").unwrap();
    let c: u8;
    for c in bs {
        hprint!("{},", c).unwrap();
    }

    hprintln!("iterate iterate using (raw) indexing").unwrap();
    let i: u8;
    for i in 0..s.len() { // Iterate since i = 0 until i = s.len()
        hprintln!("{},", bs[i]).unwrap();
    //    hprintln!("{}",i).unwrap(); // Prints i values (0-3)
    }

    hprintln!("").unwrap(); // Imprime linea vacia

   //  let a = [65u8; 4]; // Creas un vector de 4 elementos de un byte de valor 65
   let a: [u8;4] = [65u8; 4];
   // let mut a = [0u8; 4];
 
   
    hprintln!("").unwrap();
    hprintln!("a = {}", core::str::from_utf8(&a).unwrap()).unwrap();
    // from_utf8(&a) converts a vector of bytes to string (Es el inverso de as_bytes())
    // Lo que hace es transformar el 65 en A de acuerdo a la tabla ASCII

     
    loop {
        continue;
    }
}

// 0. Build and run the application (debug build).
//
//    > cargo run --example bare3
//    (or use the vscode build task)
//
// 1. What is the output in the `openocd` (Adapter Output) console?
//
/*
    The output is a serie if prints that are programmed in the code. Concretely:
    bare3
    s = ABCD
    bs = [65, 66, 67, 68]
    iterate over slice
    65,66,67,68,iterate iterate using (raw) indexing
    65,
    66,
    67,
    68,


    a = AAAA
*/
//    What is the type of `s`?
//
/*    
    Using command whatis s we can see the type of the variable
    type = &str
*/
//    What is the type of `bs`?
//
/*   
    type = &[u8]. It is a byte array 
    */
//    What is the type of `c`?

/*    
    c takes the type in bs components so type = u8 *
*/
//    What is the type of `a`?
/*   
    a is an array of integers- > type = [u8;4] 
    Then when it is printed, it prints the ASCII character related to the values of the array
    by using from_utf8(&a)
*/
//    What is the type of `i`? 
/*    
     type = u8 *
*/
//    Commit your answers (bare3_1)
//
// 2. Make types of `s`, `bs`, `c`, `a`, `i` explicit.
/*
   To make variables explicit we declare them with its type
   let s: &str = "ABCD";
   let bs: &[u8] = s.as_bytes();
   let c: u8;
   let a: [u8;4] = [65u8; 4];
   let i: u8;
*/
//    Commit your answers (bare3_2)
//
// 3. Uncomment line `let mut a = [0u8; 4];
//`
//    Run the program, what happens and why?
/*
    First a is defined as an array of 4 elements with value 65 like [65 65 65 65]
    When uncommenting line let mut a = [0u8; 4]; you allow the change of the value
    of variable a and the new array of 4 elements is [0 0 0 0]. Then when you try the conversion
    to string for printing you do not see any value because 0 encodes NULL value in ASCII table
*/
//
//    Commit your answers (bare3_3)
//
// 4. Alter the program so that the data from `bs` is copied byte
//    by byte into `a` using a loop and raw indexing.
//
//    Test that it works as intended.
/*
    let mut a = [0u8; 4];
   for j in 0..bs.len(){
       a[j] = bs[j];
   }
   for c in &a { // For testing that now a has values of 65,66,67,68
    hprint!("{},", c).unwrap();
    }
  // Then with last lines of code it will print the char value according to ASCII table,
  //hence a will be ABCD  
*/
//    Commit your answers (bare3_4)
//
// 5. Look for a way to make this copy done without a loop.
//    https://doc.rust-lang.org/std/primitive.slice.html
//
/*  // Copy the values from bs to a (need to have same dimension)
     let mut a = [0u8;4];
     a.copy_from_slice(&bs);
*/
//    Commit your answers (bare3_5)
//
// 6. Optional
//    Rust is heavily influenced by functional languages.
//    Figure out how you can use an iterator to work over both
//    the `a` and `bs` to copy the content of `bs` to `a`.
//
//    You may use
//    - `iter` (to turn a slice into an iterator)
//    - `zip` (to merge two slices into an iterator)
//    - a for loop to assign the elements
/* //NOT WORKING
     let mut a = [0,0,0,0];
    let a = &mut a[..]; // Take a full slice of `a`
    let  iterator = bs.iter().zip(a.iter());
    for i in 0..5 {
        a[iterator[i]] = bs[iterator[i]];
        
    }
*/
//    Commit your solution (bare3_6)
//
// 7. Optional
//    Iter using `foreach` and a closure instead of the for loop.
//
//    Commit your solution (bare3_7)
//
// 8. Optional*
//    Now benchmark your different solutions using the cycle accurate
//    DWT based approach (in release mode).
//
//    Cycle count for `raw` indexing
//
//    ** your answer here **
//
//    Cycle count for the primitive slice approach.
//
//    ** your answer here **
//
//    Cycle count for the primitive slice approach.
//
//    ** your answer here **
//
//    Cycle count for the zip + for loop approach.
//
//    ** your answer here **
//
//    Cycle count for the zip + for_each approach.
//
//    What conclusions can you draw, does Rust give you zero-cost abstractions?
//
//    ** your answer here **
