pub struct Money {
  pub kind: MoneyKind,
  pub amount: u16
}

impl Money {
  pub fn new(kind: Option<MoneyKind>, amount: u16) -> Money {
      match kind {
          Some(k) => Money { kind: k, amount },
          None => Money { kind: MoneyKind::Usd, amount }
      }
  }
}

pub enum MoneyKind {
  Usd
}