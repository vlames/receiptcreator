// File: pdf.rs
// Description: the file does all the core functionality to print receipts

use std::fs::File;
use std::io::BufReader;
use std::io::{BufRead, BufWriter};
use std::io::Cursor;
use image::bmp::BMPDecoder;
use printpdf::*;

// Defines which line in a .csv file the header is placed
const HEADER_LINE_NUM: usize = 3;
// Defines which line in a .csv file the receipt data starts
const DATA_START_LINE: usize = 4;
// Defines which line in a .csv file the receipt data ends
const DATA_END_LINE: usize = 10;


// Holds the necessary member information
pub struct Member {
    pub first_name: String,
    pub last_name: String,
    pub payment: String,
}

// Holds the pdf page settings
struct PageInfo {
    margin: f64,
    lineoffset: f64,
    layer: PdfLayerReference,
    font: IndirectFontRef,
    font_size: i64,
}

// Holds the position of cursor in the document
struct Position {
    x: f64,
    y: f64,
}



// Gets information to fill from an external file
// Input: a file name
// Output: a list of member information
pub fn get_info(file_name: String) -> Vec<Member> {

    let reader;
    
    // Opens a file for reading or prints an error
    match File::open(&file_name) {
        Ok(name) => reader = BufReader::new(name),
        Err(e) => { 
            println!("Get_info: failed to open {}", file_name); 
            println!("Reason: {}", e); std::process::exit(0); },
    }

    // Gets all lines from the file
    let mut buf: Vec<String> = reader
        .lines()
        .map(|x| x.unwrap().to_string())
        .collect();
    
    // Gets a header line from buffer
    let header = buf[HEADER_LINE_NUM-1].clone();
    // Gets only relevant data lines
    let data = buf.drain(DATA_START_LINE-1..DATA_END_LINE-1).collect::<Vec<String>>();


    // Will hold all member information for receipt printing
    let mut members: Vec<Member> = Vec::new();

    // Fills all the member information into members
    for line in &data {
        let first_name: String = get_data(line, &header, "First Name");
        let last_name: String = get_data(line, &header, "Last Name");
        let payment: String = get_data(line, &header, "Fee").replace("$", "").replace(" ", "");
        let member = Member { first_name, last_name, payment };
        members.push(member);
    }
   
    members
}


// Supplies back the requested information if it exists in the provided data
// Input: data string, header string and the requested str
// Output: requested data or an error wrapped in a string
fn get_data(data: &String, header: &String, request: &str) -> String {
    let index = header.split(",").position(|x| x == request);
    match index {
        Some(i) => data.split(",")
                       .map(|x| x.to_string())
                       .collect::<Vec<String>>()[i].clone(),
        None => "get_data: data not found".to_string(),
    }
}


// Prints a receipt for each member to a pdf file
// Input: Period the receipt is to be written about and members data
// Output: None
pub fn write_receipts(period: String, list: Vec<Member>) {
    
    // Define page dementions
    let width = 215.9;
    let height = 279.4;
    
    // Define page dimention variables
    let xmax = width;
    let xmin = 0.0;
    let ymax = height;
    let ymin = 0.0;
    let xmid = width / 2.0;

    // Define page style variables
    let margin = 7.76;
    let font_size = 18i64;
    let lineoffset = 7.2;

    // Creates a new document with "width" and "height" dimentions
    let (doc, page, layer1) = 
        PdfDocument::new("Reciepts", Mm(width), Mm(height), "Layer 1");
    let layer = doc.get_page(page).get_layer(layer1);
    
    // Defines thickness for all cut lines
    layer.set_outline_thickness(1.0);

    // Brings Times Roman font to the document
    let font = doc.add_builtin_font(BuiltinFont::TimesRoman).unwrap();
    
    // Stores all the necessary page setup information
    let mut page = PageInfo { layer, font, font_size, margin, lineoffset};

    // Adds a horizontal cut line to a receipt
    let mut cords = vec![xmid , ymax-1.0, xmid, ymin+1.0];
    add_line(&cords , &page.layer);
    
    let mut cursor = Position { x: xmin + page.margin, y: ymax };
    let mut count = 0;

    for member in list {

        // Prints a receipt and returns the last coordinates used
        print_receipt(member, &period, &mut cursor, &page);
        
        // Adds a logo to a receipt
        add_logo(&page.layer, cursor.x + 52.0, cursor.y + margin - 1.0);
        
        count += 1;
        
        // Changes the position of the next receipt to be to the
        // right of the middle line
        if count == 4 {
            cursor.y = ymax;
            cursor.x = xmid + page.margin;
        }
        
        // Prints correctly bottom line for a receipt
        // to the left of the mid line
        if count < 4 {
            cords = vec![xmin+1.0, cursor.y, xmid, cursor.y];
            add_line(&cords , &page.layer);
        } 
        // Prints correctly bottom line for a receipt
        // to the right of the mid line
        if count > 4 && count < 8 {
            cords = vec![xmid, cursor.y, xmax-1.0, cursor.y];
            add_line(&cords , &page.layer);
        }

        // Prepares a new page for receipt insertion
        if count % 8 == 0 {
            // Resets receipt number count if page is full
            count = 0;
            // Define page location for the next receipt
            cursor.x = xmin + page.margin;
            cursor.y = ymax;
            // Adds a new page and assigns a layer to add receipts to
            let (pagen, layer1) = doc.add_page(Mm(width), Mm(height), "Layer 1");
            page.layer = doc.get_page(pagen).get_layer(layer1);
            cords = vec![xmid , ymax-1.0, xmid, ymin+1.0];
            // Adds a middle horizontal cut line
            add_line(&cords , &page.layer);
        }
    }
    
    let file = File::create("receipts.pdf").unwrap();
    let buf = &mut BufWriter::new(file);
    doc.save(buf).unwrap();

}

