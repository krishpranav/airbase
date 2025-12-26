use axum::extract::ws::{WebSocketUpgrade, Message};
use axum::{response::IntoResponse, extract::State};
use crate::state::AppState;

pub async fn ws(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |mut socket| async move {
        let mut rx = state.events.subscribe();
        while let Ok(msg) = rx.recv().await {
            let _ = socket.send(Message::Text(msg)).await;
        }
    })
}