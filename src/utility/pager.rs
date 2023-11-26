pub struct Pager {
    /// The current page number.
    ///
    /// Usually set to 0 to start at the beginning.
    current: usize,
    /// Amount of items per page.
    per: usize,
    /// Total amount of pages.
    ///
    /// May be given a calculated value from the beginning,
    /// or initialized with a faux value and updated using `.set_total_by_len`.
    total: usize,
}

impl Default for Pager {
    fn default() -> Self {
        Self {
            current: 0,
            per: 1,
            total: 1,
        }
    }
}

impl Pager {
    /// Return a new instance of `Pager`.
    ///
    /// Generally, `current` is 0 to begin on the first page.
    /// The `per` parameter can be however many items you want to display per page.
    ///
    /// # Examples
    ///
    /// ```
    /// use locket::Pager;
    ///
    /// let items = 15;
    ///
    /// let per = 5;
    /// let total = items / per;
    ///
    /// let pager = Pager::new(0, per, total);
    ///
    /// assert_eq!(pager.get_page(), 0);
    /// assert_eq!(pager.get_per(), 5);
    /// assert_eq!(pager.get_total(), 3);
    /// ```
    pub fn new(current: usize, per: usize, total: usize) -> Self {
        Self {
            current,
            per,
            total,
        }
    }

    /// Get the amount of items displayed per page.
    ///
    /// # Examples
    ///
    /// ```
    /// use locket::Pager;
    ///
    /// let pager = Pager::new(0, 10, 10);
    ///
    /// assert_eq!(pager.get_per(), 10);
    /// ```
    pub fn get_per(&self) -> usize {
        self.per
    }

    /// Set the amount of items displayed per page.
    ///
    /// # Examples
    ///
    /// ```
    /// use locket::Pager;
    ///
    /// let mut pager = Pager::new(0, 10, 10);
    ///
    /// assert_eq!(pager.get_per(), 10);
    ///
    /// pager.set_per(5);
    ///
    /// assert_eq!(pager.get_per(), 5);
    /// ```
    pub fn set_per(&mut self, per: usize) {
        self.per = per;
    }

    /// Return the current page number.
    ///
    /// # Examples
    ///
    /// ```
    /// use locket::Pager;
    ///
    /// // Start on the first page,
    /// // which is 0 because the count is zero-indexed.
    /// let mut pager = Pager::new(0, 10, 10);
    ///
    /// assert_eq!(pager.get_page(), 0);
    ///
    /// // Move forward one page.
    /// pager.next();
    ///
    /// assert_eq!(pager.get_page(), 1);
    /// ```
    pub fn get_page(&self) -> usize {
        self.current
    }

    /// Set the current page number.
    ///
    /// # Examples
    ///
    /// ```
    /// use locket::Pager;
    ///
    /// // Start on the first page,
    /// // which is 0 because the count is zero-indexed.
    /// let mut pager = Pager::new(0, 10, 10);
    ///
    /// assert_eq!(pager.get_page(), 0);
    ///
    /// // Move to the third page.
    /// pager.set_current(2);
    ///
    /// assert_eq!(pager.get_page(), 2);
    /// ```
    pub fn set_current(&mut self, page: usize) {
        self.current = page;
    }

    /// Return the total pages.
    ///
    /// This number is (`n` / `self.per`) where `n` is the last number
    /// passed to `.set_total_by_len` or passed in at creation.
    ///
    /// # Examples
    ///
    /// ```
    /// use locket::Pager;
    ///
    /// let items = 100; // 100 distinct items to display
    ///
    /// let mut pager = Pager::default();
    /// pager.set_per(10); // set `per` first, as it effects next calculation:
    /// pager.set_total_by_len(items);
    ///
    /// let total_pages = pager.get_total();
    /// assert_eq!(total_pages, 10);
    /// ```
    ///
    /// You do not have to use `::default` if you already know the dimensions
    /// of the pager.
    ///
    /// ```
    /// use locket::Pager;
    ///
    /// let items = 100;
    /// let mut pager = Pager::new(0, 10, 10);
    ///
    /// let total_pages = pager.get_total();
    /// assert_eq!(total_pages, 10);
    /// ```
    pub fn get_total(&self) -> usize {
        self.total
    }

    /// Adjust the total number of pages by dividing the length by `self.per`.
    ///
    /// Returns the new `self.total` value.
    ///
    /// # Examples
    ///
    /// ```
    /// use locket::Pager;
    ///
    /// let mut pager = Pager::new(0, 5, 20);
    ///
    /// let total_pages = pager.get_total();
    /// assert_eq!(total_pages, 20);
    /// ```
    pub fn set_total_by_len(&mut self, len: usize) -> usize {
        if len < 1 {
            return self.total;
        }

        let mut n = len / self.per;
        if len % self.per > 0 {
            n += 1;
        }
        self.total = n;

        n
    }

