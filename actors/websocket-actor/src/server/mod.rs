use std::collections::HashMap;

use actix::{Actor, Context, Recipient};
use jirs_data::{ProjectId, UserId, WsMsg};

#[derive(actix::Message, Debug)]
#[rtype(result = "()")]
pub enum InnerMsg {
    Join(ProjectId, UserId, Recipient<InnerMsg>),
    Leave(ProjectId, UserId, Recipient<InnerMsg>),
    BroadcastToChannel(ProjectId, WsMsg),
    SendToUser(UserId, WsMsg),
    Transfer(WsMsg),
}

pub struct WsServer {
    sessions: HashMap<UserId, Vec<Recipient<InnerMsg>>>,
    rooms: HashMap<ProjectId, HashMap<UserId, i32>>,
}

impl Default for WsServer {
    fn default() -> Self {
        Self {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
        }
    }
}

impl actix::Message for WsServer {
    type Result = ();
}

impl actix::Actor for WsServer {
    type Context = Context<Self>;
}

impl actix::Handler<InnerMsg> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: InnerMsg, _ctx: &mut <Self as Actor>::Context) -> Self::Result {
        debug!("receive {:?}", msg);
        match msg {
            InnerMsg::Join(project_id, user_id, recipient) => {
                let v = self
                    .sessions
                    .entry(user_id)
                    .or_insert_with(Default::default);
                v.push(recipient);
                self.ensure_room(project_id);

                if let Some(room) = self.rooms.get_mut(&project_id) {
                    let n = *room.entry(user_id).or_insert(0);
                    room.insert(user_id, n + 1);
                }
            }
            InnerMsg::Leave(project_id, user_id, recipient) => {
                self.ensure_room(project_id);
                let room = match self.rooms.get_mut(&project_id) {
                    Some(room) => room,
                    None => return,
                };
                let n = *room.entry(user_id).or_insert(0);
                if n <= 1 {
                    room.remove(&user_id);
                    self.sessions.remove(&user_id);
                } else {
                    let v = self.sessions.entry(user_id).or_insert_with(Vec::new);
                    let mut old = vec![];
                    std::mem::swap(&mut old, v);
                    for r in old {
                        if r != recipient {
                            v.push(r);
                        }
                    }
                }
            }
            InnerMsg::SendToUser(user_id, msg) => {
                if let Some(v) = self.sessions.get(&user_id) {
                    self.send_to_recipients(v, &msg);
                }
            }
            InnerMsg::BroadcastToChannel(project_id, msg) => {
                debug!("Begin broadcast to channel {} msg {:?}", project_id, msg);
                let set = match self.rooms.get(&project_id) {
                    Some(s) => s,
                    _ => return debug!("  channel not found, aborting..."),
                };
                for r in set.keys() {
                    let v = match self.sessions.get(r) {
                        Some(v) => v,
                        _ => {
                            debug!("recipient is dead, skipping...");
                            continue;
                        }
                    };
                    self.send_to_recipients(v, &msg);
                }
            }
            _ => (),
        }
    }
}

impl WsServer {
    pub fn ensure_room(&mut self, room: i32) {
        self.rooms.entry(room).or_insert_with(HashMap::new);
    }

    fn send_to_recipients(&self, recipients: &[Recipient<InnerMsg>], msg: &WsMsg) {
        for recipient in recipients.iter() {
            match recipient.do_send(InnerMsg::Transfer(msg.clone())) {
                Ok(_) => debug!("msg sent"),
                Err(e) => error!("{}", e),
            };
        }
    }
}
