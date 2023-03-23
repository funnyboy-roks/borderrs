//! This crate allows the user to format many data structures in ways that look nicer to the
//! end-user.
//!
//! Currently, we support:
//! - [`slice`]s with [`BorderFormatter::format_slice`]
//! - [`Iterator`]s with [`BorderFormatter::format_iter`]
//! - [`HashMap`]s with [`BorderFormatter::format_hash_map`]
//! - impl [`Display`] with [`BorderFormatter::format_display`]
//! - impl [`Debug`] with [`BorderFormatter::format_debug`]
//!
//!
//! # Usage Example
//!
//! ```rust
//! use borders::{styles::THIN, BorderFormatter};
//!
//! let slice = [0, 1, 2, 3, 4];
//! println!("{}", THIN.format_slice(&slice));
//!
//! let mut map = HashMap::default();
//! map.insert("Jon", 38);
//! map.insert("Jake", 25);
//! map.insert("Josh", 17);
//! println!("{}", THIN.format_hash_map(&map));
//!
//! println!("{}", THIN.format_display("hello"));
//! println!("{}", THIN.format_debug("hello"));
//! ```
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

pub mod styles;

/// Represents a simple border style where all lines use the same format (determined by the values
/// in the struct)
pub struct SimpleBorderStyle {
    /// Used as a vertical separator
    vertical: char,
    /// Used as the horizontal separator
    horizontal: char,

    /// Used when there is a line connecting up, left, and right
    horizontal_up: char,
    /// Used when there is a line connecting down, left, and right
    horizontal_down: char,

    /// Used when there is a line connecting up, down, and right
    vertical_right: char,
    /// Used when there is a line connecting up, down, and left
    vertical_left: char,

    /// Used when there is a line connecting down and right (top-left corner)
    top_left: char,
    /// Used when there is a line connecting down and left (top-right corner)
    top_right: char,

    /// Used when there is a line connecting up and right (bottom-left corner)
    bottom_left: char,
    /// Used when there is a line connecting up and left (bottom-right corner)
    bottom_right: char,

    /// Used where ther is a line connecting in every direction
    cross: char,
}

/// Used to control the formatting for each type of BorderStyle
pub trait BorderFormatter {
    /// Format a slice into an horizontal table
    ///
    /// For example,
    /// ```rust
    /// # use borders::{styles, BorderFormatter};
    /// println!(
    ///     "{}",
    ///     styles::THIN.format_slice(&["Hello", "world", "how", "are", "you", "doing", "today"])
    /// );
    /// ```
    /// ```text
    /// ┌──────┬──────┬──────┬──────┬──────┬──────┬──────┐
    /// │ Hello│ world│   how│   are│   you│ doing│today?│
    /// └──────┴──────┴──────┴──────┴──────┴──────┴──────┘
    /// ```
    ///
    fn format_slice(&self, slice: &[impl Display]) -> String;

    /// Format an iterator into a horizontal table
    ///
    /// The default implementation collects the `iter` into a [`Vec`] and passes it to
    /// [`Self::format_slice`].
    ///
    /// See [`Self::format_slice`] for example and more info
    fn format_iter(&self, iter: impl Iterator<Item = impl Display>) -> String {
        self.format_slice(&iter.collect::<Vec<_>>())
    }

    /// Format a [`HashMap`] as a table using `Key` and `Value` as headers
    ///
    /// The default implementation calls [`Self::format_hash_map_headers`] with `"Key"` as the key
    /// header and `"Value"` as the value header.
    ///
    /// For example,
    /// ```rust
    /// # use borders::{styles, BorderFormatter};
    /// # use std::collections::HashMap;
    /// let mut map = HashMap::default();
    ///
    /// map.insert("Jon", 38);
    /// map.insert("Jake", 25);
    /// map.insert("Josh", 17);
    ///
    /// println!("{}", styles::THIN.format_hash_map(&map));
    /// ```
    /// produces the output
    /// ```text
    /// ┌─────┬─────┐
    /// │  Key│Value│
    /// ├─────┼─────┤
    /// │ Josh│   17│
    /// ├─────┼─────┤
    /// │  Jon│   38│
    /// ├─────┼─────┤
    /// │ Jake│   25│
    /// └─────┴─────┘
    /// ```
    fn format_hash_map(&self, map: &HashMap<impl Display, impl Display>) -> String {
        self.format_hash_map_headers(map, "Key", "Value")
    }

