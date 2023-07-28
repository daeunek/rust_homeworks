// Convert Celsius to Fah
pub fn fah_to_cel(fahr: i32) -> f32 {
    (5.0 / 9.0) * (fahr as f32 - 32.0)
}