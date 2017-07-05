#[derive(Debug)]
pub enum GuiResult<T>
where
    T: Into<String>,
{
    Cancel,
    Option(T),
    Custom(T),
}
