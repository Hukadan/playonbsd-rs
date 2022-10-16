pub trait BasicItem {
    fn new() -> Self;
    fn get_name(&self) -> &str;
    fn get_id(&self) -> usize;
    fn set_id(&mut self, id: usize);
    fn print_details(&self) -> String;
}
