use qrcode_generator::QrCodeEcc;

pub fn qr_code(value: &str, path: &str)
{
    qrcode_generator::to_png_to_file(value, QrCodeEcc::Low, 128, path).unwrap();
}

pub fn qr_code_save(value: &str, path: &str)
{
    qrcode_generator::to_png_to_file(value, QrCodeEcc::Low, 512, path).unwrap();
}