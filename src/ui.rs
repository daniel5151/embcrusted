pub trait Ui {
    /// Print out some text
    fn print(&self, text: &str);
    /// Print out an object (maybe you can highlight it somehow?)
    fn print_object(&mut self, object: &str);
    /// Set status bar text
    fn set_status_bar(&self, left: &str, right: &str);
    /// Return a reference to the most-recently inputted text
    fn get_input_buf(&mut self) -> &str;
}
