use crate::models::MovieTicket;

pub trait CalculateBehavior {
    fn calculate_price(&self, tickets: &[MovieTicket]) -> f32;
}

pub struct StudentCalculate;

pub struct NormalCalculate;

impl CalculateBehavior for StudentCalculate {
    fn calculate_price(&self, tickets: &[MovieTicket]) -> f32 {
        tickets
            .iter()
            .enumerate()
            .fold(0., |mut price, (i, ticket)| {
                let mut ticket_price = ticket.get_price();
                if ticket.is_premium_ticket() {
                    ticket_price += 2.0;
                }

                let day_of_week = ticket.movie_screening.get_day_of_week();

                if day_of_week >= 5 {
                    ticket_price *= 0.9;
                }
                if i != 1 {
                    price += ticket_price;
                } else {
                    println!("Second ticket for free!");
                }
                price
            })
            * 0.9
    }
}

impl CalculateBehavior for NormalCalculate {
   fn calculate_price(&self, tickets: &[MovieTicket]) -> f32 {
        tickets
            .iter()
            .enumerate()
            .fold(0., |mut price, (i, ticket)| {
                let mut ticket_price = ticket.get_price();
                if ticket.is_premium_ticket() {
                    ticket_price += 3.0;
                }
                let day_of_week = ticket.movie_screening.get_day_of_week();
                if day_of_week > 5 && tickets.len() >= 6 {
                    ticket_price *= 0.9;
                }
                if i != 1 || day_of_week > 4 {
                    price += ticket_price;
                } else {
                    println!("Second ticket for free!");
                }
                price
            })
    }
}
