use std::{io::Cursor, thread};

use rodio::{Decoder, OutputStream, Sink, Source};
pub fn play_bg_music() {
    // Spawn a new thread to play the music
    thread::spawn(|| {
        // Create an output stream
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        // Create a sink to manage audio playback
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Embed the audio file as bytes
        let music_data = include_bytes!("music.mp3");
        let cursor = Cursor::new(music_data);

        // Create a decoder from the embedded data
        let source = Decoder::new(cursor).unwrap();

        // Loop the audio indefinitely using the `repeat_infinite` method
        let repeated_source = source.repeat_infinite();

        // Play the audio asynchronously
        sink.append(repeated_source);

        // Detach the sink so that it plays the audio independently
        sink.detach();

        // Keep the thread alive to allow the music to continue playing
        loop {
            thread::park(); // Park the thread to keep it alive
        }
    });
}
