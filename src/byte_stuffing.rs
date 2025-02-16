// use crate::byte_stuffing::ReaderState::*;

pub fn encode_iter(bytes: &[u8]) -> impl Iterator<Item=u8> + '_ {
    let mut state = Some(EncodeState::Begin(bytes));
    core::iter::from_fn(move || {
        let s = state?;
        let (b, s2) = s.next();
        state = s2;
        Some(b)
    })
}

/// State for incremental encoding.
#[derive(Copy, Clone, Debug)]
enum EncodeState<'a> {
    Begin(&'a [u8]),
    Encoding(&'a [u8]),
    EscapedCharacter(u8, &'a [u8]),
}

const START_BYTE: u8 = 0x02;
const END_BYTE: u8 = 0x03;
const ESCAPE_BYTE: u8 = 0x04;

impl<'a> EncodeState<'a> {
    pub fn next(self) -> (u8, Option<Self>) {
        match self {
            EncodeState::Begin(bytes) => {
                (START_BYTE, Some(Self::Encoding(bytes)))
            }
            EncodeState::Encoding(bytes) => {
                match bytes.split_first() {
                    None => {
                        (END_BYTE, None)
                    }
                    Some((first, rest)) => {
                        match *first {
                            START_BYTE | END_BYTE | ESCAPE_BYTE => {
                                (ESCAPE_BYTE, Some(Self::EscapedCharacter(*first, rest)))
                            }
                            _ => {
                                (*first, Some(Self::Encoding(rest)))
                            }
                        }
                    }
                }
            }
            EncodeState::EscapedCharacter(character, bytes) => {
                (0xFF ^ character, Some(Self::Encoding(bytes)))
            }
        }
    }
}

// pub fn from_reader<R>(mut reader: R, message_buffer: &mut [u8]) -> Result<&[u8], DecoderError>
//     where R: std::io::Read,
// {
//     let mut current_bytes = 0;
//     let mut state = WaitingForStart;
//     let mut ingest_buffer = [0x00; 1];
//     loop {
//         let read_result = reader.read( &mut ingest_buffer);
//         match read_result {
//             Err(error) => return Err(DecoderError::IoError(error)),
//             Ok(0) => {
//                 return Err(DecoderError::ReaderEndOfFile)
//             }
//             Ok(_) => {
//                 let next = ingest_buffer[0];
//                 #[cfg(test)]
//                 println!("Byte {}, state {:?}", next, state);
//                 match next {
//                     START_BYTE => {
//                         state = InsideMessage;
//                         current_bytes = 0;
//                     }
//                     END_BYTE => {
//                         match state {
//                             WaitingForStart => {}
//                             InsideMessage => {
//                                 return Ok(&message_buffer[..current_bytes]);
//                             }
//                             InsideMessageEscaping => {
//                                 state = WaitingForStart;
//                                 #[cfg(test)]
//                                 return Err(DecoderError::InvalidEscaped(next));
//                             }
//                         }
//                     }
//                     ESCAPE_BYTE => {
//                         match state {
//                             WaitingForStart => {}
//                             InsideMessage => {
//                                 state = InsideMessageEscaping;
//                             }
//                             InsideMessageEscaping => {
//                                 state = WaitingForStart;
//                                 #[cfg(test)]
//                                 return Err(DecoderError::InvalidEscaped(next));
//                             }
//                         }
//                     }
//                     _ => {
//                         let byte = match state {
//                             WaitingForStart => {continue}
//                             InsideMessage => next,
//                             InsideMessageEscaping => {
//                                 match next ^ 0xFF {
//                                     byte @ START_BYTE |
//                                     byte @ END_BYTE |
//                                     byte @ ESCAPE_BYTE => {
//                                         byte
//                                     }
//                                     byte => {
//                                         // We got an escaped character that didn't need escaping
//                                         // We're probably getting invalid data.
//                                         // Silently recover
//                                         state = WaitingForStart;
//                                         #[cfg(test)]
//                                         return Err(DecoderError::InvalidEscaped(next));
//                                         continue;
//                                     },
//                                 }
//                             }
//                         };
//                         if let Some(slot) = message_buffer.get_mut(current_bytes) {
//                             *slot = byte;
//                             current_bytes += 1;
//                             state = InsideMessage;
//                         } else {
//                             // Writing outside memory.
//                             // We're getting longer messages than we should.
//                             // Start over and hope that it recovers later
//                             state = WaitingForStart;
//                             #[cfg(test)]
//                             return Err(DecoderError::BufferTooSmall);
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
//
// #[derive(Debug)]
// pub enum DecoderError {
//     IoError(std::io::Error),
//     ReaderEndOfFile,
//     #[cfg(test)]
//     BufferTooSmall,
//     #[cfg(test)]
//     InvalidEscaped(u8),
// }
//
// #[derive(Debug, Clone)]
// enum ReaderState {
//     WaitingForStart,
//     InsideMessage,
//     InsideMessageEscaping,
// }

#[cfg(test)]
mod tests {
    use crate::byte_stuffing::{DecoderError, encode_iter, from_reader};

    #[test]
    fn encode_test() {
        let input = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let target_output = [0x02, 0x00, 0x01, 0x04, 0xFD, 0x04, 0xFC, 0x04, 0xFB, 0x05, 0x06, 0x03];
        let output = encode_iter(&input).collect::<Vec<u8>>();
        assert_eq!(output, target_output);
    }
    #[test]
    fn decode_test() {
        let mut message_buffer= [0; 128];
        let input: [u8; 12] = [0x02, 0x00, 0x01, 0x04, 0xFD, 0x04, 0xFC, 0x04, 0xFB, 0x05, 0x06, 0x03];
        let target_output: [u8; 7] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        match from_reader(std::io::Cursor::new(input), &mut message_buffer) {
            Ok(output) => {
                assert_eq!(output, target_output);
            }
            Err(error) => {
                println!("message_buffer: {:?}", message_buffer);
                Err::<&[u8], _>(error).unwrap();
            }
        }
    }
}