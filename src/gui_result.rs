#[derive(Debug)]
#[doc = "Represents a result of the interaction between the user and the GUI."]
pub enum GuiResult<T>
where
    T: Into<String>,
{
    #[doc = "The user cancelled the GUI without selecting an option"]
    Cancel,
    #[doc = "The user selected an option from the GUI"]
    Option(T),
    #[doc = "The user entered a custom option"]
    Custom(T),
}
