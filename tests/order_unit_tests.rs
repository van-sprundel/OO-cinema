use chrono::{DateTime, TimeZone, Utc};
use cinema_rust::*;

#[test]
fn order_should_return_price() {
    let date = Utc.ymd(2022, 2, 7);

    let movie = Movie::new("Nemo");
    let movie_screening = MovieScreening::new(date, 9.0, movie);
    let mut order = Order::new(1, false, ExportToJSON);

    let movie_ticket = MovieTicket::new(1, 0, false, movie_screening);
    order.add_seat_reservation(movie_ticket);

    assert_eq!(9.0, order.calculate_price());
}

#[test]
fn premium_order_should_return_premium_price() {
    let date = Utc.ymd(2022, 2, 7);

    let movie = Movie::new("Nemo");
    let movie_screening = MovieScreening::new(date, 9.0, movie);
    let mut order = Order::new(1, false, ExportToJSON);

    let movie_ticket = MovieTicket::new(1, 0, true, movie_screening);
    order.add_seat_reservation(movie_ticket);

    assert_eq!(12.0, order.calculate_price());
}

#[test]
fn premium_order_as_student_should_return_price_of_one_seat_plus_two_with_10_percent_discount() {
    let date = Utc.ymd(2022, 2, 7);

    let movie = Movie::new("Nemo");
    let movie_screening = MovieScreening::new(date, 9.0, movie);
    let mut order = Order::new(1, true, ExportToJSON);

    for _ in 0..2 {
        let movie_ticket = MovieTicket::new(1, 0, true, movie_screening.clone());
        order.add_seat_reservation(movie_ticket);
    }

    let calculation = (9.0 + 2.0) * 0.9;
    assert_eq!(calculation, order.calculate_price());
}
