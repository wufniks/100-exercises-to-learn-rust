use std::sync::mpsc::{Receiver, Sender};
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};

pub mod data;
pub mod store;

// Refer to the tests to understand the expected schema.
pub enum Command {
    Insert {
        draft: TicketDraft,
        response_sender: Sender<TicketId>
    },
    Get {
        id: TicketId,
        response_sender: Sender<Option<Ticket>>,
    }
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: handle incoming commands as expected.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert { draft, response_sender: ack }) => {
                let ticket_id = store.add_ticket(draft);
                ack.send(ticket_id).unwrap();
            }
            Ok(Command::Get {
                   id: ticket_id,
                   response_sender: ack,
            }) => {
                let ticket = store.get(ticket_id);
                ack.send(ticket.cloned()).unwrap();
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break
            },
        }
    }
}
