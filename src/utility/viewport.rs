use std::{collections::HashMap, iter::repeat};

/// A set of upper and lower bounds.
#[derive(Debug)]
pub struct Bound {
    /// The index of the upper bound.
    pub upper: usize,
    /// The index of the lower bound.
    ///
    /// The lower bound will be `None` if the chunk that this bound
    /// represents had only one item.
    pub lower: Option<usize>,
}

impl Bound {
    /// Return a new instance of [`Bound`].
    pub fn new(upper: usize, lower: Option<usize>) -> Self {
        Self { upper, lower }
    }
}

/// A map of y-index position -> Bound.
pub type BoundMap = HashMap<usize, Bound>;

/// Facilities for vertically paginating a view.
pub struct Viewport {
    /// Viewport height.
    height: u16,
    /// Amount of rows to scroll per action.
    scroll_by: u16,

    /// Vertical scroll position.
    /// This is the distance (in lines) from the top of the viewport.
    y: u16,
}

impl Viewport {
    /// Create a new instance of `Viewport`.
    pub fn new(height: u16, scroll_by: u16) -> Self {
        Self {
            height,
            scroll_by,
            y: 0,
        }
    }

    /// Render data and return a string.
    pub fn render<T>(&self, data: T) -> String
    where
        T: AsRef<str>,
    {
        let data = data.as_ref();
        let has_ending_nl = data.ends_with("\n");

        let mut window: Vec<String> = data
            .lines()
            .skip(self.y as usize) // Skip the offset value.
            .take(self.height as usize) // Take a maximum of whatever the viewport height is.
            .map(|n| format!("{n}\r\n")) // `.lines` will remove the line endings, so put those back on..
            .collect(); // Return it as a string.
                        // Strip last line ending if needed.
        if !has_ending_nl {
            if let Some(last) = window.last_mut() {
                *last = last.strip_suffix("\r\n").unwrap().to_string();
            }
        }

        let line_len = window.len();
        window
            .into_iter()
            .chain(
                // The lines in the returned string should be equal to `self.height`.
                // If it is not, pad it with newlines.
                repeat("\r\n")
                    .take(self.height as usize - line_len)
                    .map(|n| n.to_string()),
            )
            .collect()
    }

    /// Return the [`Viewport`] height.
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Set the [`Viewport`] height.
    pub fn set_height(&mut self, height: u16) {
        self.height = height;
    }

    /// Return the [`Viewport`] `y` position
    pub fn y(&self) -> u16 {
        self.y
    }

    /// Return a [`BoundMap`] for a set of items.
    pub fn bounds(&self, len: usize) -> BoundMap {
        let mut result = BoundMap::new();
        let mut y = 0;

        loop {
            let remaining = (0..len).skip(y).count();

            if remaining == 1 {
                result.insert(y, Bound::new(y, None));
            } else if remaining > 1 {
                let height = self.height as usize;

                if remaining > height {
                    result.insert(y, Bound::new(y, Some(y + height - 1)));
                } else {
                    result.insert(y, Bound::new(y, Some(y + remaining - 1)));
                    break;
                }
            } else {
                break;
            }

            y += self.scroll_by as usize;
        }

        result
    }

    /// Move the viewport up.
    pub fn up(&mut self) -> u16 {
        if self.y > 0 {
            self.y -= self.scroll_by;
        }

        self.y
    }

    /// Move the viewport down.
    pub fn down(&mut self) -> u16 {
        if self.y < self.height {
            self.y += self.scroll_by;
        }

        self.y
    }
}

#[cfg(test)]
mod tests {
    use crate::utility::Viewport;

    #[test]
    fn test_movement() {
        let mut viewport = Viewport::new(25, 7);
        assert_eq!(viewport.down(), 7);
        assert_eq!(viewport.down(), 14);
        assert_eq!(viewport.down(), 21);
        assert_eq!(viewport.down(), 28);
        assert_eq!(viewport.down(), 28);
        assert_eq!(viewport.up(), 21);
        assert_eq!(viewport.up(), 14);
        assert_eq!(viewport.up(), 7);
        assert_eq!(viewport.up(), 0);
        assert_eq!(viewport.up(), 0);
    }

    #[test]
    fn test_render() {
        let text = "one\r\ntwo\r\nthree\r\nfour\r\nfive\r\nsix\r\nseven\r\neight";

        let mut viewport = Viewport::new(10, 2);
        assert_eq!(viewport.down(), 2);
        assert_eq!(
            viewport.render(text),
            "three\r\nfour\r\nfive\r\nsix\r\nseven\r\neight\r\n\r\n\r\n\r\n"
        );
    }

    #[test]
    fn test_bounds() {
        // 11 lines.
        let text = vec!["_", "_", "_", "_", "_", "_", "_", "_", "_", "_", "_"];
        assert_eq!(text.len(), 11, "test cases expect 11 lines");

        let one = Viewport::new(5, 3).bounds(text.len());
        // dbg!(one);
        assert_eq!(one.len(), 3);
        assert!(one
            .get(&0)
            .is_some_and(|n| n.upper == 0 && n.lower.is_some_and(|f| f == 4)));
        assert!(one
            .get(&6)
            .is_some_and(|n| n.upper == 6 && n.lower.is_some_and(|f| f == 10)));

        let two = Viewport::new(4, 2).bounds(text.len());
        // dbg!(two);
        assert_eq!(two.len(), 5);
        assert!(two
            .get(&0)
            .is_some_and(|n| n.upper == 0 && n.lower.is_some_and(|f| f == 3)));
        assert!(two
            .get(&8)
            .is_some_and(|n| n.upper == 8 && n.lower.is_some_and(|f| f == 10)));

        let three = Viewport::new(10, 10).bounds(text.len());
        // dbg!(three);
        assert_eq!(three.len(), 2);
        assert!(three
            .get(&0)
            .is_some_and(|n| n.upper == 0 && n.lower.is_some_and(|f| f == 9)));
        assert!(three
            .get(&10)
            .is_some_and(|n| n.upper == 10 && n.lower.is_none()));
    }
}
