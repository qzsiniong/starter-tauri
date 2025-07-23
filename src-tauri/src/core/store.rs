use serde_json::json;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

pub fn _setup_store(app: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // Create a new store or load the existing one
    // this also put the store in the app's resource table
    // so your following calls `store` calls (from both rust and js)
    // will reuse the same store
    let store = app.store("settings.json")?;

    // Note that values must be serde_json::Value instances,
    // otherwise, they will not be compatible with the JavaScript bindings.
    store.set("some-key", json!({ "value": 5 }));

    // Get a value from the store.
    let value = store
        .get("some-key")
        .expect("Failed to get value from store");
    println!("{}", value); // {"value":5}

    // Remove the store from the resource table
    store.close_resource();

    Ok(())
}
