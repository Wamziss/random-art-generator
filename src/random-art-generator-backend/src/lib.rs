use candid::candid_method;
use ic_cdk::update;
use ic_cdk::query;

use candid::{CandidType, Deserialize};
use ic_cdk::api::call::call_with_payment;
// use ic_cdk::api::Principal;
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
async fn generate_art() -> u64 {
    let caller = ic_cdk::caller();
    let id = NEXT_ID.with(|id| {
        let next_id = *id.borrow();
        *id.borrow_mut() = next_id + 1;
        next_id
    });

    let quantum_state = create_quantum_state().await;
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

    id
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

async fn create_quantum_state() -> QuantumState {
    let (rng_bytes,): (Vec<u8>,) = call_with_payment(
        Principal::management_canister(),
        "raw_rand",
        (),
        0,
    )
    .await
    .expect("Failed to get random bytes");
    
    // You need to process rng_bytes to extract 4-byte chunks and turn them into u32 values.
    let superposition: Vec<f64> = rng_bytes
        .chunks(4) // Taking 4 bytes at a time
        .map(|chunk| {
            let mut array = [0u8; 4];
            array.copy_from_slice(chunk);
            let random_u32 = u32::from_le_bytes(array);
            let random_float = (random_u32 as f64) / (u32::MAX as f64); // Normalize to [0, 1)
            random_float
        })
        .collect();
    
    let entanglement: Vec<usize> = rng_bytes
        .chunks(4) // Reuse chunks to create entanglements
        .map(|chunk| {
            let mut array = [0u8; 4];
            array.copy_from_slice(chunk);
            let random_u32 = u32::from_le_bytes(array);
            (random_u32 % 10) as usize // Mapping to an integer
        })
        .collect();
    
    QuantumState {
        superposition,
        entanglement,
    }
}

ic_cdk::export_candid!();

// #[query(name = "__get_candid_interface_tmp_hack")]
// fn export_candid() -> String {
//     // candid::export_service!();
//     __export_service()
// }

// candid::export_service!();

// #[ic_cdk_macros::init]
// fn init() {
//     // Any initialization code can go here
// }