    /// Format a [`HashMap`] as a table using given headers
    ///
    /// If the headers are both empty, no header should be applied.
    ///
    /// For example,
    /// ```rust
    /// # use borders::{styles, BorderFormatter};
    /// # use std::collections::HashMap;
    /// let mut map = HashMap::default();
    ///
    /// map.insert("Jon", 38);
    /// map.insert("Jake", 25);
    /// map.insert("Josh", 17);
    ///
    /// println!(
    ///     "{}",
    ///     styles::THIN.format_hash_map_headers(&map, "Name", "Score")
    /// );
    /// ```
    /// produces the output:
    /// ```text
    /// ┌─────┬─────┐
    /// │ Name│Score│
    /// ├─────┼─────┤
    /// │ Josh│   17│
    /// ├─────┼─────┤
    /// │  Jon│   38│
    /// ├─────┼─────┤
    /// │ Jake│   25│
    /// └─────┴─────┘
    /// ```
    fn format_hash_map_headers(
        &self,
        map: &HashMap<impl Display, impl Display>,
        value_header: impl AsRef<str>,
        key_header: impl AsRef<str>,
    ) -> String;

    /// Add a border around anything that implements Display
    ///
    /// For example,
    /// ```rust
    /// # use borders::BorderFormatter;
    /// println!("{}", borders::styles::DOUBLE.format_display(" Hello World! "));
    /// ```
    /// produces
    /// ```text
    /// ╔══════════════╗
    /// ║ Hello World! ║
    /// ╚══════════════╝
    /// ```
    fn format_display(&self, val: impl Display) -> String {
        self.format_slice(&[format!("{}", val)])
    }

    /// Add a border around anything that implements Debug
    ///
    /// For example,
    /// ```rust
    /// # use borders::BorderFormatter;
    /// println!("{}", borders::styles::DOUBLE.format_debug("Hello World!"));
    /// ```
    /// produces
    /// ```text
    /// ╔══════════════╗
    /// ║"Hello World!"║
    /// ╚══════════════╝
    /// ```
    fn format_debug(&self, val: impl Debug) -> String {
        self.format_slice(&[format!("{:?}", val)])
    }
}

impl SimpleBorderStyle {
    /// Get the top line for a horizontal table with a consistent width
    fn get_top_line(&self, len: usize, width: usize) -> String {
        format!(
            "{}{}{}",
            self.top_left,
            (0..len)
                .map(|_| self.horizontal.to_string().repeat(width))
                .collect::<Vec<_>>()
                .join(&self.horizontal_down.to_string()),
            self.top_right
        )
    }

    /// Get the bottom line for a horizontal table with a consistent width
    fn get_bottom_line(&self, len: usize, width: usize) -> String {
        format!(
            "{}{}{}",
            self.bottom_left,
            (0..len)
                .map(|_| self.horizontal.to_string().repeat(width))
                .collect::<Vec<_>>()
                .join(&self.horizontal_up.to_string()),
            self.bottom_right,
        )
    }
}

impl BorderFormatter for SimpleBorderStyle {
    fn format_slice(&self, slice: &[impl Display]) -> String {
        // Format all values using [`Display`] (via `format!`)
        let entries: Vec<_> = slice.iter().map(|v| format!("{}", v)).collect();
        // Split into lines so we can do processing later
        let entry_lines: Vec<_> = entries.iter().map(|n| n.lines()).collect();

        // Get the height of the row
        let lines = entry_lines
            .iter()
            .map(|n| n.clone().count())
            .max()
            .unwrap_or(1);

        // Get the width of each column
        let len = entry_lines
            .iter()
            .map(|n| n.clone().map(|l| l.chars().count()).max().unwrap_or(0))
            .max()
            .unwrap_or(1);

        // Get the top/bottom lines
        let top_line = self.get_top_line(entries.len(), len);
        let bottom_line = self.get_bottom_line(entries.len(), len);

        let mut middle = String::new();
        for i in 0..lines {
            // Format each line
            middle += &format!(
                "{vert}{}{vert}\n",
                entry_lines
                    .iter()
                    // TODO: Figure out how to do this without cloning so much, it hurts my heart :(
                    .map(|l| format!("{:>len$}", l.clone().nth(i).unwrap_or(""), len = len))
                    .collect::<Vec<_>>()
                    .join(&self.vertical.to_string()),
                vert = self.vertical
            )
        }

        format!("{}\n{}{}", top_line, middle, bottom_line)
    }

