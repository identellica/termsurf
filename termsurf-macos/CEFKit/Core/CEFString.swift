// CEFString.swift
// CEFKit - Swift bindings for CEF
//
// String conversion utilities between Swift String and CEF's UTF-16 strings.

import Foundation
import CEF

// MARK: - cef_string_t → String

/// Convert a CEF string to a Swift String
func cefStringToSwift(_ cefStr: cef_string_t) -> String {
    guard cefStr.length > 0, let str = cefStr.str else {
        return ""
    }
    return String(utf16CodeUnits: str, count: cefStr.length)
}

/// Convert a CEF string pointer to a Swift String
func cefStringToSwift(_ cefStrPtr: UnsafePointer<cef_string_t>?) -> String {
    guard let ptr = cefStrPtr else { return "" }
    return cefStringToSwift(ptr.pointee)
}

// MARK: - String → cef_string_t

/// Set a CEF string from a Swift String
/// - Parameters:
///   - string: The Swift string to convert
///   - cefString: Pointer to the CEF string to set
func setCefString(_ string: String, to cefString: UnsafeMutablePointer<cef_string_t>) {
    let utf16 = Array(string.utf16)
    _ = utf16.withUnsafeBufferPointer { buffer in
        cef_string_utf16_set(buffer.baseAddress, buffer.count, cefString, 1)
    }
}

/// Create a new CEF string from a Swift String
/// - Parameter string: The Swift string to convert
/// - Returns: A newly allocated CEF string (caller must free with cef_string_userfree_utf16_free)
func createCefString(_ string: String) -> cef_string_userfree_utf16_t? {
    guard let cefStr = cef_string_userfree_utf16_alloc() else {
        return nil
    }
    setCefString(string, to: cefStr)
    return cefStr
}

/// Free a CEF string that was allocated with createCefString
func freeCefString(_ cefStr: cef_string_userfree_utf16_t?) {
    if let str = cefStr {
        cef_string_userfree_utf16_free(str)
    }
}

// MARK: - Scoped String Helper

/// Execute a closure with a temporary CEF string, automatically cleaning up afterward
/// - Parameters:
///   - string: The Swift string to convert
///   - body: Closure that receives a pointer to the CEF string
/// - Returns: The result of the closure
func withCefString<T>(_ string: String, _ body: (UnsafePointer<cef_string_t>) throws -> T) rethrows -> T {
    var cefStr = cef_string_t()
    setCefString(string, to: &cefStr)
    defer { cef_string_utf16_clear(&cefStr) }
    return try body(&cefStr)
}

/// Execute a closure with a mutable temporary CEF string
func withMutableCefString<T>(_ string: String, _ body: (UnsafeMutablePointer<cef_string_t>) throws -> T) rethrows -> T {
    var cefStr = cef_string_t()
    setCefString(string, to: &cefStr)
    defer { cef_string_utf16_clear(&cefStr) }
    return try body(&cefStr)
}
