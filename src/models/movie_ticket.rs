use crate::models::movie_screening::MovieScreening;

pub struct MovieTicket {
    row_nr: u32,
    seat_nr: u32,
    is_premium: bool,
    pub movie_screening: MovieScreening,
}

impl MovieTicket {
    pub fn new(
        row_nr: u32,
        seat_nr: u32,
        is_premium: bool,
        movie_screening: MovieScreening,
    ) -> Self {
        Self {
            row_nr,
            seat_nr,
            is_premium,
            movie_screening,
        }
    }
    pub fn get_price(&self) -> f32 {
        self.movie_screening.get_price_per_seat()
    }

    pub fn is_premium_ticket(&self) -> bool {
        self.is_premium
    }
}
