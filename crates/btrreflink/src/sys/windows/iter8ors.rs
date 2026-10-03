use core::{iter::from_fn, ops::AddAssign};

/// Iterator that yields an infinite sequence of the previous stored value and
/// it's mutation
#[inline]
fn iter_stored<T: Clone, U>(
    mut store: T,
    mut mod_fn: impl FnMut(&mut T) -> U,
) -> impl Iterator<Item = (T, U)> {
    let mut f = move |x: &mut T| (x.clone(), mod_fn(x));
    from_fn(move || Some(f(&mut store)))
}

#[inline]
pub(super) fn iter_add<T: Clone + AddAssign<U>, U: Clone>(
    store: T,
    mut add_fn: impl FnMut(&T) -> U,
) -> impl Iterator<Item = (T, U)> {
    iter_stored(store, move |x: &mut T| {
        let v = add_fn(x);
        *x += v.clone();
        v
    })
}
