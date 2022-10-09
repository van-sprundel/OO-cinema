use crate::models::export::ExportBehavior;
use crate::models::Order;

pub trait OrderState<T: ExportBehavior> {
    fn calculate(self: Box<Self>, order: Order<T>) -> f32;
}

pub struct CreatedState;

impl<T: ExportBehavior> OrderState<T> for CreatedState {
    fn calculate(self: Box<Self>, order: Order<T>) -> f32 {
        order.calculate_price()
    }
}
