use crate::money::Money;
use crate::money::MoneyKind;

pub struct Ticket {
  id: u16,
  pub cost: Money,
  pub kind: TicketKind
}

impl Ticket {
  pub fn new(kind: TicketKind) -> Ticket {
      let id = rand::random::<u16>();
      let usd = Some(MoneyKind::Usd);
      match kind {
          TicketKind::DisneyWorld => Ticket { id, cost: Money::new(usd, 200), kind},
          TicketKind::DisneyLand => Ticket { id, cost: Money::new(usd, 150), kind},
          TicketKind::Lagoon => Ticket { id, cost: Money::new(usd, 80), kind},
          TicketKind::ReelTheatre => Ticket { id, cost: Money::new(usd, 20), kind},
          TicketKind::Football => Ticket { id, cost: Money::new(usd, 200), kind},
          TicketKind::RoaringSprings => Ticket { id, cost: Money::new(usd, 100), kind}
      }
  }
}

pub struct TicketRandomizer {
  tickets_to_pick_from: Vec<Ticket>,
}

impl TicketRandomizer {
  pub fn randomly_pick_ticket(&self) -> &Ticket {
      let pick = rand::random::<u8>() % self.tickets_to_pick_from.len() as u8;

      return &self.tickets_to_pick_from[pick as usize];
  }
}

pub enum TicketKind {
  DisneyWorld,
  Lagoon,
  DisneyLand,
  ReelTheatre,
  Football,
  RoaringSprings
}

pub enum PurchaseTicketErr {
  NotEnoughFunds,
}