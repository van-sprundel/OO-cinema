use crate::models::calculate_behavior::{CalculateBehavior, NormalCalculate, StudentCalculate};
use crate::models::export::ExportBehavior;
use crate::models::order_state::{CreatedState, OrderState};
use crate::models::MovieTicket;

pub struct Order<T: ExportBehavior> {
    order_nr: u32,
    calculate_behavior: Box<dyn CalculateBehavior>,
    export_behavior: T,
    movie_tickets: Vec<MovieTicket>,
    paid: bool,
    order_state: Box<dyn OrderState<T>>,
    // observers:Vec<Observer>
}

impl<T: ExportBehavior> Order<T> {
    pub fn new(order_nr: u32, is_student_order: bool, export_behavior: T) -> Self {
        Self {
            order_nr,
            export_behavior,
            movie_tickets: vec![],
            paid: false,
            calculate_behavior: if is_student_order {
                Box::new(StudentCalculate)
            } else {
                Box::new(NormalCalculate)
            },
            order_state: Box::new(CreatedState),
        }
    }

    pub fn get_order_number(&self) -> u32 {
        self.order_nr
    }

    pub fn add_seat_reservation(&mut self, ticket: MovieTicket) {
        self.movie_tickets.push(ticket);
    }

    pub fn calculate_price(&self) -> f32 {
        self.calculate_behavior.calculate_price(&self.movie_tickets)
    }

    pub fn set_export_behavior(&mut self, export_behavior: T) {
        self.export_behavior = export_behavior;
    }

    pub fn get_export_behavior(&self) -> &T {
        &self.export_behavior
    }
}
