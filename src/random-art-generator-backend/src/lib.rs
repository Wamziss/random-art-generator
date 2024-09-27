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

#[derive(Clone, Debug, CandidType, Deserialize)]
struct ArtMetadata {
    total_pieces: usize,
    next_id: u64,
}

type ArtStorage = HashMap<u64, ArtPiece>;

thread_local! {
    static ART_STORAGE: RefCell<ArtStorage> = RefCell::new(HashMap::new());
    static NEXT_ID: RefCell<u64> = RefCell::new(0);
}

// Custom error types for better error handling
#[derive(Debug)]
enum ArtError {
    NotFound,
    NotAuthorized,
    QuantumStateError(String),
    GeneralError(String),
}

impl From<ArtError> for String {
    fn from(e: ArtError) -> String {
        match e {
            ArtError::NotFound => "Art piece not found".to_string(),
            ArtError::NotAuthorized => "You are not authorized to perform this action".to_string(),
            ArtError::QuantumStateError(msg) => format!("Quantum state error: {}", msg),
            ArtError::GeneralError(msg) => msg,
        }
    }
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

    let quantum_state = create_quantum_state().await.map_err(ArtError::from)?;

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
fn get_art(id: u64) -> Result<ArtPiece, String> {
    ART_STORAGE.with(|storage| {
        storage.borrow().get(&id).cloned().ok_or_else(|| ArtError::NotFound.into())
    })
}

#[query]
#[candid_method(query)]
fn get_all_art() -> Vec<ArtPiece> {
    ART_STORAGE.with(|storage| storage.borrow().values().cloned().collect())
}

#[query]
#[candid_method(query)]
fn get_metadata() -> ArtMetadata {
    ART_STORAGE.with(|storage| {
        let total_pieces = storage.borrow().len();
        let next_id = NEXT_ID.with(|id| *id.borrow());
        ArtMetadata {
            total_pieces,
            next_id,
        }
    })
}

#[update]
#[candid_method(update)]
async fn transfer_ownership(id: u64, new_owner: Principal) -> Result<(), String> {
    let caller = ic_cdk::caller();

    ART_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(art_piece) = storage.get_mut(&id) {
            if art_piece.creator != caller {
                return Err(ArtError::NotAuthorized.into());
            }
            art_piece.creator = new_owner;
            Ok(())
        } else {
            Err(ArtError::NotFound.into())
        }
    })
}

#[update]
#[candid_method(update)]
fn delete_art(id: u64) -> Result<(), String> {
    let caller = ic_cdk::caller();

    ART_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(art_piece) = storage.get(&id) {
            if art_piece.creator != caller {
                return Err(ArtError::NotAuthorized.into());
            }
            storage.remove(&id);
            Ok(())
        } else {
            Err(ArtError::NotFound.into())
        }
    })
}

async fn create_quantum_state() -> Result<QuantumState, ArtError> {
    let (rng_bytes,): (Vec<u8>,) = call_with_payment(
        Principal::management_canister(),
        "raw_rand",
        (),
        0,
    )
    .await
    .map_err(|e| ArtError::QuantumStateError(format!("Failed to get random bytes: {:?}", e)))?;

    let quantum_state = process_random_bytes(rng_bytes);
    Ok(quantum_state)
}

fn process_random_bytes(rng_bytes: Vec<u8>) -> QuantumState {
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

    QuantumState {
        superposition,
        entanglement,
    }
}

ic_cdk::export_candid!();