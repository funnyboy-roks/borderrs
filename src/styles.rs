//! This module hosts the built-in styles.
//!
//! The inteded usage is as follows:
//! ```rust
//! use borders::{BorderFormatter, stypes::THIN};
//!
//! let display: String = THIN.format_display("Hello World!");
//! ```
use crate::SimpleBorderStyle;

/// Format with a single thin line
///
/// ```text
/// ┌───┬───┐
/// │   │   │
/// ├───┼───┤
/// │   │   │
/// └───┴───┘
/// ```
pub const THIN: SimpleBorderStyle = SimpleBorderStyle {
    vertical: '│',
    horizontal: '─',

    horizontal_up: '┴',
    horizontal_down: '┬',

    vertical_right: '├',
    vertical_left: '┤',

    top_left: '┌',
    top_right: '┐',

    bottom_left: '└',
    bottom_right: '┘',

    cross: '┼',
};

/// Format with a double line
///
/// ```text
/// ╔═══╦═══╗
/// ║   ║   ║
/// ╠═══╬═══╣
/// ║   ║   ║
/// ╚═══╩═══╝
/// ```
pub const DOUBLE: SimpleBorderStyle = SimpleBorderStyle {
    vertical: '║',
    horizontal: '═',

    horizontal_up: '╩',
    horizontal_down: '╦',

    vertical_right: '╠',
    vertical_left: '╣',

    top_left: '╔',
    top_right: '╗',

    bottom_left: '╚',
    bottom_right: '╝',

    cross: '╬',
};

/// Format using only ASCII characters (`+`, `-`, `|`)
///
/// ```text
/// +---+---+
/// |   |   |
/// +---+---+
/// |   |   |
/// +---+---+
/// ```
pub const ASCII: SimpleBorderStyle = SimpleBorderStyle {
    vertical: '|',
    horizontal: '-',

    horizontal_up: '+',
    horizontal_down: '+',

    vertical_right: '+',
    vertical_left: '+',

    top_left: '+',
    top_right: '+',

    bottom_left: '+',
    bottom_right: '+',

    cross: '+',
};
