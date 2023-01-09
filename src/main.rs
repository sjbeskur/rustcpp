use libc;
use std::vec::Vec;
use std::ffi::CStr;

#[link(name = "cppoxide")]
extern {
    //fn triple_input(input: libc::c_int) -> libc::c_int;
    fn do_something_cool(i: libc::c_int, j: libc::c_int) -> libc::c_float;

    // this fails
    //fn add_to_vector(x: libc::c_float) -> std::vec::Vec<libc::c_float>;
    
    fn get_string(i: libc::c_int, j: libc::c_int) -> *const libc::c_char;
}

fn main() {
    let input = 4;
    //let output = unsafe { triple_input(input) };
    //println!("{} * 3 = {}", input, output);


    let cool = unsafe{ do_something_cool(12, 3)} ;
    println!("cool: {}", cool);

    //let v = unsafe { add_to_vector(123.123) }; 
    //println!("{:?}",v);

    let buff = unsafe { get_string(123, 456) }; 
    println!("asdfasdf");
    let c_str: &CStr = unsafe { CStr::from_ptr(buff) };
    println!("asdfasdf");
    let str_slice: &str = c_str.to_str().unwrap();
    println!("asdfasdf");
    let str_buf: String = str_slice.to_owned();     

    println!("{}",str_buf);

}


// extern "C" {
//     fn do_something_cool(i: u32, j: u32) -> f32;
// }

// fn main() {
//     println!("Hello, world!");
//     unsafe{
//      let r = do_something_cool(12, 3);
//      println!("result from cpp: {}", r);
//     }

// }
