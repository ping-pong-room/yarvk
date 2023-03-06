pub mod desccriptor_pool;
pub mod descriptor_set;
pub mod descriptor_set_layout;
pub mod descriptor_type;
pub mod descriptor_variadic_generics;
pub(crate) mod private;
mod private_descriptor_variadic_generics;
pub mod write_descriptor_set;

pub(crate) fn diff<T: PartialEq, const I: usize>(
    old: &[T; I],
    new: &[T; I],
    mut f: impl FnMut(usize, usize),
) {
    let mut i = 0;
    let mut start = None;
    let mut end = None;
    while i < I {
        if old[i] == new[i] {
            if start.is_some() {
                end = Some(i);
            }
        } else if start.is_none() {
            start = Some(i);
        }
        if end.is_some() {
            f(start.unwrap(), end.unwrap());
            start = None;
            end = None;
        }
        i += 1;
    }
    if let Some(start) = start {
        f(start, I);
    }
}
