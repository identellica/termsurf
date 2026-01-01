import Cocoa
import SwiftUI

struct DraggableWindowView: NSViewRepresentable {
    func makeNSView(context: Context) -> DraggableWindowNSView {
        return DraggableWindowNSView()
    }

    func updateNSView(_ nsView: DraggableWindowNSView, context: Context) {
        // No need to update anything here
    }
}

class DraggableWindowNSView: NSView {
    override func mouseDown(with event: NSEvent) {
        guard let window = self.window else { return }
        window.performDrag(with: event)
    }
}
