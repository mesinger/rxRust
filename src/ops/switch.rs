use crate::prelude::*;

#[derive(Clone)]
pub struct SwitchOp<S> {
  pub(crate) source: S,
}

impl<S> Observable for SwitchOp<S> where S: Observable, S::Item: Observable {
  type Item = <S::Item as Observable>::Item;
  type Err = <S::Item as Observable>::Err;
}

impl<'a, S> LocalObservable<'a> for SwitchOp<S> where S: LocalObservable<'a> + Observable<Item = S> + 'a  {
  type Unsub = S::Unsub;

  fn actual_subscribe<O>(self, observer: O) -> Self::Unsub where O: Observer<Item=Self::Item, Err=Self::Err> + 'a {
    self.source.actual_subscribe(SwitchObserver {
      observer,
      current_observable: None,
    })
  }
}

pub struct SwitchObserver<O, OO> {
  observer: O,
  current_observable: Option<OO>
}

impl<'a, O, OO> Observer for SwitchObserver<O, OO> where O: Observer<Item = OO::Item, Err = OO::Err>, OO: LocalObservable<'a> {
  type Item = OO;
  type Err = OO::Err;

  fn next(&mut self, value: Self::Item) {
    println!("next");
  }

  fn error(&mut self, err: Self::Err) {
    println!("error");
  }

  fn complete(&mut self) {
    println!("complete");
  }
}

#[cfg(test)]
mod tests {
  use crate::prelude::*;

  #[test]
  fn test() {
    observable::from_iter(0..3)
        .map(|i| observable::from_iter(i..3))
        .switch().subscribe(|v| println!("woama"));

    println!("shibe")
  }
}
