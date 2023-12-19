// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![create_svg])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command(rename_all = "snake_case")]
fn create_svg(url: String, color: String) -> String {
  let qr = QrCode::encode_text(&url, QrCodeEcc::Medium).unwrap();
  let svg = to_svg_string(&qr, 4, color);
  svg.into()
}

fn to_svg_string(qr: &QrCode, border: i32, color: String) -> String {
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
  result += "\t<rect width=\"100%\" height=\"100%\" fill=\"none\"/>\n";
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
  result += "\" fill=\"";
  result += &color;
  result += "\"/>\n";
  result += "</svg>\n";
  result
}
