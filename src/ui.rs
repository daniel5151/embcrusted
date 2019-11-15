pub trait Ui {
    fn print(&self, text: &str);
    fn print_object(&mut self, object: &str);

    fn clear(&self);
    fn set_status_bar(&self, left: &str, right: &str);

    fn get_user_input(&self) -> String;
    fn reset(&self);
}
