import { createFileRoute } from "@tanstack/react-router";
import { useState } from "react";

export const Route = createFileRoute("/test-media")({
  component: TestMediaPage,
});

function TestMediaPage() {
  const [status, setStatus] = useState<string>("");
  const [stream, setStream] = useState<MediaStream | null>(null);

  const requestMedia = async (constraints: MediaStreamConstraints) => {
    try {
      setStatus("Requesting permission...");
      const mediaStream = await navigator.mediaDevices.getUserMedia(constraints);
      setStream(mediaStream);
      setStatus(`Access granted! Tracks: ${mediaStream.getTracks().map(t => t.kind).join(", ")}`);
    } catch (err) {
      const error = err as Error;
      setStatus(`Error: ${error.name} - ${error.message}`);
    }
  };

  const stopMedia = () => {
    if (stream) {
      stream.getTracks().forEach(track => track.stop());
      setStream(null);
      setStatus("Stopped");
    }
  };

  return (
    <div className="max-w-2xl mx-auto p-8">
      <h1 className="text-2xl font-bold mb-6">Media Capture Test</h1>

      <section className="mb-6 p-4 bg-gray-100 rounded">
        <h2 className="font-semibold mb-2">Camera Only</h2>
        <p className="text-sm text-gray-600 mb-2">
          Request camera access (video only):
        </p>
        <button
          onClick={() => requestMedia({ video: true })}
          className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
        >
          Request Camera
        </button>
      </section>

      <section className="mb-6 p-4 bg-gray-100 rounded">
        <h2 className="font-semibold mb-2">Microphone Only</h2>
        <p className="text-sm text-gray-600 mb-2">
          Request microphone access (audio only):
        </p>
        <button
          onClick={() => requestMedia({ audio: true })}
          className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
        >
          Request Microphone
        </button>
      </section>

      <section className="mb-6 p-4 bg-gray-100 rounded">
        <h2 className="font-semibold mb-2">Camera + Microphone</h2>
        <p className="text-sm text-gray-600 mb-2">
          Request both camera and microphone:
        </p>
        <button
          onClick={() => requestMedia({ video: true, audio: true })}
          className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
        >
          Request Both
        </button>
      </section>

      {stream && (
        <section className="mb-6 p-4 bg-green-100 rounded">
          <h2 className="font-semibold mb-2">Active Stream</h2>
          <button
            onClick={stopMedia}
            className="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600"
          >
            Stop Stream
          </button>
        </section>
      )}

      {status && (
        <section className="p-4 bg-gray-200 rounded">
          <h2 className="font-semibold mb-2">Status</h2>
          <p className="font-mono text-sm">{status}</p>
        </section>
      )}
    </div>
  );
}
