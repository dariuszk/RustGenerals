

#[derive(Clone, Serialize, Deserialize)]
pub struct BoardSize {
    pub rows_numbers: u32,
    pub columns_numbers: u32,
    pub field_size: u32,
}
impl BoardSize {
    pub fn new(rows_numbers: u32, columns_numbers: u32, field_size : u32) -> BoardSize { BoardSize {
        rows_numbers,
        columns_numbers,
        field_size,
    } }

    pub fn get_width(&mut self) -> u32 { self.field_size * self.rows_numbers }
    pub fn get_height(&mut self) -> u32 { self.field_size * self.columns_numbers }
}