    /// Return the total number of items on the current page by passing the
    /// given length to `.get_bounds`.
    ///
    /// # Examples
    ///
    /// ```
    /// use locket::Pager;
    ///
    /// // Start out with 25 items.
    /// let items = 25;
    ///
    /// let mut pager = Pager::default();
    /// pager.set_per(5);
    /// pager.set_total_by_len(25);
    ///
    /// // Move to the last page.
    /// pager.next();
    /// pager.next();
    /// pager.next();
    /// pager.next();
    /// pager.next();
    ///
    /// // At this point, maybe two of the items were deleted,
    /// // so we have 23. How many are on the page?
    /// let total_on_page = pager.get_num_on_page(23);
    ///
    /// assert_eq!(total_on_page, 3);
    /// ```
    pub fn get_num_on_page(&self, items: usize) -> usize {
        if items < 1 {
            return 0;
        }

        let (start, end) = self.get_bounds(items);

        end - start
    }

    /// Returns (start, end) bounds.
    ///
    /// Return the (start, end) bounds of the items that should be in view,
    /// based on the pager state.
    ///
    /// # Examples
    ///
    /// ```
    /// use locket::Pager;
    ///
    /// let mut pager = Pager::new(0, 10, 10);
    ///
    /// // Move forward two pages.
    /// pager.next();
    /// pager.next();
    ///
    /// // If we have 622 items, based on the above pager,
    /// // which are currently visible?
    ///
    /// let visible_items = pager.get_bounds(622);
    /// assert_eq!(visible_items,  (20, 30));
    /// ```
    pub fn get_bounds(&self, len: usize) -> (usize, usize) {
        let start = self.current * self.per;
        let temp = self.current * self.per + self.per;

        let end = temp.min(len);

        (start, end)
    }

    /// Move to the previous page.
    ///
    /// # Examples
    ///
    /// ```
    /// use locket::Pager;
    ///
    /// // Start on the last page.
    /// // This is `4` rather than `5`, because the count is zero-indexed.
    /// let mut pager = Pager::new(4, 5, 5);
    ///
    /// assert_eq!(pager.get_page(), 4);
    ///
    /// // Pager will not wrap beyond first page.
    /// for _ in 0..=10 {
    ///     pager.previous();
    /// }
    ///
    /// assert_eq!(pager.get_page(), 0);
    /// ```
    pub fn previous(&mut self) {
        match self.current {
            _ if self.current > 0 => {
                self.current -= 1;
            }
            _ => {}
        }
    }

    /// Move to the next page.
    ///
    /// # Examples
    ///
    /// ```
    /// use locket::Pager;
    ///
    /// let mut pager = Pager::new(0, 5, 5);
    ///
    /// assert_eq!(pager.get_page(), 0);
    ///
    /// // Pager will not wrap beyond last page.
    /// for _ in 0..=10 {
    ///     pager.next();
    /// }
    ///
    /// // We have `5` total pages, but the count is zero-indexed,
    /// // so 4 is the last page.
    /// assert_eq!(pager.get_page(), 4);
    /// ```
    pub fn next(&mut self) {
        match self.current {
            _ if !self.is_on_last_page() => {
                self.current += 1;
            }
            _ => {}
        }
    }

    /// Return true if the pager is on the last page.
    ///
    /// # Examples
    ///
    /// ```
    /// use locket::Pager;
    ///
    /// let mut pager = Pager::new(0, 5, 5);
    ///
    /// // Move forward two pages.
    /// pager.next();
    /// pager.next();
    ///
    /// // 5 total pages -- so no.
    /// assert_eq!(pager.is_on_last_page(), false);
    ///
    /// // Move to the last page.
    /// pager.next();
    /// pager.next();
    /// pager.next();
    ///
    /// assert_eq!(pager.is_on_last_page(), true);
    /// ```
    pub fn is_on_last_page(&self) -> bool {
        self.current == self.total.checked_sub(1).unwrap_or(0)
    }

    /// Return true if the pager is on the first page.
    ///
    /// # Examples
    ///
    /// ```
    /// use locket::Pager;
    ///
    /// let mut pager = Pager::new(0, 5, 5);
    ///
    /// // We passed `0` for the first argument above,
    /// // so pager starts on the first page.
    /// assert_eq!(pager.is_on_first_page(), true);
    ///
    /// // Move forward one page.
    /// pager.next();
    ///
    /// assert_eq!(pager.is_on_first_page(), false);
    /// ```
    pub fn is_on_first_page(&self) -> bool {
        self.current == 0
    }
}

#[cfg(test)]
mod tests {
    use crate::utility::pager::Pager;

    #[test]
    fn test_get_bounds() {
        let length = 100;
        let per = 10;
        let mut pager = Pager::new(0, 1, 1);

        pager.set_per(per);
        pager.set_total_by_len(length);

        // println!("{}", pager.get_page());
        assert_eq!(pager.get_page(), 0);
        // println!("{}", pager.get_per());
        assert_eq!(pager.get_per(), 10);
        // println!("{}", pager.get_total());
        assert_eq!(pager.get_total(), 10); // length / per

        // Pager should stay put when it gets to the last page.
        for _ in 0..=1000 {
            pager.next(); // TODO: Update test when infinite scroll is in.
        }

        // println!("{}", pager.get_page());
        assert_eq!(pager.get_page(), 9);
        // println!("{:?}", bounds);
        assert_eq!(pager.get_bounds(length), (90, 100));
    }
}
