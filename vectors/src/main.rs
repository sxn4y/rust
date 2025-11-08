fn main() {
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec! [
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];

    match &row[0] {
        SpreadSheetCell::Int(value) => println!("Integer: {}", value),
        _ => println!("Not an integer"),
    }
}
