import Foundation
import os

private let logger = Logger(subsystem: "com.termsurf", category: "ProfileManager")

/// Errors that can occur during bookmark operations
enum BookmarkError: LocalizedError {
  case alreadyExists(name: String)
  case notFound(name: String)

  var errorDescription: String? {
    switch self {
    case .alreadyExists(let name):
      return "Bookmark '\(name)' already exists"
    case .notFound(let name):
      return "Bookmark '\(name)' not found"
    }
  }
}

/// Manages profile data including UUIDs and bookmarks.
/// All operations are thread-safe via a serial dispatch queue.
class ProfileManager {
  /// Shared singleton instance
  static let shared = ProfileManager()

  /// Serial queue for thread-safe access to profile data
  private let queue = DispatchQueue(label: "com.termsurf.ProfileManager")

  /// Cache of loaded profiles to avoid repeated disk I/O
  private var profileCache: [String: Profile] = [:]

  private init() {}

  // MARK: - Public API

  /// Get the UUID for a profile using deterministic hash-based generation.
  /// This is the same algorithm used in WebViewOverlay for WebKit data stores.
  func uuidForProfile(name: String) -> UUID {
    // Create deterministic UUID from profile name using a hash
    let hash = name.hash
    let hashStr = String(
      format: "%08x%08x%04x%04x%012x",
      UInt32(truncatingIfNeeded: hash),
      UInt32(truncatingIfNeeded: hash >> 32),
      UInt16(truncatingIfNeeded: hash >> 48),
      UInt16(truncatingIfNeeded: hash >> 56),
      UInt64(truncatingIfNeeded: hash))
    let uuidStr =
      "\(hashStr.prefix(8))-\(hashStr.dropFirst(8).prefix(4))-\(hashStr.dropFirst(12).prefix(4))-\(hashStr.dropFirst(16).prefix(4))-\(hashStr.dropFirst(20).prefix(12))"
    return UUID(uuidString: uuidStr) ?? UUID()
  }

  /// Load a profile by name. Creates a new profile if it doesn't exist.
  func loadProfile(name: String) -> Profile {
    return queue.sync {
      // Check cache first
      if let cached = profileCache[name] {
        return cached
      }

      // Try to load from disk
      let path = getProfilePath(name: name)
      if FileManager.default.fileExists(atPath: path.path) {
        do {
          let data = try Data(contentsOf: path)
          let profile = try JSONDecoder().decode(Profile.self, from: data)
          profileCache[name] = profile
          logger.info("Loaded profile '\(name)' from disk")
          return profile
        } catch {
          logger.error("Failed to load profile '\(name)': \(error.localizedDescription)")
          // Fall through to create new profile
        }
      }

      // Create new profile with deterministic UUID
      let profile = Profile(id: uuidForProfile(name: name), bookmarks: [:])
      profileCache[name] = profile

      // Save to disk
      do {
        try saveProfileInternal(name: name, profile: profile)
        logger.info("Created new profile '\(name)' with UUID: \(profile.id)")
      } catch {
        logger.error("Failed to save new profile '\(name)': \(error.localizedDescription)")
      }

      return profile
    }
  }

  /// Save a profile to disk.
  func saveProfile(name: String, profile: Profile) throws {
    try queue.sync {
      try saveProfileInternal(name: name, profile: profile)
      profileCache[name] = profile
    }
  }

  /// Get the file path for a profile's JSON file.
  func getProfilePath(name: String) -> URL {
    let configDir = getConfigDirectory()
    return configDir.appendingPathComponent("\(name).json")
  }

  /// Ensure the profile exists on disk (creates if needed).
  /// Call this when a webview uses a profile to ensure the JSON file exists.
  func ensureProfileExists(name: String) {
    _ = loadProfile(name: name)
  }

  // MARK: - Bookmark Operations

  /// Add a new bookmark to a profile.
  /// - Throws: `BookmarkError.alreadyExists` if bookmark with same name exists
  func addBookmark(profile: String, name: String, title: String, url: String) throws {
    try queue.sync {
      var profileData = loadProfileInternal(name: profile)

      guard profileData.bookmarks[name] == nil else {
        throw BookmarkError.alreadyExists(name: name)
      }

      profileData.bookmarks[name] = Bookmark(title: title, url: url)
      try saveProfileInternal(name: profile, profile: profileData)
      profileCache[profile] = profileData
      logger.info("Added bookmark '\(name)' to profile '\(profile)'")
    }
  }

