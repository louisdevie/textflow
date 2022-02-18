extern crate textflow;

use textflow::columns;
use textflow::Alignment::*;
use textflow::Layout;
use textflow::Spacing::*;

fn main() -> Result<(), String> {
    let text1 = "The November sky above was blue.";
    let text2 =
        "It was mid-November, the time when the season of changing leaves was drawing to a close;";
    let text3 =
        "midday was wrapped in a pleasant coolness and breakfast brought adopted a sharp chill.";
    let roles = "Tōma Kamijō\nAccelerator\nMikoto Misaka";
    let actors = "Atsushi Abe\nNobuhiko Okamoto\nRina Satō";

    println!("BASIC ======================================================\n");
    println!(
        "{}",
        columns([text1, text2, text3], BETWEEN, Layout::default(), 60)
    );

    println!("\n\nMIRRORED =================================================\n");
    println!(
        "{}",
        columns([roles, actors], BETWEEN, Layout::from_pattern("> <")?, 60)
    );

    println!("\n\nFIXED WIDTH COLUMN =========================================\n");
    println!(
        "{}",
        columns(
            [text1, text2, text3],
            AROUND,
            Layout::from_pattern("16 *")?,
            60
        )
    );

    println!("\n\nCOMPLEX PATTERN ============================================\n");
    println!(
        "{}",
        columns(
            [text1, text2, text3],
            BETWEEN,
            Layout::from_pattern("=- ^15* >--")?,
            60
        )
    );

    let my_layout = Layout::new()
        .fractional(1, JUSTIFY)
        .fixed(15, CENTER)
        .repeat()
        .fractional(2, RIGHT);
    println!("\n\nSAME LAYOUT, DIFFERENT CONSTRUCTOR =========================\n");
    println!("{}", columns([text1, text2, text3], BETWEEN, my_layout, 60));

    Ok(())
}
