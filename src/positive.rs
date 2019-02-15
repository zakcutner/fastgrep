use std::str::FromStr;

pub struct Positive(usize);

impl FromStr for Positive {
  type Err = ();

  fn from_str(src: &str) -> Result<Self, Self::Err> {
    if let Ok(num) = src.parse::<usize>() {
      if num > 0 {
        return Ok(Positive(num));
      }
    }

    Err(())
  }
}

impl From<Positive> for usize {
  fn from(Positive(num): Positive) -> Self {
    num
  }
}
