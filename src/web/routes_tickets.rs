use crate::ctx::Ctx;
use crate::model::{CreateTicketInput, ModelController, Ticket};
use crate::ApiResult;
use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}

async fn create_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(ticket_fc): Json<CreateTicketInput>,
) -> ApiResult<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");
    let ticket = mc.create_ticket(&ctx, ticket_fc).await?;
    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>) -> Json<Vec<Ticket>> {
    println!("->> {:<12} - list_tickets", "HANDLER");
    Json(mc.list_tickets().await)
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> ApiResult<Json<Ticket>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");
    let tickets = mc.delete_ticket(&ctx, id).await?;
    Ok(Json(tickets))
}
