// An internal helper for decoding an iterator of chars and displaying them
pub struct Decode<F>(pub F);

impl<F, R, E> core::fmt::Display for Decode<F>
where
    F: Clone + FnOnce() -> R,
    R: IntoIterator<Item = Result<char, E>>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use core::fmt::Write;
        let iter = self.0.clone();
        for c in iter().into_iter() {
            f.write_char(c.unwrap_or(core::char::REPLACEMENT_CHARACTER))?
        }
        Ok(())
    }
}
