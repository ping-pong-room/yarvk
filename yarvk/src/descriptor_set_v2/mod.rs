pub mod descriptor_set;
pub mod descriptor_type;
pub mod desccriptor_pool;
pub(crate) mod private;
pub mod descriptor_set_layout;
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
        if &old[i] == &new[i] {
            if start != None {
                end = Some(i);
            }
        } else {
            if start == None {
                start = Some(i);
            }
        }
        if end != None {
            f(start.unwrap(), end.unwrap());
            start = None;
            end = None;
        }
        i = i + 1;
    }
    if start != None {
        f(start.unwrap(), I);
    }
}
