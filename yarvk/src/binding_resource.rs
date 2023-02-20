pub trait BindingResource: Send + Sync {
    type RawTy;
    fn raw(&self) -> &Self::RawTy;
    fn raw_mut(&mut self) -> &mut Self::RawTy;
}