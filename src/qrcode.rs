use qrcode_generator::QrCodeEcc;

pub fn qr_code(value: &str)
{
    qrcode_generator::to_png_to_file(value, QrCodeEcc::Low, 512, "file.png").unwrap();
}