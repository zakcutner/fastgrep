use std::str::FromStr;

pub struct Positive(usize);

impl FromStr for Positive {
  type Err = ();

  fn from_str(src: &str) -> Result<Self, Self::Err> {
    match src.parse::<usize>() {
      Ok(num) => {
        if num > 0 {
          Ok(Positive(num))
        } else {
          Err(())
        }
      }
      Err(_) => Err(()),
    }
  }
}

impl From<Positive> for usize {
  fn from(Positive(num): Positive) -> Self {
    num
  }
}
