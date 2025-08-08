#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reloc {
    // This is a stub type which is intentionally a zero-variant enum.
    // Currently it is used as Option<Reloc> which is always None.
}
