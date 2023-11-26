use std::fmt::Write;

use crossterm::event::{KeyCode, KeyEvent};

use crate::{font::foreground, Command, Message, Model, Pager};

pub struct List {
    /// Height of the list.
    ///
    /// This is the space that the list has to render items.
    /// If the list does not have enough items to fill this
    /// space when `view` is called, padding `\r\n` is added.
    height: usize,
    /// Index of the cursor over the current page.
    position: usize,

    /// The text color of the selected item.
    foreground: u8,

    /// Items visible in the list.
    items: Vec<String>,

    /// Nested `Pager` helps with pagination.
    pager: Pager,
}

impl List {
    pub fn new<T>(items: T, height: usize, foreground: u8) -> Self
    where
        T: Iterator<Item = String>,
    {
        let items: Vec<String> = items.collect();
        let items_len = items.len();

        Self {
            height,
            position: 0,
            foreground,
            items,
            pager: Pager::new(0, height, items_len / height),
        }
    }

    /// Return the cursor index over the current page.
    ///
    /// # Warning
    ///
    /// This is not the cursor index within the overall set of items,
    /// for that use `get_overall_index`.
    pub fn get_position(&self) -> usize {
        self.position
    }

    /// Set the height of the list.
    pub fn set_height(&mut self, height: usize) {
        self.height = height;
    }

    /// Move the cursor up.
    ///
    /// This may also adjust the current page, depending on the current position.
    ///
    /// # Examples
    ///
    /// ```
    /// use locket::components::List;
    ///
    /// // Start with `15` items.
    /// let mut items = Vec::new();
    /// for i in 0..15 {
    ///     items.push(format!("{i}"))
    /// }
    /// assert_eq!(items.len(), 15);
    ///
    /// let mut list = List::new(items.into_iter(), 5, 10);
    ///
    /// // Move to the middle of the page.
    /// list.set_position(3);
    ///
    /// assert_eq!(list.get_position(), 3);
    ///
    /// // Move to the first item again.
    /// for i in 0..3 {
    ///     list.handle_up();
    /// }
    ///
    /// assert_eq!(list.get_position(), 0);
    /// assert_eq!(list.get_page(), 0);
    ///
    /// // The component will not wrap around if we are on the first page.
    /// list.handle_up();
    ///
    /// assert_eq!(list.get_position(), 0);
    /// assert_eq!(list.get_page(), 0);
    /// ```
    pub fn handle_up(&mut self) {
        if self.is_on_first_item() {
            // Move to the previous page.
            self.previous();
        } else {
            // Just move the cursor up.
            self.position -= 1;
        }
    }

    /// Move the cursor down.
    ///
    /// This may also adjust the current page, depending on the current position.
    ///
    /// # Examples
    ///
    /// ```
    /// use locket::components::List;
    ///
    /// // Start with `15` items.
    /// let mut items = Vec::new();
    /// for i in 0..15 {
    ///     items.push(format!("{i}"))
    /// }
    /// assert_eq!(items.len(), 15);
    ///
    /// let mut list = List::new(items.into_iter(), 5, 10);
    ///
    /// assert_eq!(list.get_position(), 0);
    ///
    /// // Move to the last item.
    /// for i in 0..4 {
    ///     list.handle_down();
    /// }
    ///
    /// assert_eq!(list.get_position(), 4);
    /// assert_eq!(list.get_page(), 0);
    ///
    /// // The list will move to the next page, if one exists.
    /// list.handle_down();
    ///
    /// assert_eq!(list.get_position(), 0);
    /// assert_eq!(list.get_page(), 1);
    /// ```
    pub fn handle_down(&mut self) {
        if self.is_on_last_item() {
            // Move to the next page.
            self.next();
        } else {
            // Just move the cursor down.
            self.position += 1;
        }
    }

    /// Return the current page number.
    pub fn get_page(&self) -> usize {
        self.pager.get_page()
    }

    /// Return the total amount of pages.
    pub fn get_total(&self) -> usize {
        self.pager.get_total()
    }

    /// Return the number of items displayed per page.
    pub fn get_per(&self) -> usize {
        self.pager.get_per()
    }

    /// Move to the next page and adjust cursor position.
    pub fn next(&mut self) {
        if self.pager.is_on_last_page() {
            return;
        }

        self.pager.next();
        self.position = 0;
    }

