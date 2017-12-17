extern crate serde_json;

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

    pub fn get_width(&self) -> f64 { (self.field_size * self.rows_numbers) as f64 }
    pub fn get_height(&self) -> f64 { (self.field_size * self.columns_numbers) as f64 }
    pub fn get_all_fields(&self) -> usize { (self.columns_numbers * self.rows_numbers) as usize }
    pub fn get_field_size(&self) -> f64 { self.field_size as f64 }
}



#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub title: String,
    pub board_size: BoardSize,
}


impl Default for Config {
    fn default() -> Config {

        Config {
            title: "Rust generals".to_owned(),
            board_size: BoardSize {
                rows_numbers: 10,
                columns_numbers: 10,
                field_size: 50,
            },
        }
    }
}