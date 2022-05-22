pub struct JobResult {
  pub payed_amount: u16
}

pub struct JobErr {
  pub kind: JobFailureKind,
  pub payed_amount: u16
}

pub enum JobFailureKind {
  ForgotToShowUp,
  LazyWorker,
  FuckedUp
}

pub enum JobKind {
  SoftwareEngineer,
  Retail
}