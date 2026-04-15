#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Certificate {
    pub id: u64,
    pub owner: Address,
    pub issuer_name: String,
    pub cert_name: String,
    pub description: String,
    pub date: u64,
}

// Key untuk menyimpan ID terakhir agar tidak bentrok
const LAST_ID: Symbol = symbol_short!("LAST_ID");

#[contract]
pub struct CertificateContract;

#[contractimpl]
impl CertificateContract {

    // Fungsi untuk menerbitkan sertifikat (Hanya bisa dipanggil oleh Issuer)
    pub fn issue_cert(
        env: Env, 
        issuer: Address, 
        owner: Address, 
        issuer_name: String, 
        cert_name: String, 
        description: String
    ) -> u64 {
        // Memastikan yang memanggil fungsi ini adalah pihak Issuer
        issuer.require_auth();

        // Generate ID sederhana (increment)
        let mut last_id: u64 = env.storage().instance().get(&LAST_ID).unwrap_or(0);
        last_id += 1;

        let certificate = Certificate {
            id: last_id,
            owner,
            issuer_name,
            cert_name,
            description,
            date: env.ledger().timestamp(), // Tanggal otomatis dari blockchain
        };

        // Simpan sertifikat berdasarkan ID-nya langsung (O(1) access)
        env.storage().persistent().set(&last_id, &certificate);
        
        // Simpan ID terakhir kembali
        env.storage().instance().set(&LAST_ID, &last_id);

        last_id
    }

    // Ambil data sertifikat berdasarkan ID
    pub fn get_cert(env: Env, id: u64) -> Option<Certificate> {
        env.storage().persistent().get(&id)
    }

    // Fungsi untuk menghapus sertifikat (Hanya jika salah input, oleh Issuer)
    pub fn revoke_cert(env: Env, issuer: Address, id: u64) {
        issuer.require_auth();
        env.storage().persistent().remove(&id);
    }
}