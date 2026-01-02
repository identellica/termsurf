import Foundation
import AppKit
import os

private let logger = Logger(subsystem: "com.termsurf", category: "SocketServer")

/// Unix domain socket server for CLI-to-app communication.
/// Creates a socket at `/tmp/termsurf-{pid}.sock` and listens for connections.
class SocketServer {
    static let shared = SocketServer()

    /// The socket path, available after start() is called
    private(set) var socketPath: String?

    private var serverSocket: Int32 = -1
    private var isRunning = false
    private var connections = [String: SocketConnection]()
    private let connectionLock = NSLock()

    private init() {
        // Register for app termination to clean up
        NotificationCenter.default.addObserver(
            self,
            selector: #selector(applicationWillTerminate),
            name: NSApplication.willTerminateNotification,
            object: nil
        )
    }

    deinit {
        stop()
    }

    /// Start the socket server
    func start() {
        guard !isRunning else {
            logger.warning("Socket server already running")
            return
        }

        let pid = ProcessInfo.processInfo.processIdentifier
        let path = "/tmp/termsurf-\(pid).sock"

        // Remove existing socket file if present
        unlink(path)

        // Create socket
        serverSocket = socket(AF_UNIX, SOCK_STREAM, 0)
        guard serverSocket >= 0 else {
            logger.error("Failed to create socket: \(String(cString: strerror(errno)))")
            return
        }

        // Bind to path
        var addr = sockaddr_un()
        addr.sun_family = sa_family_t(AF_UNIX)

        // Copy path to sun_path using withCString
        let bindResult = path.withCString { pathCString in
            // Copy the path string into sun_path
            withUnsafeMutableBytes(of: &addr.sun_path) { sunPathBuffer in
                let maxLen = sunPathBuffer.count
                var i = 0
                while i < maxLen - 1, pathCString[i] != 0 {
                    sunPathBuffer[i] = UInt8(bitPattern: pathCString[i])
                    i += 1
                }
                sunPathBuffer[i] = 0 // null terminate
            }

            return withUnsafePointer(to: &addr) { addrPtr in
                addrPtr.withMemoryRebound(to: sockaddr.self, capacity: 1) { sockaddrPtr in
                    Darwin.bind(serverSocket, sockaddrPtr, socklen_t(MemoryLayout<sockaddr_un>.size))
                }
            }
        }

        guard bindResult == 0 else {
            logger.error("Failed to bind socket: \(String(cString: strerror(errno)))")
            Darwin.close(serverSocket)
            serverSocket = -1
            return
        }

        // Set socket permissions (readable/writable by owner only for security)
        chmod(path, S_IRUSR | S_IWUSR)

        // Listen for connections
        guard listen(serverSocket, 5) == 0 else {
            logger.error("Failed to listen on socket: \(String(cString: strerror(errno)))")
            Darwin.close(serverSocket)
            unlink(path)
            serverSocket = -1
            return
        }

        socketPath = path
        isRunning = true

        logger.info("Socket server started at \(path)")

        // Start accept loop
        acceptLoop()
    }

    /// Stop the socket server
    func stop() {
        guard isRunning else { return }

        isRunning = false

        // Close all connections
        connectionLock.lock()
        for (_, connection) in connections {
            connection.close()
        }
        connections.removeAll()
        connectionLock.unlock()

        // Close server socket
        if serverSocket >= 0 {
            Darwin.close(serverSocket)
            serverSocket = -1
        }

        // Remove socket file
        if let path = socketPath {
            unlink(path)
            socketPath = nil
        }

        logger.info("Socket server stopped")
    }

    /// Send an event to all connections subscribed to the given request ID
    func broadcastEvent(_ event: TermsurfEvent) {
        connectionLock.lock()
        let currentConnections = Array(connections.values)
        connectionLock.unlock()

        for connection in currentConnections {
            connection.sendEvent(event)
        }
    }

    // MARK: - Private

    private func acceptLoop() {
        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            guard let self = self else { return }

            while self.isRunning {
                var clientAddr = sockaddr_un()
                var clientAddrLen = socklen_t(MemoryLayout<sockaddr_un>.size)

                let clientSocket = withUnsafeMutablePointer(to: &clientAddr) { addrPtr in
                    addrPtr.withMemoryRebound(to: sockaddr.self, capacity: 1) { sockaddrPtr in
                        Darwin.accept(self.serverSocket, sockaddrPtr, &clientAddrLen)
                    }
                }

                guard clientSocket >= 0 else {
                    if self.isRunning {
                        logger.error("Accept failed: \(String(cString: strerror(errno)))")
                    }
                    continue
                }

                // Create connection handler
                let connection = SocketConnection(fileDescriptor: clientSocket)
                let connectionId = UUID().uuidString

                self.connectionLock.lock()
                self.connections[connectionId] = connection
                self.connectionLock.unlock()

                connection.onDisconnect = { [weak self] in
                    self?.connectionLock.lock()
                    self?.connections.removeValue(forKey: connectionId)
                    self?.connectionLock.unlock()
                }

                connection.start()
            }
        }
    }

    @objc private func applicationWillTerminate(_ notification: Notification) {
        stop()
    }
}
