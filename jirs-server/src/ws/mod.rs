use actix::{Actor, Addr, StreamHandler};
use actix_web::web::Data;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use jirs_data::{Project, WsMsg};

use crate::db::projects::LoadCurrentProject;
use crate::db::DbExecutor;

struct WebSocketActor {
    db: Data<Addr<DbExecutor>>,
}

impl Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        use futures::executor::block_on;

        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => {
                let ws_msg: bincode::Result<jirs_data::WsMsg> =
                    bincode::deserialize(bin.to_vec().as_slice());
                match ws_msg {
                    Ok(WsMsg::Ping) => ctx.binary(bincode::serialize(&WsMsg::Pong).unwrap()),
                    Ok(WsMsg::Pong) => ctx.binary(bincode::serialize(&WsMsg::Ping).unwrap()),
                    Ok(WsMsg::ProjectRequest) => match block_on(load_project(self.db.clone())) {
                        Some(p) => {
                            ctx.binary(bincode::serialize(&WsMsg::ProjectLoaded(p)).unwrap())
                        }
                        _ => eprintln!("Failed to load project"),
                    },
                    _ => eprintln!("Failed to resolve message"),
                };
            }
            _ => (),
        }
    }
}

pub async fn load_project(db: Data<Addr<DbExecutor>>) -> Option<Project> {
    match db.send(LoadCurrentProject { project_id: 1 }).await {
        Ok(Ok(p)) => Some(p.into()),
        Ok(e) => {
            eprintln!("{:?}", e);
            None
        }
        _ => None,
    }
}

#[get("/ws/")]
pub async fn index(
    req: HttpRequest,
    stream: web::Payload,
    db: Data<Addr<DbExecutor>>,
) -> Result<HttpResponse, Error> {
    ws::start(WebSocketActor { db }, &req, stream)
}
