use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let greeting = "नमस्ते";

    
    for b in greeting.bytes() {
        println!("{}", b);
    }

    for c in greeting.chars() {
        println!("{}", c);
    }

    for g in greeting.graphemes(true) {
        println!("{}", g);
    }
}
