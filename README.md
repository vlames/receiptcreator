# ReceiptCreator

The receiptcreator is a simple application that allows a user to create a list of receipts written to a .pdf file.

## Requirements

* Terminal
* [Rust](https://www.rust-lang.org) programming language

## How to build and run

* Clone the repository
* Using terminal, change into the ```receiptcreator/src``` directory
* Type ```cargo build``` to build the application
* Type ```cargo run``` to run the application
* Open the ```receipts.pdf``` file to view the created receipts 

## Important

* If you reposition header, add/remove data lines in a .csv file, edit the variables at the top of the ```pdf.rs``` file accordingly
* First Name, Last Name, and Fee header fields must be present in the .csv file

## Credits

* [Printpdf](https://docs.rs/printpdf/0.3.2/printpdf/) and [Rust](https://doc.rust-lang.org/std/) documentation