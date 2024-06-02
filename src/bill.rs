#[derive(Default, Debug)]
pub struct Bill {
    pub id: i32,
    pub date: String,
    pub description: String,
    pub notes: String,
    pub amount: f64,
    pub timestamp: String,
    pub allocated: bool,
    pub sent: bool,
    pub paid: bool,
}
