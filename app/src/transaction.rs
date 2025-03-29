use std::time::Instant;
use serde::{Serialize, Deserialize};
use bevy::prelude::*;

type TransactionId = u64;

#[derive(Debug)]
pub struct Transaction {
    id: TransactionId,
    timestamp: Instant,
    operations: Operation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Applied,
    Rejected { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct TransactionResponse {
    transaction_id: u64,
    status: TransactionStatus,
}


#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub enum Operation {
    Undo {
        transaction_id: u64,
    },
    Redo {
        transaction_id: u64,
    },
}



