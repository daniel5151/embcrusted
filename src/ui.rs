pub trait Ui {
    fn print(&self, text: &str);
    fn print_object(&mut self, object: &str);

    fn clear(&self);
    fn set_status_bar(&self, left: &str, right: &str);

    fn get_input_buf(&mut self) -> &mut [u8];
}
