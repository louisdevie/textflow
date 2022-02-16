extern crate textflow;

use textflow::columns;
use textflow::Alignment::*;
use textflow::Layout;
use textflow::Spacing::*;

fn main() {
    let text1 = "First";
    let text2 = "Second";
    let text3 = "Third";
    let text4 = "Fourth";

    println!("BASIC ==========================================\n");
    println!(
        "{}",
        columns([text1, text2, text3], BETWEEN, Layout::default(), 48)
    );

    println!("\n\nMIRRORED =======================================\n");
    println!(
        "{}",
        columns([text1, text2], BETWEEN, Layout::from_pattern("> <"), 48)
    );

    println!("\n\nFIXED WIDTH COLUMN =============================\n");
    println!(
        "{}",
        columns(
            [text1, text2, text3],
            AROUND,
            Layout::from_pattern("20 *"),
            48
        )
    );

    println!("\n\nCOMPLEX PATTERN ==================================\n");
    println!(
        "{}",
        columns(
            [text1, text2, text3, text4],
            AROUND,
            Layout::from_pattern("=- ^15* >--"),
            48
        )
    );

    let my_layout = Layout::new()
        .fractional(1, JUSTIFY)
        .fixed(15, CENTER)
        .repeat()
        .fractional(2, RIGHT);
    println!("\n\nSAME LAYOUT, DIFFERENT CONSTRUCTOR =============\n");
    println!(
        "{}",
        columns([text1, text2, text3, text4], AROUND, my_layout, 48)
    );
}
