
// Build function to compile and integrate GUI in the final binary
//
fn main() {
  slint_build::compile("ui/app-window.slint").expect("Slint build failed");
}
