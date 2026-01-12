import Foundation

/// True if we appear to be running in Xcode.
func isRunningInXcode() -> Bool {
  if ProcessInfo.processInfo.environment["__XCODE_BUILT_PRODUCTS_DIR_PATHS"] != nil {
    return true
  }

  return false
}
