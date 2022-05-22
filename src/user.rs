use crate::money::{Money, MoneyKind};
use crate::ticket::{Ticket, PurchaseTicketErr};
use crate::job::{JobErr, JobKind, JobResult, JobFailureKind};

pub struct User {
  id: u16,
  name: String,
  tickets: Vec<Ticket>,
  wallet: Money
}

impl User {
  pub fn new(name: &str) -> User {
      User { 
          id: rand::random(), 
          name: name.to_string(), 
          tickets: Vec::new(),
          wallet: Money { kind: MoneyKind::Usd, amount: 0 }
      }
  }
  fn add_ticket(&mut self, ticket: Ticket) {
      self.tickets.push(ticket)
  }
  fn purchase_ticket(&mut self, ticket: Ticket) -> Result<(), PurchaseTicketErr> {
      if self.wallet.amount < ticket.cost.amount {
          Err(PurchaseTicketErr::NotEnoughFunds)
      } else {
          self.wallet.amount -= ticket.cost.amount;
          self.add_ticket(ticket);
          Ok(())
      }
  }
  fn get_a_job(&mut self, dur: u16, kind: JobKind) -> Result<JobResult, JobErr> {
      let chance = rand::random::<u8>();

      let res: Result<JobResult, JobErr> = if chance < 10 {
          Err(JobErr {kind: JobFailureKind::ForgotToShowUp, payed_amount: 0})
      } else if chance < 20 {
          Err(JobErr {kind: JobFailureKind::FuckedUp, payed_amount: 10})
      } else if chance < 30 {
          Err(JobErr {kind: JobFailureKind::LazyWorker, payed_amount: 20})
      } else {
          match kind {
              JobKind::Retail => Ok(JobResult {payed_amount: 10 * dur}),
              JobKind::SoftwareEngineer => Ok(JobResult {payed_amount: 30 * dur}),
          }
      };

      match res {
          Err(err) => {
              self.wallet.amount += err.payed_amount;
              Err(err)
          },
          Ok(got) => {
              self.wallet.amount += got.payed_amount;
              Ok(got)
          }
      }
  }
}

