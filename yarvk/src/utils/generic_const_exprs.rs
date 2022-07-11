pub struct Boolean<const B: bool>;

pub trait ConstWhere {}

impl ConstWhere for Boolean<true> {}
