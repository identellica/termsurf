import Cocoa
import SwiftUI

// For testing.
struct ColorizedGhosttyIconView: View {
  var body: some View {
    Image(
      nsImage: ColorizedGhosttyIcon(
        screenColors: [.purple, .blue],
        ghostColor: .yellow,
        frame: .aluminum
      ).makeImage()!)
  }
}
