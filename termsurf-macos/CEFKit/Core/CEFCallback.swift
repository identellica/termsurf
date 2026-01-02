// CEFCallback.swift
// CEFKit - Swift bindings for CEF
//
// Callback marshalling pattern for creating CEF handler objects from Swift.
// CEF handlers are C structs with function pointers. We need to:
// 1. Create the struct with proper size and ref counting
// 2. Store a reference to the Swift handler object
// 3. Marshal calls from C function pointers to Swift methods

import Foundation
import CEF

// MARK: - Handler Context

/// Storage for the Swift handler object associated with a CEF handler struct.
/// We store a raw pointer to this in the CEF struct's user data.
final class CEFHandlerContext<Handler: AnyObject> {
    weak var handler: Handler?
    var refCount: Int32 = 1

    init(_ handler: Handler) {
        self.handler = handler
    }
}

// MARK: - Reference Counting Callbacks

/// Standard add_ref implementation for client-allocated CEF structs
func cefAddRef<T>(_ ptr: UnsafeMutablePointer<T>?) {
    guard let ptr = ptr else { return }
    let context = getHandlerContext(from: ptr) as CEFHandlerContext<AnyObject>?
    if let ctx = context {
        OSAtomicIncrement32(&ctx.refCount)
    }
}

/// Standard release implementation for client-allocated CEF structs
/// Returns 1 if the object was deleted, 0 otherwise
func cefRelease<T>(_ ptr: UnsafeMutablePointer<T>?) -> Int32 {
    guard let ptr = ptr else { return 0 }
    let context = getHandlerContext(from: ptr) as CEFHandlerContext<AnyObject>?
    guard let ctx = context else { return 0 }

    let newCount = OSAtomicDecrement32(&ctx.refCount)
    if newCount == 0 {
        // Clean up
        Unmanaged.passUnretained(ctx).release()
        ptr.deallocate()
        return 1
    }
    return 0
}

/// Standard has_one_ref implementation
func cefHasOneRef<T>(_ ptr: UnsafeMutablePointer<T>?) -> Int32 {
    guard let ptr = ptr else { return 0 }
    let context = getHandlerContext(from: ptr) as CEFHandlerContext<AnyObject>?
    return context?.refCount == 1 ? 1 : 0
}

/// Standard has_at_least_one_ref implementation
func cefHasAtLeastOneRef<T>(_ ptr: UnsafeMutablePointer<T>?) -> Int32 {
    guard let ptr = ptr else { return 0 }
    let context = getHandlerContext(from: ptr) as CEFHandlerContext<AnyObject>?
    return (context?.refCount ?? 0) >= 1 ? 1 : 0
}

// MARK: - Context Storage

/// Store handler context in the CEF struct.
/// We use the memory immediately after the base struct for our context pointer.
/// This is a common pattern in C APIs - store user data alongside the struct.
func setHandlerContext<T, Handler: AnyObject>(_ context: CEFHandlerContext<Handler>, on ptr: UnsafeMutablePointer<T>) {
    // Store retained reference to context
    let retained = Unmanaged.passRetained(context)
    let contextPtr = retained.toOpaque()

    // Store at the end of the struct (we allocate extra space for this)
    ptr.withMemoryRebound(to: UInt8.self, capacity: MemoryLayout<T>.size + MemoryLayout<UnsafeRawPointer>.size) { bytePtr in
        let contextLocation = bytePtr.advanced(by: MemoryLayout<T>.size)
        contextLocation.withMemoryRebound(to: UnsafeMutableRawPointer?.self, capacity: 1) { storage in
            storage.pointee = contextPtr
        }
    }
}

/// Retrieve handler context from a CEF struct
func getHandlerContext<T, Handler: AnyObject>(from ptr: UnsafeMutablePointer<T>) -> CEFHandlerContext<Handler>? {
    return ptr.withMemoryRebound(to: UInt8.self, capacity: MemoryLayout<T>.size + MemoryLayout<UnsafeRawPointer>.size) { bytePtr in
        let contextLocation = bytePtr.advanced(by: MemoryLayout<T>.size)
        return contextLocation.withMemoryRebound(to: UnsafeMutableRawPointer?.self, capacity: 1) { storage in
            guard let contextPtr = storage.pointee else { return nil }
            return Unmanaged<CEFHandlerContext<Handler>>.fromOpaque(contextPtr).takeUnretainedValue()
        }
    }
}

/// Get the Swift handler object from a CEF struct pointer
func getHandler<T, Handler: AnyObject>(from ptr: UnsafeMutablePointer<T>?) -> Handler? {
    guard let ptr = ptr else { return nil }
    let context: CEFHandlerContext<Handler>? = getHandlerContext(from: ptr)
    return context?.handler
}

// MARK: - Handler Struct Allocation

/// Allocate a CEF handler struct with space for our context pointer.
/// The struct is zero-initialized with the size field set.
func allocateHandler<T>(_ type: T.Type) -> UnsafeMutablePointer<T> {
    // Allocate struct + space for context pointer
    let totalSize = MemoryLayout<T>.size + MemoryLayout<UnsafeRawPointer>.size
    let ptr = UnsafeMutableRawPointer.allocate(byteCount: totalSize, alignment: MemoryLayout<T>.alignment)

    // Zero initialize
    memset(ptr, 0, totalSize)

    let typedPtr = ptr.bindMemory(to: T.self, capacity: 1)

    // Set size field (first field in all CEF structs)
    typedPtr.withMemoryRebound(to: Int.self, capacity: 1) { sizePtr in
        sizePtr.pointee = MemoryLayout<T>.size
    }

    return typedPtr
}

// MARK: - Base Ref Counted Setup

/// Set up the base reference counting callbacks on a CEF struct.
/// Call this after allocating the handler struct.
func setupBaseRefCounted<T>(_ ptr: UnsafeMutablePointer<T>) {
    ptr.withMemoryRebound(to: cef_base_ref_counted_t.self, capacity: 1) { basePtr in
        basePtr.pointee.size = MemoryLayout<T>.size

        // Set up function pointers for ref counting
        basePtr.pointee.add_ref = { ptr in
            cefAddRef(ptr)
        }
        basePtr.pointee.release = { ptr in
            return cefRelease(ptr)
        }
        basePtr.pointee.has_one_ref = { ptr in
            return cefHasOneRef(ptr)
        }
        basePtr.pointee.has_at_least_one_ref = { ptr in
            return cefHasAtLeastOneRef(ptr)
        }
    }
}
