use std::fs;

use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;

fn main() {
    let text: &str = "https://nerdetteshsv.com";
    let qr: QrCode = QrCode::encode_text(text, QrCodeEcc::Low).unwrap();

    let svg: String = to_svg_string(&qr, 4);
    print_qr(&qr);
    write_svg(svg)
}

fn write_svg(svg: String) {
    let path = "output.svg";
    fs::write(path, svg);
}

fn to_svg_string(qr: &QrCode, border: i32) -> String {
    assert!(border >= 0, "Border must be non-negative");
    let mut result: String = String::new();
    result += "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
    result += "<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n";
    let dimension = qr
        .size()
        .checked_add(border.checked_mul(2).unwrap())
        .unwrap();
    result += &format!(
		"<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"0 0 {0} {0}\" stroke=\"none\">\n", dimension);
    result += "\t<rect width=\"100%\" height=\"100%\" fill=\"#FFFFFF\"/>\n";
    result += "\t<path d=\"";
    for y in 0..qr.size() {
        for x in 0..qr.size() {
            if qr.get_module(x, y) {
                if x != 0 || y != 0 {
                    result += " ";
                }
                result += &format!("M{},{}h1v1h-1z", x + border, y + border);
            }
        }
    }
    result += "\" fill=\"#ED1E79\"/>\n";
    result += "</svg>\n";
    result
}

fn print_qr(qr: &QrCode) {
	let border: i32 = 4;
	for y in -border .. qr.size() + border {
		for x in -border .. qr.size() + border {
			let c: char = if qr.get_module(x, y) { '█' } else { ' ' };
			print!("{0}{0}", c);
		}
		println!();
	}
	println!();
}
