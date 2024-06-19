use std::borrow::Cow;
use std::io::{Read, Write};

use serde::{Deserialize, Serialize};

use super::Error;
use crate::managers::room::RoomID;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Request<'a> {
    Login {
        #[serde(borrow)]
        username: Cow<'a, str>,
        #[serde(borrow)]
        password: Cow<'a, str>,
    },
    Signup {
        #[serde(borrow)]
        username: Cow<'a, str>,
        #[serde(borrow)]
        password: Cow<'a, str>,
    },
    JoinRoom(RoomID),
    CreateRoom {
        #[serde(borrow)]
        name: Cow<'a, str>,
        max_users: usize,
    },
    PersonalStats,
    Highscores,
    Logout,
    RoomList,
    CloseRoom,
    StartGame,
    RoomState,
    LeaveRoom,
    LeaveGame,
    GameResult,
}

impl<'buf> Request<'buf> {
    pub fn read_from(buf: &'buf mut Vec<u8>, reader: &mut impl Read) -> Result<Self, Error> {
        let mut buf_data_len = [0; 4];
        reader.read_exact(&mut buf_data_len)?;
        let data_len = u32::from_le_bytes(buf_data_len);
        let data_len = data_len as usize;

        buf.clear();
        buf.resize(data_len, 0);
        reader.read_exact(buf)?;

        let request = serde_json::from_slice(buf)?;
        Ok(request)
    }

    pub fn write_to(&self, writer: &mut impl Write) -> Result<(), Error> {
        let json = serde_json::to_vec(&self)?;
        let len = json.len() as u32;
        let len_bytes = len.to_le_bytes();
        writer.write_all(&len_bytes)?;
        writer.write_all(&json)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct RequestInfo<'a> {
    pub data: Request<'a>,
}

impl<'a> RequestInfo<'a> {
    pub fn new(data: Request<'a>) -> Self {
        Self { data }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn serde() {
        use super::Request;
        use std::io::Cursor;

        let to_test = [
            Request::Signup {
                username: "user1234".into(),
                password: "Pass@123".into(),
            },
            Request::Login {
                username: "user1234".into(),
                password: "Pass@123".into(),
            },
        ];

        for original_response in to_test {
            let mut buf = Vec::new();
            original_response.write_to(&mut buf).unwrap();
            let mut reader = Cursor::new(buf);
            let mut output = Vec::new();
            let parsed_response = Request::read_from(&mut output, &mut reader).unwrap();
            assert_eq!(original_response, parsed_response);
        }
    }
}
