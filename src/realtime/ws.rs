use axum::extract::ws::*;
use axum::{extract::State, response::IntoResponse};
use std::collections::HashSet;
use serde_json::Value;

use crate::state::AppState;

pub async fn handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |mut socket| async move {
        let mut subs = HashSet::new();
        let mut rx = state.events.subscribe();

        loop {
            tokio::select! {
                Some(Ok(Message::Text(msg))) = socket.recv() => {
                    let v: Value = serde_json::from_str(&msg).unwrap();
                    if v["type"] == "subscribe" {
                        subs.insert(v["collection"].as_str().unwrap().to_string());
                    }
                }

                Ok(event) = rx.recv() => {
                    let e: Value = serde_json::from_str(&event).unwrap();
                    if subs.contains(e["collection"].as_str().unwrap()) {
                        socket.send(Message::Text(event)).await.ok();
                    }
                }
            }
        }
    })
}