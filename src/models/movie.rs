use crate::models::movie_screening::MovieScreening;

#[derive(Clone)]
pub struct Movie {
    title: String,
    movie_screenings: Vec<MovieScreening>,
}

impl Movie {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_owned(),
            movie_screenings: vec![],
        }
    }
    pub fn add_screening(&mut self, movie_screening: MovieScreening) {
        self.movie_screenings.push(movie_screening);
    }
}
