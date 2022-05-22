use rand;

fn main() {
    let mut user = User::new("ben");

    let res = loop {
        match user.purchase_ticket(Ticket::new(TicketKind::Lagoon)) {
            Ok(res) => {
                println!("User purchased ticket successfully");
                break res;
            },
            Err(_) => {
                println!("Couldn't afford to purchase ticket");
                println!("User now has to get a job");
                match user.get_a_job(10, JobKind::Retail) {
                    Ok(res) => println!("User made ${}", res.payed_amount),
                    Err(_) => println!("User fucked up")
                };
                continue;
            },
        };
    };

}

struct User {
    id: u16,
    name: String,
    tickets: Vec<Ticket>,
    wallet: Money
}

impl User {
    fn new(name: &str) -> User {
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

struct Ticket {
    id: u16,
    cost: Money,
    kind: TicketKind
}

impl Ticket {
    fn new(kind: TicketKind) -> Ticket {
        let id = rand::random::<u16>();
        let usd = Some(MoneyKind::Usd);
        match kind {
            TicketKind::DisneyWorld => Ticket { id, cost: Money::new(usd, 200), kind},
            TicketKind::DisneyLand => Ticket { id, cost: Money::new(usd, 150), kind},
            TicketKind::Lagoon => Ticket { id, cost: Money::new(usd, 80), kind},
            TicketKind::ReelTheatre => Ticket { id, cost: Money::new(usd, 20), kind},
            TicketKind::Football => Ticket { id, cost: Money::new(usd, 200), kind}
        }
    }
}

struct Money {
    kind: MoneyKind,
    amount: u16
}

impl Money {
    fn new(kind: Option<MoneyKind>, amount: u16) -> Money {
        match kind {
            Some(k) => Money { kind: k, amount },
            None => Money { kind: MoneyKind::Usd, amount }
        }
    }
}

enum MoneyKind {
    Usd
}

enum TicketKind {
    DisneyWorld,
    Lagoon,
    DisneyLand,
    ReelTheatre,
    Football
}

enum PurchaseTicketErr {
    NotEnoughFunds,
}

struct JobResult {
    payed_amount: u16
}

struct JobErr {
    kind: JobFailureKind,
    payed_amount: u16
}

enum JobFailureKind {
    ForgotToShowUp,
    LazyWorker,
    FuckedUp
}

enum JobKind {
    SoftwareEngineer,
    Retail
}

