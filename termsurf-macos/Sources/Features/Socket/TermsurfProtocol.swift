import Foundation

// Protocol messages for CLI-to-app communication over Unix domain sockets.
// All messages are JSON-encoded with newline delimiters.

// MARK: - Request (CLI → App)

struct TermsurfRequest: Codable {
  /// Unique request ID for matching responses
  let id: String

  /// Action to perform: "ping", "open", "close", "show", "hide"
  let action: String

  /// Target pane ID (from TERMSURF_PANE_ID env var)
  let paneId: String?

  /// Action-specific data
  let data: [String: AnyCodableValue]?

  init(id: String, action: String, paneId: String? = nil, data: [String: AnyCodableValue]? = nil) {
    self.id = id
    self.action = action
    self.paneId = paneId
    self.data = data
  }
}

// MARK: - Response (App → CLI)

struct TermsurfResponse: Codable {
  /// Request ID this is responding to
  let id: String

  /// Status: "ok" or "error"
  let status: String

  /// Response data (action-specific)
  let data: [String: AnyCodableValue]?

  /// Error message if status is "error"
  let error: String?

  static func ok(id: String, data: [String: AnyCodableValue]? = nil) -> TermsurfResponse {
    TermsurfResponse(id: id, status: "ok", data: data, error: nil)
  }

  static func error(id: String, message: String) -> TermsurfResponse {
    TermsurfResponse(id: id, status: "error", data: nil, error: message)
  }
}

// MARK: - Event (App → CLI, for long-running operations)

struct TermsurfEvent: Codable {
  /// Request ID this event relates to
  let id: String

  /// Event type: "closed", "backgrounded", "loaded", etc.
  let event: String

  /// Event-specific data
  let data: [String: AnyCodableValue]?

  init(id: String, event: String, data: [String: AnyCodableValue]? = nil) {
    self.id = id
    self.event = event
    self.data = data
  }
}

// MARK: - AnyCodableValue for flexible JSON values

/// A type-erased Codable value for flexible JSON encoding/decoding
enum AnyCodableValue: Codable, Equatable {
  case string(String)
  case int(Int)
  case double(Double)
  case bool(Bool)
  case null

  init(from decoder: Decoder) throws {
    let container = try decoder.singleValueContainer()

    if container.decodeNil() {
      self = .null
    } else if let bool = try? container.decode(Bool.self) {
      self = .bool(bool)
    } else if let int = try? container.decode(Int.self) {
      self = .int(int)
    } else if let double = try? container.decode(Double.self) {
      self = .double(double)
    } else if let string = try? container.decode(String.self) {
      self = .string(string)
    } else {
      throw DecodingError.typeMismatch(
        AnyCodableValue.self,
        DecodingError.Context(codingPath: decoder.codingPath, debugDescription: "Unsupported type")
      )
    }
  }

  func encode(to encoder: Encoder) throws {
    var container = encoder.singleValueContainer()
    switch self {
    case .string(let value): try container.encode(value)
    case .int(let value): try container.encode(value)
    case .double(let value): try container.encode(value)
    case .bool(let value): try container.encode(value)
    case .null: try container.encodeNil()
    }
  }

  var stringValue: String? {
    if case .string(let value) = self { return value }
    return nil
  }

  var intValue: Int? {
    if case .int(let value) = self { return value }
    return nil
  }

  var boolValue: Bool? {
    if case .bool(let value) = self { return value }
    return nil
  }
}

// MARK: - Convenience extensions

extension TermsurfRequest {
  /// Get a string value from the data dictionary
  func getString(_ key: String) -> String? {
    data?[key]?.stringValue
  }

  /// Get an int value from the data dictionary
  func getInt(_ key: String) -> Int? {
    data?[key]?.intValue
  }

  /// Get a bool value from the data dictionary
  func getBool(_ key: String) -> Bool? {
    data?[key]?.boolValue
  }
}
