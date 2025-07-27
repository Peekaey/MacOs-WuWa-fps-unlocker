use crate::helpers::execute_update_max_refreshrate;

// We follow the convention that the user installs the native Wuthering Waves application from the App Store,LocalStorage is then stored in users file path
// Example path:
// /Users/UserName/Library/Containers/com.kurogame.wutheringwaves.global/Data/Library/Client/Saved/LocalStorage/LocalStorage.db
pub fn get_local_storage_filepath() -> String {
    // Get the path to the LocalStorage file for Wuthering Waves
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| "/Users/Shared".to_string());
    let local_storage_path = format!("{}/Library/Containers/com.kurogame.wutheringwaves.global/Data/Library/Client/Saved/LocalStorage/LocalStorage.db", home_dir);
    
    // After building path, we check if its valid
    
    return  local_storage_path
}

pub fn execute_unlock_fps() -> Result<(), String> {
    // Get the path to the LocalStorage file
    let local_storage_filepath = get_local_storage_filepath();

    // Check if the LocalStorage file exists
    if !std::path::Path::new(&local_storage_filepath).exists() {
        return Err("LocalStorage file does not exist. Please ensure that you have run the game at least once.".to_string());
    }

    // Now that we have confirmed the db file exists, we can proceed with updating the configuration
    let update_result = execute_update_max_refreshrate(&local_storage_filepath);

    return match update_result {
        Ok(_) => {
            println!("Successfully updated the LocalStorage file to set the maximum frame rate to 120 FPS.");
            Ok(())
        },
        Err(e) => {
            eprintln!("Failed to update the LocalStorage file: {}", e);
            Err(format!("Failed to update the LocalStorage file: {}", e))
        }
    }
}



