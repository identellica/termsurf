import Foundation
import os

private let logger = Logger(subsystem: "com.termsurf", category: "SocketConnection")

/// Handles a single client connection to the Unix domain socket.
/// Reads newline-delimited JSON requests, processes them, and sends responses.
class SocketConnection {
    private let fileHandle: FileHandle
    private let id: String
    private var buffer = Data()
    private let encoder = JSONEncoder()
    private let decoder = JSONDecoder()

    /// Callbacks for connection lifecycle
    var onDisconnect: (() -> Void)?

    /// Active subscriptions for events (request IDs waiting for events)
    private var eventSubscriptions = Set<String>()

    init(fileDescriptor: Int32) {
        self.fileHandle = FileHandle(fileDescriptor: fileDescriptor, closeOnDealloc: true)
        self.id = UUID().uuidString.prefix(8).description
        logger.info("New connection: \(self.id)")
    }

    deinit {
        logger.info("Connection closed: \(self.id)")
    }

    /// Start reading from the connection
    func start() {
        readLoop()
    }

    /// Send an event to this connection if it's subscribed
    func sendEvent(_ event: TermsurfEvent) {
        guard eventSubscriptions.contains(event.id) else { return }

        do {
            let data = try encoder.encode(event)
            try send(data)
        } catch {
            logger.error("Failed to send event: \(error.localizedDescription)")
        }
    }

    /// Subscribe this connection to events for a request
    func subscribeToEvents(requestId: String) {
        eventSubscriptions.insert(requestId)
    }

    /// Close the connection
    func close() {
        try? fileHandle.close()
        onDisconnect?()
    }

    // MARK: - Private

    private func readLoop() {
        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            guard let self = self else { return }

            while true {
                do {
                    guard let data = try self.fileHandle.availableData(), !data.isEmpty else {
                        // Connection closed
                        logger.info("Connection \(self.id) closed by client")
                        DispatchQueue.main.async {
                            self.onDisconnect?()
                        }
                        return
                    }

                    self.buffer.append(data)
                    self.processBuffer()
                } catch {
                    logger.error("Read error on connection \(self.id): \(error.localizedDescription)")
                    DispatchQueue.main.async {
                        self.onDisconnect?()
                    }
                    return
                }
            }
        }
    }

    private func processBuffer() {
        // Process all complete lines (newline-delimited JSON)
        while let newlineIndex = buffer.firstIndex(of: UInt8(ascii: "\n")) {
            let lineData = buffer.prefix(upTo: newlineIndex)
            buffer = buffer.suffix(from: buffer.index(after: newlineIndex))

            if lineData.isEmpty { continue }

            processLine(Data(lineData))
        }
    }

    private func processLine(_ data: Data) {
        do {
            let request = try decoder.decode(TermsurfRequest.self, from: data)
            logger.debug("Received request: \(request.action) id=\(request.id)")

            // Handle on main thread for UI operations
            DispatchQueue.main.async { [weak self] in
                guard let self = self else { return }
                let response = CommandHandler.shared.handle(request)
                self.sendResponse(response)
            }
        } catch {
            logger.error("Failed to decode request: \(error.localizedDescription)")
            // Send error response
            let errorResponse = TermsurfResponse.error(id: "unknown", message: "Invalid JSON: \(error.localizedDescription)")
            sendResponse(errorResponse)
        }
    }

    private func sendResponse(_ response: TermsurfResponse) {
        do {
            let data = try encoder.encode(response)
            try send(data)
        } catch {
            logger.error("Failed to send response: \(error.localizedDescription)")
        }
    }

    private func send(_ data: Data) throws {
        var dataWithNewline = data
        dataWithNewline.append(UInt8(ascii: "\n"))
        try fileHandle.write(contentsOf: dataWithNewline)
    }
}

// MARK: - FileHandle extension for better error handling

private extension FileHandle {
    func availableData() throws -> Data? {
        // Use read() for better control
        let bufferSize = 4096
        var buffer = [UInt8](repeating: 0, count: bufferSize)

        let bytesRead = Darwin.read(self.fileDescriptor, &buffer, bufferSize)

        if bytesRead < 0 {
            throw NSError(domain: POSIXError.errorDomain, code: Int(errno))
        } else if bytesRead == 0 {
            return nil // EOF
        } else {
            return Data(buffer.prefix(bytesRead))
        }
    }
}