  /// Get a bookmark by name from a profile.
  /// Returns nil if the bookmark doesn't exist.
  func getBookmark(profile: String, name: String) -> Bookmark? {
    return queue.sync {
      let profileData = loadProfileInternal(name: profile)
      return profileData.bookmarks[name]
    }
  }

  /// List all bookmarks in a profile.
  func listBookmarks(profile: String) -> [String: Bookmark] {
    return queue.sync {
      let profileData = loadProfileInternal(name: profile)
      return profileData.bookmarks
    }
  }

  /// Update an existing bookmark.
  /// - Throws: `BookmarkError.notFound` if bookmark doesn't exist
  func updateBookmark(profile: String, name: String, title: String?, url: String?) throws {
    try queue.sync {
      var profileData = loadProfileInternal(name: profile)

      guard var bookmark = profileData.bookmarks[name] else {
        throw BookmarkError.notFound(name: name)
      }

      if let title = title {
        bookmark.title = title
      }
      if let url = url {
        bookmark.url = url
      }

      profileData.bookmarks[name] = bookmark
      try saveProfileInternal(name: profile, profile: profileData)
      profileCache[profile] = profileData
      logger.info("Updated bookmark '\(name)' in profile '\(profile)'")
    }
  }

  /// Delete a bookmark from a profile.
  /// - Throws: `BookmarkError.notFound` if bookmark doesn't exist
  func deleteBookmark(profile: String, name: String) throws {
    try queue.sync {
      var profileData = loadProfileInternal(name: profile)

      guard profileData.bookmarks[name] != nil else {
        throw BookmarkError.notFound(name: name)
      }

      profileData.bookmarks.removeValue(forKey: name)
      try saveProfileInternal(name: profile, profile: profileData)
      profileCache[profile] = profileData
      logger.info("Deleted bookmark '\(name)' from profile '\(profile)'")
    }
  }

  /// Derive a bookmark name from a URL.
  /// - google.com -> google
  /// - www.google.com -> google
  /// - blog.myname.com -> blog
  /// - google.co.uk -> google
  static func deriveNameFromURL(_ url: URL) -> String {
    guard let host = url.host else {
      return "bookmark"
    }

    let parts = host.split(separator: ".")
    guard !parts.isEmpty else {
      return "bookmark"
    }

    // Skip "www" if it's the first part
    let startIndex = (parts.first == "www" && parts.count > 1) ? 1 : 0
    return String(parts[startIndex])
  }

  // MARK: - Private Helpers

  /// Internal load method (must be called within queue.sync)
  private func loadProfileInternal(name: String) -> Profile {
    // Check cache first
    if let cached = profileCache[name] {
      return cached
    }

    // Try to load from disk
    let path = getProfilePath(name: name)
    if FileManager.default.fileExists(atPath: path.path) {
      do {
        let data = try Data(contentsOf: path)
        let profile = try JSONDecoder().decode(Profile.self, from: data)
        profileCache[name] = profile
        return profile
      } catch {
        logger.error("Failed to load profile '\(name)': \(error.localizedDescription)")
      }
    }

    // Create new profile with deterministic UUID
    let profile = Profile(id: uuidForProfile(name: name), bookmarks: [:])
    profileCache[name] = profile
    return profile
  }

  /// Get the TermSurf config directory (~/.config/termsurf/)
  private func getConfigDirectory() -> URL {
    let homeDir = FileManager.default.homeDirectoryForCurrentUser
    return homeDir.appendingPathComponent(".config/termsurf")
  }

  /// Internal save method (must be called within queue.sync)
  private func saveProfileInternal(name: String, profile: Profile) throws {
    let path = getProfilePath(name: name)

    // Ensure config directory exists
    let configDir = getConfigDirectory()
    if !FileManager.default.fileExists(atPath: configDir.path) {
      try FileManager.default.createDirectory(at: configDir, withIntermediateDirectories: true)
      logger.info("Created config directory: \(configDir.path)")
    }

    // Encode and write profile
    let encoder = JSONEncoder()
    encoder.outputFormatting = [.prettyPrinted, .sortedKeys]
    let data = try encoder.encode(profile)
    try data.write(to: path)
    logger.debug("Saved profile '\(name)' to \(path.path)")
  }
}

// MARK: - Data Types

/// A user profile containing a UUID (for WebKit data store) and bookmarks.
struct Profile: Codable {
  /// Deterministic UUID generated from profile name, used for WebKit data store
  let id: UUID

  /// Bookmarks keyed by unique name
  var bookmarks: [String: Bookmark]
}

/// A bookmark entry
struct Bookmark: Codable {
  /// Human-readable title (e.g., "Google Search")
  var title: String

  /// The URL to open
  var url: String
}
