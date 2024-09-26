use candid::candid_method;
use ic_cdk::update;
use ic_cdk::query;
use candid::{CandidType, Deserialize};
use ic_cdk::api::call::call_with_payment;
use candid::Principal;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Debug, CandidType, Deserialize)]
struct QuantumState {
    superposition: Vec<f64>,
    entanglement: Vec<usize>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct ArtPiece {
    id: u64,
    creator: Principal,
    quantum_state: QuantumState,
    timestamp: u64,
}

type ArtStorage = HashMap<u64, ArtPiece>;

thread_local! {
    static ART_STORAGE: RefCell<ArtStorage> = RefCell::new(HashMap::new());
    static NEXT_ID: RefCell<u64> = RefCell::new(0);
}

#[update]
#[candid_method(update)]
async fn generate_art() -> Result<u64, String> {
    let caller = ic_cdk::caller();
    let id = NEXT_ID.with(|id| {
        let next_id = *id.borrow();
        *id.borrow_mut() = next_id + 1;
        next_id
    });

    // Generate quantum state and handle potential errors
    let quantum_state = match create_quantum_state().await {
        Ok(state) => state,
        Err(e) => return Err(format!("Failed to create quantum state: {}", e)),
    };

    let timestamp = ic_cdk::api::time();

    let art_piece = ArtPiece {
        id,
        creator: caller,
        quantum_state,
        timestamp,
    };

    ART_STORAGE.with(|storage| {
        storage.borrow_mut().insert(id, art_piece);
    });

    Ok(id)
}

#[query]
#[candid_method(query)]
fn get_art(id: u64) -> Option<ArtPiece> {
    ART_STORAGE.with(|storage| storage.borrow().get(&id).cloned())
}

#[query]
#[candid_method(query)]
fn get_all_art() -> Vec<ArtPiece> {
    ART_STORAGE.with(|storage| storage.borrow().values().cloned().collect())
}

async fn create_quantum_state() -> Result<QuantumState, String> {
    // Call the raw_rand function and handle potential errors
    let (rng_bytes,): (Vec<u8>,) = call_with_payment(
        Principal::management_canister(),
        "raw_rand",
        (),
        0,
    )
    .await
    .map_err(|e| format!("Failed to get random bytes: {:?}", e))?;

    // Process rng_bytes to create superposition and entanglement
    let superposition: Vec<f64> = rng_bytes
        .chunks(4)
        .map(|chunk| {
            let mut array = [0u8; 4];
            array.copy_from_slice(chunk);
            let random_u32 = u32::from_le_bytes(array);
            (random_u32 as f64) / (u32::MAX as f64)
        })
        .collect();

    let entanglement: Vec<usize> = rng_bytes
        .chunks(4)
        .map(|chunk| {
            let mut array = [0u8; 4];
            array.copy_from_slice(chunk);
            let random_u32 = u32::from_le_bytes(array);
            (random_u32 % 10) as usize
        })
        .collect();

    Ok(QuantumState {
        superposition,
        entanglement,
    })
}

ic_cdk::export_candid!();