# Digital Certificate Locker

Kontrak pintar (Smart Contract) berbasis **Soroban (Stellar)** untuk penerbitan dan verifikasi sertifikat digital secara on-chain.

## 📋 Deskripsi Proyek
Proyek ini mengonversi logika CRUD sederhana menjadi sistem manajemen kredensial yang aman. Setiap sertifikat yang diterbitkan bersifat permanen (immutable) dan terikat pada alamat publik (Address) pemiliknya di jaringan Stellar.

## 🛠️ Spesifikasi Teknis
- **Bahasa**: Rust
- **SDK**: Soroban SDK v20.0.0+
- **Penyimpanan**: Persistent Storage (Mapping ID -> Certificate)
- **Keamanan**: `Address.require_auth()` untuk validasi Issuer

## 🏗️ Struktur Data
```rust
pub struct Certificate {
    pub id: u64,             // Auto-increment ID
    pub owner: Address,      // Penerima sertifikat
    pub issuer_name: String, // Nama Institusi
    pub cert_name: String,   // Nama Sertifikat
    pub description: String, // Detail pencapaian
    pub date: u64,           // Timestamp blok otomatis
}