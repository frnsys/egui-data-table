use std::{collections::VecDeque, sync::atomic::AtomicU64};

pub mod draw;
pub mod viewer;

pub use draw::Renderer;
pub use viewer::{RowViewer, UiAction};

/* ---------------------------------------------------------------------------------------------- */
/*                                           CORE CLASS                                           */
/* ---------------------------------------------------------------------------------------------- */

/// Prevents direct modification of `Vec`
pub struct Spreadsheet<R> {
    /// Efficient row data storage
    rows: VecDeque<R>,

    /// Is Dirty?
    dirty_flag: bool,

    /// Ui
    ui: Option<Box<draw::state::UiState<R>>>,
}

impl<R: std::fmt::Debug> std::fmt::Debug for Spreadsheet<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Spreadsheet")
            .field("rows", &self.rows)
            .finish()
    }
}

impl<R> Default for Spreadsheet<R> {
    fn default() -> Self {
        Self {
            rows: Default::default(),
            ui: Default::default(),
            dirty_flag: false,
        }
    }
}

impl<R> FromIterator<R> for Spreadsheet<R> {
    fn from_iter<T: IntoIterator<Item = R>>(iter: T) -> Self {
        Self {
            rows: iter.into_iter().collect(),
            ..Default::default()
        }
    }
}

impl<R> Spreadsheet<R> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &R> {
        self.rows.iter()
    }

    pub fn take(&mut self) -> VecDeque<R> {
        std::mem::take(&mut self.rows)
    }

    pub fn replace(&mut self, new: VecDeque<R>) -> VecDeque<R> {
        std::mem::replace(&mut self.rows, new)
    }

    pub fn retain(&mut self, mut f: impl FnMut(&R) -> bool) {
        let mut removed_any = false;
        self.rows.retain(|row| {
            let retain = f(row);
            removed_any |= !retain;
            retain
        });

        if removed_any {
            self.ui = None;
        }
    }

    pub fn clear_dirty_flag(&mut self) {
        self.dirty_flag = false;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty_flag
    }

    pub fn has_user_modification(&self) -> bool {
        self.ui.as_ref().is_some_and(|x| todo!())
    }

    pub fn clear_user_modification_flag(&mut self) {
        todo!()
    }
}

impl<R> Extend<R> for Spreadsheet<R> {
    /// Programmatic extend operation will invalidate the index table cache.
    fn extend<T: IntoIterator<Item = R>>(&mut self, iter: T) {
        // Invalidate the cache
        self.ui = None;
        self.rows.extend(iter);
    }
}

fn default<T: Default>() -> T {
    T::default()
}
