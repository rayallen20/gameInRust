#[derive(Debug)]
pub enum Action {
    Accept,
    AcceptWithNote {note: String},
    Refuse,
    Probation,
}