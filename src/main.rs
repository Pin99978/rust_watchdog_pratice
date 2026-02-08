use sysinfo::System;
use std::time::Duration; // Note: std::thread is not used anymore in async!
use axum::{Router, routing::get, extract::State, response::Html};
use tokio::net::TcpListener;
use std::sync::{Arc, Mutex};


// === [ Concept: Shared State Structure ] ===
// This struct will sit on the Heap.
// We derive `Clone` not because we copy the TV, but because Arc uses it internally? 
// No, actually Arc doesn't need T to be Clone. But our internal logic might.
#[derive(Clone)]
struct AppState {
    ram: f64,
}

// === [ Concept: Async Runtime ] ===
// #[tokio::main] is a macro that transforms async main() into a synchronous runtime starter.
// It initializes the Thread Pool (User Space Scheduler).
#[tokio::main]
async fn main() {

    // === [ Concept: Ownership & Arc<Mutex<T>> ] ===
    // 1. AppState {..} is created on Stack.
    // 2. Mutex::new(..) wraps it (adding a Lock).
    // 3. Arc::new(..) moves it to Heap and returns a Pointer.
    // Result: `state` is a smart pointer to the Heap memory. RefCount = 1.
    let state = Arc::new(Mutex::new(AppState{ ram: 0.0 }));

    // === [ Concept: Reference Counting ] ===
    // We clone the pointer (Arc), NOT the data.
    // Now RefCount = 2. Monitor thread will hold this one.
    let state_monitor = state.clone();
    
    // Now RefCount = 3. Web Server thread will hold this one.
    let state_server  = state.clone();

    // === [ Concept: Spawning Background Task ] ===
    // tokio::spawn creates a "Green Thread" (Task).
    // `move` keyword forces `state_monitor` to move ownership INTO this task.
    tokio::spawn(async move{
        let mut sys = System::new_all();
        loop {
                sys.refresh_memory();
                sys.refresh_cpu(); 
                
                // Calculate logic (CPU Bound)
                let used_percentage = (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0;
                println!("Used memory: {:.2}%", used_percentage);

                // === [ Concept: Critical Section (Write) ] ===
                {
                    // 1. Lock: Ask OS/Mutext for permission. If locked, we wait here (Block).
                    let mut guard = state_monitor.lock().unwrap(); 
                    // 2. Critical Section: We are the only one here. Safe to write.
                    guard.ram = used_percentage;
                } // 3. Scope End: Guard is dropped. Lock is automatically released! (RAII)


                // CPU Monitoring Logic
                for cpu in sys.cpus() {
                    if cpu.cpu_usage() > 50.0 {
                        println!("High CPU Usage: {:.2}%", cpu.cpu_usage());
                    }
                }

                // === [ Concept: Async Sleep ] ===
                // Non-blocking sleep. We yield the CPU back to the Runtime 
                // so it can run other tasks (like the Web Server) while we wait.
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await
            }
    });

    println!("Monitor is running in background...");

    // === [ Concept: Web Handler (Reader) ] ===
    // This function runs whenever a user visits localhost:3000
    // It borrows the Shared State from the Router.
    async fn root_handler(State(state): State<Arc<Mutex<AppState>>>)-> Html<String> {
        // === [ Concept: Critical Section (Read) ] ===
        let guard = state.lock().unwrap(); // Lock for reading
        let ram = guard.ram; // Copy the f64 value out
        // Guard dropped here, lock released ASAP.

        // HTML Response with Auto-Refresh
        let html = format!(r#"
        <html>
            <head>
                <title>RustyWatchdog</title>
                <meta http-equiv="refresh" content="1"> <!-- Auto Refresh every 1s -->
                <style>
                    body {{ font-family: sans-serif; text-align: center; padding: 50px; background: #222; color: #fff; }}
                    h1 {{ font-size: 3em; }}
                    .value {{ font-size: 5em; font-weight: bold; color: #4CAF50; }}
                </style>
            </head>
            <body>
                <h1>Current RAM Usage</h1>
                <div class="value">{:.2}%</div>
            </body>
        </html>
        "#, ram);

        Html(html)
    }

    let app = Router::new()
        .route("/", get(root_handler)) 
        .with_state(state_server); // Pass the pointer to the Router

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server is listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap(); 
}
