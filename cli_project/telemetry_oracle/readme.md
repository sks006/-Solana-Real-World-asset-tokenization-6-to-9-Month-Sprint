# 🏎️ Telemetry Oracle: RWA Valuation Engine

**A high-performance Local Price Oracle prototype.** This system bridges the gap between static asset registries and dynamic global markets—essential for **Real-World Asset (RWA)** tokenization on high-throughput chains like **Solana**.

## 🛠️ The Engineering Specification

In a production DeFi environment, "Purchase Price" is a historical artifact. "Market Value" is the only metric that matters for collateralization and lending.

### ⚡ Solving "Turbo Lag" (Asynchronous I/O)

Network requests are slow. If the engine stalls waiting for an API response, the system fails.

* **The Compiler Invariant:** We use the `tokio` runtime to manage non-blocking I/O.
* **Logical Journey:** While a request is in flight (The Radio Tower), the CPU (The ECU) remains free to handle UI tasks or local computations.

### 🛡️ The Schema Contract (Zero-Trust Serialization)

External APIs are untrusted. Their data must be validated before entering the engine block.

* **The Rule:** We enforce a strict contract via `serde`. If the API response deviates from our defined `struct`, the data is rejected at the boundary.
* **The Trace:** JSON Byte-stream ➡️ `serde` Deserializer ➡️ Type-Safe Rust Struct.

### 💾 Fault-Tolerant Persistence (The LKG Cache)

DeFi protocols cannot stop because an internet connection flickers.

* **The Rule:** Implement **Last Known Good (LKG)** caching.
* **Exit Strategy:** If the Oracle feed (API) returns an error, the system automatically falls back to `cache.db`, providing the most recent available valuation with a "Stale Data" warning.

---

## 📂 System Architecture

The project is structured to separate the **Telemetry (Client)** from the **Calculations (Engine)**.

```text
telemetry_oracle/
├── Cargo.toml          # The Spec Sheet (Tokio, Reqwest, Serde)
├── .env                # The Fuel Grade (API Keys & Secrets)
├── src/
│   ├── main.rs         # 🔑 The Ignition: Async entry point
│   ├── lib.rs          # 🏛️ The Chassis: Public module registry
│   ├── errors.rs       # ⚠️ Check Engine Light: Custom Error Enums
│   │
│   ├── client/         # 📡 THE RADIO TOWER (Networking)
│   │   └── api.rs      #    - Reqwest implementation for Price Feeds
│   │
│   ├── engine/         # ⚙️ THE POWERTRAIN (Logic)
│   │   ├── model.rs    #    - Asset & Rate Data Structures
│   │   ├── storage.rs  #    - Persistence (LKG Cache Logic)
│   │   └── calculator.rs #   - Cross-currency valuation engine
│   │
│   └── commands/       # 🕹️ THE DASHBOARD (User Interface)
│       ├── add.rs      #    - Registering assets with currency tags
│       └── list.rs     #    - Real-time Oracle-driven valuation
│
└── cache.db            # 💾 Local storage for last known market rates

```

---

## 🚀 Why This Matters for Solana RWA

This project serves as an **Off-Chain Worker** prototype.

In a Solana context, you cannot fetch HTTP data directly inside an On-Chain program (Smart Contract). You need an **Oracle**—an off-chain service that fetches "real-world" telemetry and submits it to the cluster. This CLI demonstrates the exact logic required to:

1. Verify external data.
2. Normalize values across different denomination (USD/EUR).
3. Handle network latency asynchronously.

---

## 🏁 How to Drive

1. **Configure Fuel:** Add your API Key to the `.env` file.
2. **Add Asset:** `cargo run -- add --name "Ferrari GTO" --amount 5000000 --currency USD`
3. **Check Valuation:** `cargo run -- list --target EUR`






