use sysinfo::System;
use crate::helpers::execute_update_max_refreshrate;


// We follow the convention that the user installs the native Wuthering Waves application from the App Store,LocalStorage is then stored in users file path
// Example path:
// /Users/UserName/Library/Containers/com.kurogame.wutheringwaves.global/Data/Library/Client/Saved/LocalStorage/LocalStorage.db
pub fn get_local_storage_filepath() -> String {
    // Get the path to the LocalStorage file for Wuthering Waves
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| "/Users/Shared".to_string());
    let local_storage_path = format!("{}/Library/Containers/com.kurogame.wutheringwaves.global/Data/Library/Client/Saved/LocalStorage/LocalStorage.db", home_dir);
    return  local_storage_path
}

pub fn execute_unlock_fps() -> Result<(), String> {
    
    // Check if Wuthering Waves is running and not continue if it is
    let is_running = check_wuwa_running();
    if is_running == true {
        return Err("Wuthering Waves is currently running. Please close the game before proceeding.".to_string());
    }
    
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
            Ok(())
        },
        Err(e) => {
            Err(format!("Failed to update the LocalStorage file: {}", e))
        }
    }
}

// Checks if Wuthering Waves is currently running via process
// Using sysinfo create to potentially bring forward Windows support in future
pub fn check_wuwa_running() -> bool {
    // Client-Mac-Shipping corresponds to Wuthering Waves on MacOS
    let process_name = "Client-Mac-Shipping".to_string();
    
    // initialise sysinfo create
    let mut system = System::new_all();
    system.refresh_all();

    for (_, process) in system.processes() {
        
        if process.name().to_str().unwrap().to_string()  == process_name {
            return true; // Wuthering Waves is running
        }
    }
    return  false; // Wuthering Waves not running
}

// We can delete the LocalStorage file if anything catastrophic happens
// This will let Wuthering Waves recreate the file on next launch
pub fn delete_localstorage_file(file_path: &String) -> Result<(), bool> {
    // Attempt to delete the LocalStorage file
    match std::fs::remove_file(file_path) {
        Ok(_) => Ok(()), 
        Err(e) => {
            Err(false) 
        }
    }
    
}



