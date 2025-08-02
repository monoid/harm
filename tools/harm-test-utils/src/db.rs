use aarchmrs_types::InstructionCode;
use hex::FromHex as _;
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Malformed db line: {0:?}")]
    MalformedDbLine(&'static str),
    #[error("Malformed hex opcode: {0:?}")]
    MalformedHexOpcode(&'static str),
}

pub struct Db {
    codes: HashMap<&'static str, InstructionCode>,
}

impl Db {
    pub fn new(db_data: &'static str) -> Result<Self, DbError> {
        let codes = db_data
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| -> Result<_, DbError> {
                let (hex, cmd) = line
                    .split_once('\t')
                    .ok_or(DbError::MalformedDbLine(line))?;
                let instruction = InstructionCode::from_hex(hex.as_bytes())
                    .map_err(|_| DbError::MalformedHexOpcode(hex))?;
                Ok((cmd, instruction))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let codes = HashMap::from_iter(codes);
        Ok(Self { codes })
    }

    pub fn find(&self, key: &str) -> Option<InstructionCode> {
        self.codes.get(key).cloned()
    }

    pub fn keys(&self) -> impl Iterator<Item = &'static str> {
        self.codes.keys().copied()
    }
}

pub fn db(db_data: &'static str, key: &str) -> InstructionCode {
    let db = Db::new(db_data).unwrap_or_else(|e| panic!("{e}"));
    db.find(key)
        .unwrap_or_else(|| panic!("key not found: {key:?}"))
}
