// main.swift
import Foundation

// Declare the Rust function
@_silgen_name("callRustFunction") func callRustFunction()

// Swift function to call the Rust function
func callRust() {
    callRustFunction()
}

// Call the Rust function
callRust()
