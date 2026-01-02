// CEFBase.swift
// CEFKit - Swift bindings for CEF
//
// Reference counting wrapper for CEF objects.
// CEF uses reference counting for memory management - all objects inherit from cef_base_ref_counted_t.

import Foundation
import CEF

// MARK: - CEFRefCounted Protocol

/// Protocol for CEF objects that use reference counting
protocol CEFRefCounted {
    associatedtype Wrapped
    var cefObject: UnsafeMutablePointer<Wrapped> { get }
    func addRef()
    func release() -> Bool
}

// MARK: - CEFBase Wrapper

/// Generic wrapper for CEF reference-counted objects.
/// Automatically releases the object when the wrapper is deallocated.
///
/// Usage:
/// ```swift
/// let browser = CEFBase<cef_browser_t>(ptr)
/// browser.cefObject.pointee.go_back(browser.cefObject)
/// // Automatically released when browser goes out of scope
/// ```
final class CEFBase<T>: CEFRefCounted {
    typealias Wrapped = T

    let cefObject: UnsafeMutablePointer<T>

    /// Initialize with a CEF object pointer.
    /// Takes ownership of one reference (does not add a new ref).
    /// - Parameter ptr: Pointer to the CEF object
    init?(_ ptr: UnsafeMutablePointer<T>?) {
        guard let ptr = ptr else { return nil }
        self.cefObject = ptr
    }

    /// Initialize with a CEF object pointer, adding a reference.
    /// Use this when you're receiving a pointer that you don't own.
    /// - Parameter ptr: Pointer to the CEF object
    /// - Parameter addRef: Pass true to add a reference
    init?(_ ptr: UnsafeMutablePointer<T>?, addRef: Bool) {
        guard let ptr = ptr else { return nil }
        self.cefObject = ptr
        if addRef {
            self.addRef()
        }
    }

    deinit {
        _ = release()
    }

    /// Add a reference to the CEF object
    func addRef() {
        withBasePointer { base in
            base.pointee.add_ref?(base)
        }
    }

    /// Release a reference to the CEF object
    /// - Returns: true if the object was deleted (ref count reached 0)
    @discardableResult
    func release() -> Bool {
        return withBasePointer { base in
            (base.pointee.release?(base) ?? 0) != 0
        }
    }

    /// Check if this object has only one reference
    func hasOneRef() -> Bool {
        return withBasePointer { base in
            (base.pointee.has_one_ref?(base) ?? 0) != 0
        }
    }

    /// Check if this object has at least one reference
    func hasAtLeastOneRef() -> Bool {
        return withBasePointer { base in
            (base.pointee.has_at_least_one_ref?(base) ?? 0) != 0
        }
    }

    /// Get a pointer to the base ref-counted struct
    private func withBasePointer<R>(_ body: (UnsafeMutablePointer<cef_base_ref_counted_t>) -> R) -> R {
        return cefObject.withMemoryRebound(to: cef_base_ref_counted_t.self, capacity: 1, body)
    }

    /// Pass to CEF API, adding a reference.
    /// Use when passing to a CEF function that will store the reference.
    func toCEF() -> UnsafeMutablePointer<T> {
        addRef()
        return cefObject
    }
}

// MARK: - Struct Size Initialization

/// Helper to initialize a CEF struct with its size field set correctly.
/// All CEF structs have a `size` field that must be set before use.
///
/// Usage:
/// ```swift
/// var settings = initCefStruct(cef_settings_t.self)
/// settings.no_sandbox = 1
/// ```
func initCefStruct<T>(_ type: T.Type) -> T {
    let instance = UnsafeMutablePointer<T>.allocate(capacity: 1)
    defer { instance.deallocate() }

    // Zero-initialize
    memset(instance, 0, MemoryLayout<T>.size)

    // Set size field (first field in all CEF structs)
    instance.withMemoryRebound(to: Int.self, capacity: 1) { sizePtr in
        sizePtr.pointee = MemoryLayout<T>.size
    }

    return instance.pointee
}

/// Initialize a CEF struct in place
func initCefStructInPlace<T>(_ ptr: UnsafeMutablePointer<T>) {
    memset(ptr, 0, MemoryLayout<T>.size)
    ptr.withMemoryRebound(to: Int.self, capacity: 1) { sizePtr in
        sizePtr.pointee = MemoryLayout<T>.size
    }
}
