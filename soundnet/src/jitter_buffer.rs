use crate::network::packet::AudioPacket;
use std::collections::VecDeque;

pub struct JitterBuffer {
    buffer: VecDeque<AudioPacket>,
    max_size: usize,
}

impl JitterBuffer {
    pub fn new(max_size: usize) -> Self {
        JitterBuffer {
            buffer: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    pub fn add(&mut self, packet: AudioPacket) {
        if self.buffer.len() >= self.max_size {
            // Buffer is full, drop the oldest packet
            self.buffer.pop_front();
        }

        // Insert the packet in the correct position based on the timestamp
        let mut inserted = false;
        for i in 0..self.buffer.len() {
            if packet.timestamp < self.buffer[i].timestamp {
                self.buffer.insert(i, packet.clone());
                inserted = true;
                break;
            }
        }

        if !inserted {
            self.buffer.push_back(packet);
        }
    }

    pub fn get_next_frame(&mut self) -> Option<AudioPacket> {
        self.buffer.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jitter_buffer_add_and_get() {
        let mut jitter_buffer = JitterBuffer::new(3);
        jitter_buffer.add(AudioPacket { timestamp: 1, audio_data: vec![1.0] });
        jitter_buffer.add(AudioPacket { timestamp: 3, audio_data: vec![3.0] });
        jitter_buffer.add(AudioPacket { timestamp: 2, audio_data: vec![2.0] });

        assert_eq!(jitter_buffer.get_next_frame().unwrap().timestamp, 1);
        assert_eq!(jitter_buffer.get_next_frame().unwrap().timestamp, 2);
        assert_eq!(jitter_buffer.get_next_frame().unwrap().timestamp, 3);
    }

    #[test]
    fn test_jitter_buffer_full() {
        let mut jitter_buffer = JitterBuffer::new(2);
        jitter_buffer.add(AudioPacket { timestamp: 1, audio_data: vec![1.0] });
        jitter_buffer.add(AudioPacket { timestamp: 2, audio_data: vec![2.0] });
        jitter_buffer.add(AudioPacket { timestamp: 3, audio_data: vec![3.0] });

        assert_eq!(jitter_buffer.get_next_frame().unwrap().timestamp, 2);
        assert_eq!(jitter_buffer.get_next_frame().unwrap().timestamp, 3);
    }
}
