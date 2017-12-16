


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
                field_size: 10,
            },
        }
    }
}