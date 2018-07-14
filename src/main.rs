extern crate tinyfiledialogs;


fn main() {
   let res = tinyfiledialogs::select_folder_dialog("Select music directory", "/");
   println!("{:?}", res);
}
