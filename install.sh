#!/bin/bash

# Bash script to build the Trade Management System project and configure SQLite database using Diesel.

# Function to display messages in green color.
print_success() {
    echo -e "\033[0;32m$1\033[0m"
}

# Function to display messages in red color.
print_error() {
    echo -e "\033[0;31m$1\033[0m"
}

# Check and install Git if not installed.
if ! command -v git &> /dev/null; then
    print_error "Git is not installed. Installing Git..."
    sudo apt-get update
    sudo apt-get install git
    print_success "Git has been installed."
fi

# Check and install Rust if not installed.
if ! command -v rustc &> /dev/null; then
    print_error "Rust is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
    print_success "Rust has been installed."
fi

# Check and install Diesel CLI if not installed.
if ! command -v diesel &> /dev/null; then
    print_error "Diesel CLI is not installed. Installing Diesel CLI..."
    cargo install diesel_cli --no-default-features --features sqlite
    print_success "Diesel CLI has been installed."
fi

# Clone the repository.
print_success "Cloning the repository..."
git clone https://github.com/dalmasjunior/TradeManagementSystem.git
cd TradeManagementSystem

# Build the project.
print_success "Building the project..."
cargo build

# Set up the SQLite database with Diesel.
print_success "Configuring the SQLite database..."
diesel setup

# Run migrations.
print_success "Running migrations..."
diesel migration run

# Provide instructions to run the application.
print_success "Project setup is complete. You can now run the application using 'cargo run'."
print_success "The server will start on port 9000. Open your web browser or Postman and visit: http://localhost:9000"

# Exit the script.
exit 0
