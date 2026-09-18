pub struct HexRow {
    pub offset: usize,
    pub bytes: Vec<u8>,
    pub ascii: String,
}

pub fn build_hex_rows(data: &[u8], bytes_per_row: usize) -> Vec<HexRow> {
    let mut rows = Vec::new();
    let mut offset = 0;
    while offset < data.len() {
        let end = (offset + bytes_per_row).min(data.len());
        let chunk = &data[offset..end];
        let ascii: String = chunk
            .iter()
            .map(|b| {
                if *b >= 0x20 && *b < 0x7f {
                    *b as char
                } else {
                    '.'
                }
            })
            .collect();
        rows.push(HexRow {
            offset,
            bytes: chunk.to_vec(),
            ascii,
        });
        offset = end;
    }
    rows
}

pub fn read_file_bytes(path: &std::path::Path) -> anyhow::Result<Vec<u8>> {
    Ok(std::fs::read(path)?)
}

#[derive(Clone, Debug)]
pub struct BytePatch {
    pub offset: usize,
    pub original: u8,
    pub new_value: u8,
}

pub struct PatchSet {
    pub patches: Vec<BytePatch>,
}

impl PatchSet {
    pub fn new() -> Self {
        Self { patches: Vec::new() }
    }

    pub fn apply_patch(&mut self, data: &mut [u8], offset: usize, new_value: u8) {
        if offset >= data.len() {
            return;
        }
        let original = data[offset];
        if original == new_value {
            self.patches.retain(|p| p.offset != offset);
            return;
        }
        if let Some(existing) = self.patches.iter_mut().find(|p| p.offset == offset) {
            existing.new_value = new_value;
        } else {
            self.patches.push(BytePatch {
                offset,
                original,
                new_value,
            });
        }
        data[offset] = new_value;
    }

    pub fn revert_patch(&mut self, data: &mut [u8], offset: usize) {
        if let Some(index) = self.patches.iter().position(|p| p.offset == offset) {
            let patch = self.patches.remove(index);
            if offset < data.len() {
                data[offset] = patch.original;
            }
        }
    }

    pub fn revert_all(&mut self, data: &mut [u8]) {
        for patch in self.patches.drain(..) {
            if patch.offset < data.len() {
                data[patch.offset] = patch.original;
            }
        }
    }

    pub fn is_patched(&self, offset: usize) -> bool {
        self.patches.iter().any(|p| p.offset == offset)
    }

    pub fn has_changes(&self) -> bool {
        !self.patches.is_empty()
    }
}

impl Default for PatchSet {
    fn default() -> Self {
        Self::new()
    }
}

pub fn write_patched_file(path: &std::path::Path, data: &[u8]) -> anyhow::Result<()> {
    std::fs::write(path, data)?;
    Ok(())
}

pub fn write_patched_copy(
    original_path: &std::path::Path,
    data: &[u8],
) -> anyhow::Result<std::path::PathBuf> {
    let mut new_path = original_path.to_path_buf();
    let stem = original_path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "patched".to_string());
    let extension = original_path.extension().and_then(|e| e.to_str());
    let new_name = match extension {
        Some(ext) => format!("{stem}_patched.{ext}"),
        None => format!("{stem}_patched"),
    };
    new_path.set_file_name(new_name);
    std::fs::write(&new_path, data)?;
    Ok(new_path)
}
