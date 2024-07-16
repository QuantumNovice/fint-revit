// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::fs;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

const DEFAULT_ADDIN: &str = "C:\\ProgramData\\Autodesk\\";
const INSTALL_DIR: &str = "C:\\Program Files\\Autodesk";
const ADDIN_PATH: &str = "\\Autodesk\\Revit\\Addins\\";

#[tauri::command]
fn get_installed_revit_dirs() -> Result<Vec<String>, String> {
    // Read the directory
    let entries = fs::read_dir(INSTALL_DIR)
        .map_err(|e| format!("Failed to read directory {}: {}", INSTALL_DIR, e))?;

    // Collect directory names starting with "Revit"
    let revit_dirs: Vec<String> = entries
        .filter_map(Result::ok)
        .filter_map(|entry| {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.starts_with("Revit") {
                    Some(file_name.to_string())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    Ok(revit_dirs)
}

#[tauri::command]
fn get_installed_addins() -> Result<Vec<String>, String> {
    let found_revit = get_installed_revit_dirs();
    println!("Found Revit add-ins: {:?}", found_revit);

    let mut addin_dir: String = String::from("C:\\Users\\us2er\\AppData\\Roaming\\Autodesk\\Revit\\Addins\\2024"); // place holder value
    match found_revit {
        Ok(addins) => {
            println!("Found Revit add-ins: {:?}", addins);

            let last_elements: Vec<_> = addins
                .iter()
                .map(|addin| addin.split_whitespace().last().unwrap_or(""))
                .collect();

            println!(
                "Last elements of each add-in after splitting by space: {:?}",
                last_elements
            );

            let revit_ver = last_elements
                .iter()
                .filter_map(|s| s.parse::<i32>().ok())
                .nth(0);

            if let Some(version) = revit_ver {
                println!("Revit version: {}", version);
            } else {
                println!("No valid Revit version found.");
            }

            
            if let Some(version) = revit_ver {
                let version_str = version.to_string();
                println!("Revit version as string: {}", version_str);
                // print found revit to console

                addin_dir = match env::var("APPDATA") {
                    Ok(val) => {
                        let mut val_as_string = val.clone(); // Clone the String to make it mutable
                        val_as_string.push_str(ADDIN_PATH); // Push the ADDIN_PATH to the mutable String
                        val_as_string.push_str(&version_str);
                        val_as_string // Return the resulting String
                    }
                    Err(_) => String::from("C:\\path\\to\\default\\addins"), // Create a new String for the default path
                };

                println!("Addin directory: {}", addin_dir);
            } else {
                println!("No valid Revit version found.");
            }
        }
        Err(err) => {
            println!("Error finding Revit add-ins: {}", err);
        }
    }

    // Read the directory
    let entries = fs::read_dir(&addin_dir)
        .map_err(|e| format!("Failed to read directory {}: {}", &addin_dir, e))?;

    // Collect directory names starting with "Revit"
    let revit_dirs: Vec<String> = entries
        .filter_map(Result::ok)
        .filter_map(|entry| {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".addin") {
                    Some(file_name.to_string())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    Ok(revit_dirs)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_installed_revit_dirs,
            get_installed_addins
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