    /// Move to the previous page and adjust the cursor position.
    pub fn previous(&mut self) {
        if self.pager.is_on_first_page() {
            return;
        }

        self.pager.previous();

        //
        // Note: Make sure this next call to `get_index_last_item` always happens
        //       after the page is adjusted, or the index might be wrong!
        //

        self.position = self.get_index_last_item();
    }

    /// Return true if the cursor is over the last item on the current page.
    pub fn is_on_last_item(&self) -> bool {
        self.position == self.get_index_last_item()
    }

    /// Return true if the cursor is over the first item on the current page.
    pub fn is_on_first_item(&self) -> bool {
        self.position == 0
    }

    /// Get the index of the last item on the current page.
    pub fn get_index_last_item(&self) -> usize {
        self.pager
            .get_num_on_page(self.items.len())
            .checked_sub(1)
            .unwrap_or(0)
        // self.pager.get_num_on_page(self.items.len()) - 1
    }

    /// Set the position
    ///
    /// # Panics
    ///
    /// The new position must be < `self.pager.get_num_on_page(usize)` to prevent
    /// cursor from going out of bounds.
    pub fn set_position(&mut self, position: usize) {
        assert!(
            position < self.pager.get_num_on_page(self.items.len()),
            "position must be < items on page or cursor will be out of bounds"
        );

        self.position = position;
    }

    /// Return the index of the selected item as it appears in the overall
    /// set of items.
    ///
    /// # Examples
    ///
    /// ```
    /// use locket::components::List;
    ///
    /// // We have `14` total items.
    /// let mut items = Vec::new();
    /// for i in 0..14 {
    ///     items.push(format!("{i}"))
    /// }
    /// assert_eq!(items.len(), 14);
    ///
    /// let mut list = List::new(items.into_iter(), 5, 10);
    ///
    /// // We start on the first item.
    /// assert_eq!(list.get_overall_position(), 0);
    ///
    /// assert_eq!(list.get_per(), 5);
    /// assert_eq!(list.get_page(), 0);
    ///
    /// // Move two pages forward, and two items down.
    /// for _ in 0..list.get_per() {
    ///     list.handle_down();
    /// }
    ///
    /// assert_eq!(list.get_page(), 1);
    ///
    /// list.handle_down();
    /// list.handle_down();
    ///
    /// assert_eq!(list.get_overall_position(), 7);
    /// ```
    pub fn get_overall_position(&self) -> usize {
        self.pager.get_page() * self.pager.get_per() + self.position
    }
}

impl Model for List {
    fn update(&mut self, message: &Message) -> Option<Command> {
        // Handle `KeyEvent`.
        if let Some(event) = message.downcast_ref::<KeyEvent>() {
            match event.code {
                // Movement.
                KeyCode::Up => self.handle_up(),
                KeyCode::Down => self.handle_down(),

                // No action.
                _ => {}
            }
        };

        //
        // On `ResizeEvent`:
        //
        // This component is likely to be used alongside other components.
        // It is the parent model's job to keep the height of the list in
        // check via the `set_height` method.
        //

        None
    }

    fn view(&self) -> String {
        // The component must render the "flexible" part of itself with as much height
        // as remains after considering the other fixed children of the component.
        //
        // This may include a status/title bar or any other informational messages that
        // are supported by the component.
        //
        // With the height of all fixed components in hand as n, the remaining height is
        // (`self.height` - `n`).
        //
        // No child components are supported by list right now, so use `self.height`.
        let available_height = self.height;

        // Based on the pager state, what items should be visible?
        let (start, end) = self.pager.get_bounds(self.items.len());
        // 30:15? TODO

        let selected = self
            .items
            .get(start..end)
            .expect("get_bounds should never return out of bounds indices");

        let mut buffer = String::new();

        for (index, item) in selected.into_iter().enumerate() {
            let string = if self.position == index {
                format!("{}", foreground(item, self.foreground))
            } else {
                format!("{item}")
            };
            write!(buffer, "{string}").unwrap();

            // Add a newline, unless this is the last item.
            if index != selected.len() - 1 {
                write!(buffer, "\r\n").unwrap();
            }
        }

        // Now we should consume any remaining height.
        let buffer_height = buffer.lines().count();
        let remaining_height = available_height - buffer_height;

        let padding: String = std::iter::repeat("\r\n").take(remaining_height).collect();

        format!("{buffer}{padding}")
    }
}
