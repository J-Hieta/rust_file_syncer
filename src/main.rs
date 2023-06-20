use notify::{Watcher, RecursiveMode};
use std::io;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::fs;
use serde_json;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
struct FolderData {
    source_folder: String,
    destination_folder: String,
    file_name: String
}

fn main() {

    // Read the source and destination from data.json
    let json_data = fs::read_to_string("src/data.json");

    let data_result = match json_data {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Error reading data.json: {}", error);
            create_data_json();
            print!("Please restart the program");
            return;
        }
    };

    let folder_data: FolderData = serde_json::from_str(&data_result).expect("JSON was not well-formatted");
    
    // Create a channel to receive the events.
    let (sender, receiver) = channel();
    let mut watcher = notify::watcher(sender, Duration::from_secs(1)).unwrap();

    // Define a folder to watch
    watcher.watch(folder_data.source_folder, RecursiveMode::Recursive).unwrap();
    
    loop {
        match receiver.recv() {
            Ok(event) => copy_to_folder(event, folder_data.destination_folder.clone(), folder_data.file_name.clone()),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn copy_to_folder(event: notify::DebouncedEvent, destination_folder: String, new_file_name: String) {
    let source_file = match event {
        notify::DebouncedEvent::Create(ref path) => path,
        notify::DebouncedEvent::Write(ref path) => path,
        notify::DebouncedEvent::Chmod(ref path) => path,
        notify::DebouncedEvent::Remove(ref path) => path,
        notify::DebouncedEvent::Rename(ref path, _) => path,
        _ => return,
    };

    //Check if event is of type create or write
    //if it is, copy the file to destination folder
    if event == notify::DebouncedEvent::Create(source_file.clone()) || (event == notify::DebouncedEvent::Write(source_file.clone())) {
        println!("File created or modified: {:?}", source_file);
        match std::fs::copy(source_file, destination_folder.clone() + "/" + &new_file_name) {
            Ok(_) => print!("File copied succesfully!"),
            Err(error) => eprintln!("Error copying file: {}", error)
        }
    }
}

fn create_data_json() {
    // Create data.json file    
    print!("Creating data.json file...\n");

    // Get the source and destination folder from user
    let mut source_folder = String::new();
    println!("Enter the source folder path: ");
    io::stdin().read_line(&mut source_folder).expect("Failed to read line");

    let mut destination_folder = String::new();
    println!("Enter the destination folder path: ");
    io::stdin().read_line(&mut destination_folder).expect("Failed to read line");

    // Get the new file name from user
    let mut file_name = String::new();
    println!("Enter the new file name: ");
    io::stdin().read_line(&mut file_name).expect("Failed to read line");

    // Create a struct to store the data
    let data = FolderData {
        source_folder: source_folder.trim().to_string(),
        destination_folder: destination_folder.trim().to_string(),
        file_name: file_name.trim().to_string()
    };

    let json = serde_json::to_string(&data).unwrap();

    // Write the data to data.json
    let new_data_json = fs::write("src/data.json", json);
    match new_data_json {
        Ok(_) => print!("data.json file created succesfully!\n"),
        Err(error) => {
            eprintln!("Error creating data.json file: {}", error);
        }
    }

}


