use crate::models::movie::Movie;
use chrono::{Date, Datelike, Utc};

#[derive(Clone)]
pub struct MovieScreening {
    on_date: Date<Utc>,
    price_per_seat: f32,
    movie: Movie,
}

impl MovieScreening {
    pub fn new(on_date: Date<Utc>, price_per_seat: f32, movie: Movie) -> Self {
        Self {
            on_date,
            price_per_seat,
            movie,
        }
    }
    pub fn get_price_per_seat(&self) -> f32 {
        self.price_per_seat
    }

    pub fn get_day_of_week(&self) -> u32 {
        self.on_date.weekday().num_days_from_monday()
    }
}
