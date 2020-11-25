use crate::expression::Selectable;
use crate::query_source::Table;

/// The `select_by` method
///
/// This trait should not be relied on directly by most apps. Its behavior is
/// provided by [`QueryDsl`]. However, you may need a where clause on this trait
/// to call `select_by` from generic code.
///
/// [`QueryDsl`]: ../trait.QueryDsl.html
pub trait SelectByDsl<Selection: Selectable> {
    // FIXME: Once we've refactored the `impl Expression` on `SelectStatement`
    // to not conditionally be `sql_types::Array`, it is probably worthwhile to
    // add a `: Expression<SqlType = Selection::SqlType>` bound here.
    /// The type returned by `.select_by`
    type Output;

    /// See the trait documentation
    fn select_by(self) -> Self::Output;
}

impl<T, Selection> SelectByDsl<Selection> for T
where
    Selection: Selectable,
    T: Table,
    T::Query: SelectByDsl<Selection>,
{
    type Output = <T::Query as SelectByDsl<Selection>>::Output;

    fn select_by(self) -> Self::Output {
        self.as_query().select_by()
    }
}