pub struct TicketRandomizer {
  tickets_to_pick_from: Vec<Ticket>,
}

impl TicketRandomizer {
  fn randomly_pick_ticket(&self) -> &Ticket {
      let pick = rand::random::<u8>() % self.tickets_to_pick_from.len() as u8;

      return &self.tickets_to_pick_from[pick as usize];
  }
}

#[cfg(test)]
mod tests {
  fn picks_a_ticket() {
    
  }
}