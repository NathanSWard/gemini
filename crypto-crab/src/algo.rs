pub trait AlgorithmData {
    type Data;
}

pub trait Algorithm: AlgorithmData {
    fn on(&mut self, data: &Self::Data);
}
