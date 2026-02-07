---
description: 
---

# RustyWatchdog Project Workflow: From CLI to Async Web Service

## Phase 1: The Foundation (Hello System)
**目標**：建立專案，引入依賴，並成功讀取一次系統記憶體與 CPU 資訊。
**實作功能**：
1. `cargo new` 初始化專案。
2. 設定 `Cargo.toml` 引入 `sysinfo`。
3. 在 `main.rs` 中獲取並列印記憶體總量與使用量。
**關鍵學習點 (Rust Concepts)**：
- **Ownership (所有權)**：為什麼 `sys` 變數必須是 `mut`？
- **Type System (型別系統)**：`u64` vs `f64`，以及如何安全地進行轉型 (Type Casting) 以避免 Overflow。
- **Crates & Modules**：如何使用外部生態系。

## Phase 2: The Loop & Logic (Simulating a Daemon)
**目標**：讓程式變成一個持續運行的監控 Daemon，並加入警報邏輯。
**實作功能**：
1. 實作 `loop` 迴圈與 `thread::sleep`。
2. 加入邏輯判斷：如果 Memory > 80%，印出警告訊息 (Stderr)。
3. 使用 `struct` 來封裝監控的數據 (Data Modeling)。
**關鍵學習點 (Rust Concepts)**：
- **Stack vs Heap**：你的數據現在存在哪裡？
- **Control Flow**：`loop` 與 `break` 的機制。
- **OS Interaction**：`thread::sleep` 對作業系統排程器 (Scheduler) 意味著什麼？(Context Switch)。

## Phase 3: The Transformation (CLI to Web Server)
**目標**：廢除 CLI 輸出，改用 HTTP 介面回傳 JSON 格式的數據。
**實作功能**：
1. 引入 `axum` (Web Framework) 與 `tokio` (Async Runtime)。
2. 將同步的 `main` 改寫為 `#[tokio::main]` 非同步函數。
3. 建立一個 `/metrics` API，回傳系統數據。
**關鍵學習點 (Rust Concepts)**：
- **Async/Await**：為什麼系統程式需要非同步？它跟多執行緒 (Multi-threading) 有什麼不同？
- **Runtime**：Tokio 是如何作為一個 User-space scheduler 運作的？

## Phase 4: Shared State & Safety (Handling Concurrency)
**目標**：解決「Web Server 執行緒」與「系統監控執行緒」如何安全地存取同一份數據。
**實作功能**：
1. 建立一個背景 Task 定期更新系統數據。
2. 使用 `Arc` (Atomic Reference Counting) 和 `Mutex` (Mutual Exclusion) 來在多個執行緒間共享系統狀態。
3. 確保讀取 API 時，不會因為寫入操作而產生 Data Race。
**關鍵學習點 (Rust Concepts)**：
- **Thread Safety**：什麼是 Send 和 Sync trait？
- **Memory Safety**：`Arc` 如何確保記憶體不會被過早釋放？`Mutex` 如何防止競爭條件 (Race Condition)？
- **Interior Mutability**：如何在不可變的引用中修改數據？