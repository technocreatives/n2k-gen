#[derive(Debug, Copy, Clone)]
pub enum FastPacketError {
    UnexpectedFrameIndex { index: usize, expected: usize },
    PacketTooBig { actual: usize, expected: usize },
}

/// Identifier for fast packets that are being reassembled
pub type FastPacketIdentifier = (u8, u32, u8);
pub struct FastPacketCache {
    pub data: heapless::Vec<u8, 255>,
    pub next_frame: Option<usize>,
    pub total_size: usize,
}

impl FastPacketCache {
    pub fn new(total_size: usize) -> Self {
        Self {
            data: heapless::Vec::new(),
            next_frame: None,
            total_size,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.data.len() >= self.total_size
    }

    pub fn complete_data(&self) -> Option<&[u8]> {
        if self.is_complete() {
            Some(&self.data[..self.total_size])
        } else {
            None
        }
    }

    pub fn extend(&mut self, frame_index: usize, data: &[u8]) -> Result<bool, FastPacketError> {
        if self.next_frame.is_some() && frame_index != self.next_frame.unwrap() {
            return Err(FastPacketError::UnexpectedFrameIndex {
                index: frame_index,
                expected: self.next_frame.unwrap(),
            });
        }

        self.data.extend_from_slice(data).unwrap();

        // Increment frame counter
        self.next_frame = Some(frame_index + 1);

        Ok(self.is_complete())
    }
}
