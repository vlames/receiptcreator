// File: main.rs
// Description: the file provides the overall program flow

mod pdf;
extern crate printpdf;
use std::io::*;
use pdf::*;

fn main() {
    
    // Prints Greeting
    println!("\nWellcome to Receipt Creator!");

    let reader = stdin();
   
    let mut file_name = String::new();
    print!("Data file: ");
    stdout().flush().unwrap();
    reader.read_line(&mut file_name).unwrap();
    file_name = file_name.trim().to_string();

    // Gets all receipt info from a file
    let info = get_info(file_name);

    // Gets the payment period
    let mut period = String::new();
    print!("Enter period: ");
    stdout().flush().unwrap();
    reader.read_line(&mut period).unwrap();
    period = period.trim().to_string();

    // Request to wait for the work to be done
    println!("Please, wait!");

    // Writes out all the receipts
    write_receipts(period, info);

    // Signifies completion of work
    println!("Work is Done! Exiting ...\n");

}