// Adds a line to a page
// Input: coordinates to where to place a line and pdf layer information
// Output: none
fn add_line(cords: &Vec<f64>, layer: &PdfLayerReference) {

    let pts = vec![(Point::new(Mm(cords[0]), Mm(cords[1])), false),
                   (Point::new(Mm(cords[2]), Mm(cords[3])), false)];

    let line = Line {
            points: pts,
            is_closed: true,
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false,
    };
    
    layer.add_shape(line);
    
}

// Adds a logo to a page layer
// Input: pdf layer information and logo coordinates
// Output: none
fn add_logo(layer: &PdfLayerReference, x: f64, y: f64) {

    let image_bytes = include_bytes!("logo.bmp");
    let mut reader = Cursor::new(image_bytes.as_ref());

    let decoder = BMPDecoder::new(&mut reader);
    let logo = Image::try_from(decoder).unwrap();

    // layer, x move, y move, rotate, resize x, resize y, ?
    logo.add_to_layer(layer.clone(), Some(Mm(x)), Some(Mm(y)), None, Some(1.8), Some(1.8), None);

}

// Prints receipt for a member
// Input: member, period and positions to place receipt to
// Output: none
fn print_receipt(member: Member, period: &String, 
                 cursor: &mut Position, page: &PageInfo) {
    
    let mut text: String;
    let x = cursor.x;
    let mut y = cursor.y - page.margin - (page.margin / 2.0);

    // Print Mutual Fund on the first line
    text = "Mutual Fund".to_string();
    page.layer.use_text(text, page.font_size, Mm(x), Mm(y), &page.font);
    y -= page.lineoffset + page.lineoffset;

    // Print receipt period on the second line
    text = "Period: ".to_string();
    text.push_str(&period);
    page.layer.use_text(text, page.font_size, Mm(x), Mm(y), &page.font);
    y -= page.lineoffset;

    // Print member name on the third line
    text = "Name: ".to_string();
    text.push_str(&member.last_name);
    text.push_str(" ");
    text.push_str(&member.first_name);
    page.layer.use_text(text, page.font_size, Mm(x), Mm(y), &page.font);
    y -= page.lineoffset;

    // Print payment information on the fourth line
    text = "Payment: $".to_string();
    text.push_str(&(member.payment).to_string());
    page.layer.use_text(text, page.font_size, Mm(x), Mm(y), &page.font);
    y -= page.lineoffset + page.lineoffset;

    // Print date and signature on the fifth line
    text = "Date:".to_string();
    page.layer.use_text(text, page.font_size, Mm(x), Mm(y), &page.font);
    y -= page.lineoffset;
    
    text = "Signature:".to_string();
    page.layer.use_text(text, page.font_size, Mm(x), Mm(y), &page.font);
    y -= page.margin;
    
    cursor.y = y;

}