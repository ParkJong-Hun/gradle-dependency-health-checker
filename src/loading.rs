/*
 * Copyright (c) 2025 ParkJong-Hun
 *
 * Licensed under the MIT License.
 * See LICENSE file in the project root for full license information.
 */

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;
use colored::*;

pub struct LoadingSpinner {
    running: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl LoadingSpinner {
    pub fn new(message: &str) -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = Arc::clone(&running);
        let message = message.to_string();
        
        let handle = thread::spawn(move || {
            let spinner_chars = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
            let mut idx = 0;
            
            // Hide cursor
            print!("\x1B[?25l");
            
            while running_clone.load(Ordering::Relaxed) {
                print!("\r{} {}...", spinner_chars[idx].blue().bold(), message.bright_white());
                std::io::Write::flush(&mut std::io::stdout()).unwrap_or(());
                
                idx = (idx + 1) % spinner_chars.len();
                thread::sleep(Duration::from_millis(100));
            }
            
            // Clear the line and show cursor
            print!("\r\x1B[K");
            print!("\x1B[?25h");
            std::io::Write::flush(&mut std::io::stdout()).unwrap_or(());
        });
        
        Self {
            running,
            handle: Some(handle),
        }
    }
    
    pub fn finish(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
    
    pub fn finish_with_message(&mut self, message: &str) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
        println!("{}", message);
    }
}

impl Drop for LoadingSpinner {
    fn drop(&mut self) {
        self.finish();
    }
}

pub struct ProgressBar {
    running: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
    message: String,
}

impl ProgressBar {
    pub fn new(message: &str) -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = Arc::clone(&running);
        let message = message.to_string();
        let message_clone = message.clone();
        
        let handle = thread::spawn(move || {
            let mut dots = 0;
            
            // Hide cursor
            print!("\x1B[?25l");
            
            while running_clone.load(Ordering::Relaxed) {
                let dots_str = ".".repeat(dots % 4);
                let padding = " ".repeat(3 - (dots % 4));
                
                print!("\r🔍 {}{}{}", message_clone.bright_white(), dots_str.blue(), padding);
                std::io::Write::flush(&mut std::io::stdout()).unwrap_or(());
                
                dots += 1;
                thread::sleep(Duration::from_millis(300));
            }
            
            // Clear the line and show cursor
            print!("\r\x1B[K");
            print!("\x1B[?25h");
            std::io::Write::flush(&mut std::io::stdout()).unwrap_or(());
        });
        
        Self {
            running,
            handle: Some(handle),
            message,
        }
    }
    
    pub fn finish(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
    
    pub fn finish_with_message(&mut self, message: &str) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
        println!("{}", message);
    }
}

impl Drop for ProgressBar {
    fn drop(&mut self) {
        self.finish();
    }
}

/// Simple loading indicator for file operations
pub fn with_loading<F, R>(message: &str, operation: F) -> R
where 
    F: FnOnce() -> R,
{
    let mut spinner = LoadingSpinner::new(message);
    let result = operation();
    spinner.finish();
    result
}

/// Progress indicator with success/error handling
pub fn with_progress<F, R, E>(message: &str, operation: F) -> Result<R, E>
where 
    F: FnOnce() -> Result<R, E>,
{
    let mut progress = ProgressBar::new(message);
    let result = operation();
    
    match &result {
        Ok(_) => progress.finish_with_message(&format!("✅ {} completed", message)),
        Err(_) => progress.finish_with_message(&format!("❌ {} failed", message)),
    }
    
    result
}