// CEFApp.swift
// CEFKit - Swift bindings for CEF
//
// CEF application initialization and message loop management.

import Foundation
import CEF

// MARK: - CEFApp

/// Main entry point for CEF initialization and management.
///
/// Usage:
/// ```swift
/// // Initialize CEF at app startup
/// var settings = CEFSettings()
/// settings.cachePath = "~/.termsurf/cache"
/// try CEFApp.initialize(settings: settings)
///
/// // In your run loop, call periodically:
/// CEFApp.doMessageLoopWork()
///
/// // At app shutdown:
/// CEFApp.shutdown()
/// ```
public final class CEFApp {

    /// Whether CEF has been initialized
    public private(set) static var isInitialized = false

    // MARK: - Initialization

    /// Initialize CEF with the specified settings.
    /// Must be called once at application startup before creating any browsers.
    ///
    /// - Parameter settings: CEF initialization settings
    /// - Throws: CEFError if initialization fails
    public static func initialize(settings: CEFSettings = CEFSettings()) throws {
        guard !isInitialized else {
            throw CEFError.alreadyInitialized
        }

        // Set up main args from command line
        var mainArgs = cef_main_args_t()
        mainArgs.argc = Int32(CommandLine.argc)
        mainArgs.argv = CommandLine.unsafeArgv

        // Convert settings
        var cefSettings = settings.toCEF()

        // Initialize CEF
        let result = cef_initialize(&mainArgs, &cefSettings, nil, nil)

        if result == 0 {
            throw CEFError.initializationFailed
        }

        isInitialized = true
    }

    /// Initialize CEF with custom main args.
    /// Use this if you need to pass custom command-line arguments to CEF.
    ///
    /// - Parameters:
    ///   - args: Command-line arguments
    ///   - settings: CEF initialization settings
    /// - Throws: CEFError if initialization fails
    public static func initialize(args: [String], settings: CEFSettings = CEFSettings()) throws {
        guard !isInitialized else {
            throw CEFError.alreadyInitialized
        }

        // Convert args to C strings
        var cArgs = args.map { strdup($0) }
        defer { cArgs.forEach { free($0) } }

        var mainArgs = cef_main_args_t()
        mainArgs.argc = Int32(cArgs.count)

        try cArgs.withUnsafeMutableBufferPointer { buffer in
            mainArgs.argv = buffer.baseAddress

            var cefSettings = settings.toCEF()
            let result = cef_initialize(&mainArgs, &cefSettings, nil, nil)

            if result == 0 {
                throw CEFError.initializationFailed
            }
        }

        isInitialized = true
    }

    // MARK: - Message Loop

    /// Perform a single iteration of CEF message loop work.
    /// Call this periodically from your application's run loop.
    ///
    /// This allows CEF to process pending events without blocking.
    /// For macOS, integrate with the main run loop using a Timer or DispatchSourceTimer.
    ///
    /// Example:
    /// ```swift
    /// let timer = Timer.scheduledTimer(withTimeInterval: 1.0/60.0, repeats: true) { _ in
    ///     CEFApp.doMessageLoopWork()
    /// }
    /// ```
    public static func doMessageLoopWork() {
        guard isInitialized else { return }
        cef_do_message_loop_work()
    }

    /// Run the CEF message loop.
    /// This blocks until cef_quit_message_loop() is called.
    ///
    /// Note: For most applications, you should use doMessageLoopWork() instead
    /// to integrate with the existing run loop.
    public static func runMessageLoop() {
        guard isInitialized else { return }
        cef_run_message_loop()
    }

    /// Quit the CEF message loop.
    /// Call this to exit runMessageLoop().
    public static func quitMessageLoop() {
        guard isInitialized else { return }
        cef_quit_message_loop()
    }

    // MARK: - Shutdown

    /// Shut down CEF.
    /// Call this once at application exit, after all browsers have been closed.
    public static func shutdown() {
        guard isInitialized else { return }
        cef_shutdown()
        isInitialized = false
    }
}

// MARK: - CEFError

/// Errors that can occur during CEF operations
public enum CEFError: Error, LocalizedError {
    case alreadyInitialized
    case initializationFailed
    case notInitialized
    case browserCreationFailed

    public var errorDescription: String? {
        switch self {
        case .alreadyInitialized:
            return "CEF is already initialized"
        case .initializationFailed:
            return "CEF initialization failed"
        case .notInitialized:
            return "CEF is not initialized"
        case .browserCreationFailed:
            return "Failed to create browser"
        }
    }
}
