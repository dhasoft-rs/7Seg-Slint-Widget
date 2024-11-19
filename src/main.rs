// Prevent console window in addition to Slint window in Windows release builds when,
// e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
slint::include_modules!();


// Entry point /main() function
//
fn main() -> Result<(), Box<dyn Error>> {
  let ui = AppWindow::new()?;

  // request_increase_value callback
  //
  ui.on_request_increase_value({
    let ui_handle = ui.as_weak();
    move || {
      let ui = ui_handle.unwrap();
      ui.set_counter(ui.get_counter() + 1);
      ui.set_counter_as_string(ui.get_counter().to_string().into());
    }
  });

  // request_decrease_value callback
  //
  ui.on_request_decrease_value({
    let ui_handle = ui.as_weak();
    move || {
      let ui = ui_handle.unwrap();
      ui.set_counter(ui.get_counter() - 1);
      ui.set_counter_as_string(ui.get_counter().to_string().into());
    }
  });

  // check_input_value callback
  //
  ui.on_check_input_value({
    let ui_handle = ui.as_weak();
    move || {
      let ui = ui_handle.unwrap();
      let mut new_str = ui.get_counter_as_string();
      if new_str.len() > 3 {        
        new_str = new_str[0..3].into(); // Remove characters afters 3rd character from input string
      }
      let my_integer: Result<i32, _> = remove_bad_char(&new_str).parse();
      match my_integer { // Check my_integer is number
        Ok(_number) => ui.set_counter_as_string(new_str),
        Err(_) => {} // Can only occur if remove_bad_char() returns an empty string
      }
    }
  });

  // validate_input_value callback (called when ENTER pressed)
  //
  ui.on_validate_input_value({
    let ui_handle = ui.as_weak();
    move || {
      let ui = ui_handle.unwrap();
      let new_str = ui.get_counter_as_string();
      if new_str.len() == 0 {
        ui.set_counter_as_string(ui.get_counter().to_string().into()); // Get previous counter value if input field is empty
        return;
      }
      let my_integer: Result<i32, _> = new_str.parse(); // At this point, we're sure that new_str contains only digits/numeric
      match my_integer {
        Ok(number) => { // So we're sure that my_integer is a valid integer (was checked before)
          ui.set_counter(number);
          ui.set_counter_as_string(format!("{:03}", ui.get_counter()).into());
        }
        Err(_) => {}
      }
    }
  });

  ui.run()?;
  Ok(())
}


// Remove all characters that are not digit / numeric and return purged string
//
fn remove_bad_char(num: &str) -> String {
  let mut purged_str = String::from("");
  for (_i, c) in num.chars().enumerate() {
    if c.is_numeric() {
      purged_str.push(c);
    }
  }
  purged_str
}
