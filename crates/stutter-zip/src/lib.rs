pub struct StutterZip<I, J, F> {
  iter_a: I,
  iter_b: J,
  condition: F,
}

impl<I, J, F> Iterator for StutterZip<I, J, F>
where
  I: Iterator,
  J: Iterator,
  F: Fn(&I::Item) -> bool,
{
  type Item = (I::Item, Option<J::Item>);

  fn next(&mut self) -> Option<Self::Item> {
    self.iter_a.next().map(|a| {
      let b = if (self.condition)(&a) {
        None
      } else {
        self.iter_b.next()
      };
      (a, b)
    })
  }
}

pub trait StutterZipIterator: Iterator {
  fn stutter_zip<J, F>(self, iter_b: J, condition: F) -> StutterZip<Self, J, F>
  where
    Self: Sized,
    J: Iterator,
    F: Fn(&Self::Item) -> bool;
}

impl<I> StutterZipIterator for I
where
  I: Iterator,
{
  fn stutter_zip<J, F>(self, iter_b: J, condition: F) -> StutterZip<Self, J, F>
  where
    Self: Sized,
    J: Iterator,
    F: Fn(&Self::Item) -> bool,
  {
    StutterZip {
      iter_a: self,
      iter_b,
      condition,
    }
  }
}
