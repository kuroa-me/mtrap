use anyhow::{bail, Result};
use octets::Octets;
use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("IPDB Error: {0}")]
struct IPDBError(String);

const IPV4: i32 = 0x01;
const IPV6: i32 = 0x02;
const MAGIC_PREFIX_LEN: u32 = 0x4;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct MetaData {
    pub build: i64,
    pub ip_version: u16,
    pub languages: HashMap<String, i64>,
    pub node_count: i64,
    pub total_size: i64,
    pub fields: Vec<String>,
}

struct Reader {
    file_size: i64,
    node_count: i64,
    v4_offset: i64,

    meta: MetaData,
    data: Vec<u8>,

    ref_type: HashMap<String, String>,
}

impl Reader {
    fn new(name: String) -> Result<Self> {
        let mut file = File::open(name)?;
        let file_size = file.metadata()?.len();
        let mut data = Vec::with_capacity(file_size as usize);
        file.read_to_end(&mut data)?;

        let mut reader = Reader {
            file_size: 0,
            node_count: 0,
            v4_offset: 0,
            meta: MetaData {
                build: 0,
                ip_version: 0,
                languages: HashMap::new(),
                node_count: 0,
                total_size: 0,
                fields: Vec::new(),
            },
            data,
            ref_type: HashMap::new(),
        };

        reader.init_bytes()?;
        Ok(reader)
    }

    fn init_bytes(&mut self) -> Result<()> {
        let mut buf = Octets::with_slice(&self.data);

        let meta_len = buf.get_u32()?;
        if self.file_size < (meta_len + MAGIC_PREFIX_LEN) as i64 {
            bail!("file size error");
        }

        let meta: MetaData = serde_json::from_slice(buf.get_bytes(meta_len as usize)?.as_ref())?;
        if meta.languages.len() == 0 || meta.fields.len() == 0 {
            bail!("metadata error");
        }
        if self.file_size != (MAGIC_PREFIX_LEN as i64 + meta_len as i64 + meta.total_size) {
            bail!("file size doesn't match with metadata");
        }

        // let mut dm = HashMap::<String, String>::new();

        self.meta = meta;
        self.node_count = self.meta.node_count;
        self.data = self.data[buf.off()..].to_vec();

        if self.v4_offset == 0 {
            let node = 0;
            for i in 0..96 {
                if node >= self.meta.node_count {
                    break;
                }
                if i >= 80 {
                    node = self.read_node(node, 1);
                } else {
                    node = self.read_node(node, 0);
                }
            }
            self.v4_offset = node;
        }

        Ok(())
    }

    fn read_node(&mut self, node: i64, index: i64) -> i64 {
        let off = node * 8 + index * 4;
        return (); //TODO
    }
}