    fn format_hash_map_headers(
        &self,
        map: &HashMap<impl Display, impl Display>,
        key_header: impl AsRef<str>,
        value_header: impl AsRef<str>,
    ) -> String {
        // Resolve the reference to cleanup code further down
        let key_header = key_header.as_ref();
        let value_header = value_header.as_ref();

        // Format all of the values using [`Display`] (via `format!`)
        let vals: Vec<_> = map.values().map(|v| format!("{}", v)).collect();
        // Split them into their lines so we can do processing later
        let vals: Vec<_> = vals.iter().map(|v| v.lines()).collect();

        // Get the longest value's length to use as the column width
        let val_width = vals
            .iter()
            .map(|l| l.clone().map(|v| v.len()).max().unwrap_or(1))
            .max()
            .unwrap_or(1)
            .max(value_header.len());

        // Format all keys using [`Display`] (via `format!`)
        let keys: Vec<_> = map.keys().map(|k| format!("{}", k)).collect();
        // Split them into their lines so we can do processing later
        let keys: Vec<_> = keys.iter().map(|k| k.lines()).collect();

        // Get the longest key's length to use as the column width
        let key_width = keys
            .iter()
            .map(|l| l.clone().map(|k| k.len()).max().unwrap_or(1))
            .max()
            .unwrap_or(1)
            .max(key_header.len());

        // Format the top line using the widths calculuated and the values in the struct
        let top_line = format!(
            "{}{}{}{}{}",
            self.top_left,
            self.horizontal.to_string().repeat(key_width),
            self.horizontal_down,
            self.horizontal.to_string().repeat(val_width),
            self.top_right
        );

        // Format the bottom line using the widths calculuated and the values in the struct
        let bottom_line = format!(
            "{}{}{}{}{}",
            self.bottom_left,
            self.horizontal.to_string().repeat(key_width),
            self.horizontal_up,
            self.horizontal.to_string().repeat(val_width),
            self.bottom_right
        );

        let mut entries: Vec<_> = keys
            .iter()
            .zip(vals)
            .map(|(key, val)| (key.clone(), val))
            .collect();

        // Put the header on the top of the table if they are provided
        if !key_header.is_empty() || !value_header.is_empty() {
            let mut new_entries = vec![(key_header.lines(), value_header.lines())];
            new_entries.extend(entries);
            entries = new_entries;
        }

        let mut middle = String::new();
        for i in 0..entries.len() {
            let (ref mut key, ref mut val) = entries[i];
            let height = key.clone().count().max(val.clone().count()); // The height of this row
            for _ in 0..height {
                // Add the line
                middle += &format!(
                    "{vert}{key:>key_width$}{vert}{val:>val_width$}{vert}\n",
                    key = key.next().unwrap_or(""), // Get the next line or nothing if we're out of lines to grab
                    val = val.next().unwrap_or(""), // ^
                    key_width = key_width,
                    val_width = val_width,
                    vert = self.vertical
                )
            }

            // If we are before the last item
            if i < entries.len() - 1 {
                // Apply the middle line
                middle += &format!(
                    "{}{}{}{}{}\n",
                    self.vertical_right,
                    self.horizontal.to_string().repeat(key_width),
                    self.cross,
                    self.horizontal.to_string().repeat(val_width),
                    self.vertical_left,
                )
            }
        }

        format!("{}\n{}{}", top_line, middle, bottom_line)
    }
}
