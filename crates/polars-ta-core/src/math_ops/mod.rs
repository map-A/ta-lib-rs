//! Math Operator indicators: arithmetic operations and rolling window statistics.

pub mod add;
pub mod div;
pub mod max;
pub mod maxindex;
pub mod min;
pub mod minindex;
pub mod minmax;
pub mod minmaxindex;
pub mod mult;
pub mod sub;
pub mod sum;

pub use add::add;
pub use div::div;
pub use max::max;
pub use maxindex::maxindex;
pub use min::min;
pub use minindex::minindex;
pub use minmax::{minmax, MinMaxOutput};
pub use minmaxindex::{minmaxindex, MinMaxIndexOutput};
pub use mult::mult;
pub use sub::sub;
pub use sum::sum;
