# Chubbishop

Chubbishop adalah platform e-commerce yang dibangun dengan arsitektur microservices. Proyek ini dirancang untuk memberikan pengalaman belanja yang cepat, aman, dan mudah digunakan bagi pengguna, serta memberikan fleksibilitas dan skalabilitas bagi pengembang.

## Fitur Utama

- **Microservices Architecture**: Setiap layanan berfungsi secara independen, memungkinkan pengembangan dan pengelolaan yang lebih mudah.
- **User Management**: Registrasi, login, dan manajemen profil pengguna.
- **Product Management**: Tambah, edit, dan hapus produk dengan kategori dan atribut yang dapat disesuaikan.
- **Shopping Cart**: Pengguna dapat menambahkan produk ke keranjang belanja dan melakukan checkout.
- **Order Management**: Melacak status pesanan dan riwayat pembelian.
- **Payment Integration**: Mendukung berbagai metode pembayaran yang aman.
- **Admin Dashboard**: Antarmuka untuk mengelola produk, pesanan, dan pengguna.

## Arsitektur

Chubbishop terdiri dari beberapa microservices, termasuk:

- **User Service**: Mengelola semua fungsi terkait pengguna.
- **Product Service**: Mengelola produk dan kategori.
- **Order Service**: Mengelola pesanan dan statusnya.
- **Payment Service**: Mengelola transaksi pembayaran.
- **Notification Service**: Mengirimkan notifikasi kepada pengguna.

## Teknologi yang Digunakan

- **Backend**: Rust
- **Database**: PostgreSQL, Redis, ElasticSearch
- **Frontend**: React.js
- **Containerization**: Docker
- **Orkestrasi**: Kubernetes
- **CI/CD**: GitHub Actions

## Instalasi

Untuk menjalankan Chubbishop secara lokal, ikuti langkah-langkah berikut:

1. **Clone Repository**:
   ```git clone https://github.com/adityars-sidqi/chubbishop.git cd chubbishop```
2. **Instalasi Dependensi**: Setiap microservice memiliki foldernya sendiri. Masuk ke masing-masing folder dan jalankan:
   ```cargo build```
3. **Konfigurasi Environment**: Buat file `.env` di setiap microservice dan sesuaikan dengan konfigurasi yang diperlukan.
4. **Menjalankan Layanan**: Anda dapat menggunakan Docker untuk menjalankan semua layanan: ```docker-compose up```

## Pengujian

Untuk menjalankan pengujian, gunakan perintah berikut di masing-masing microservice: ```cargo test```
