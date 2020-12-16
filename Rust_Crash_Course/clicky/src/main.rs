mod replace_callback;

// extern crate gtk;

// use gtk::prelude::*;
// use gtk::{Button, Window, WindowType};

fn main() {
  /*
  if gtk::init().is_err() {
    println!("failed to init GTK.");
    return;
  }

  let window = Window::new(WindowType::Toplevel);
  window.set_title("First GTK+ Program");
  window.set_default_size(350, 70);

  let button = Button::new_with_label("Click me");
  window.add(&button);
  window.show_all();

  window.connect_delete_event(|_, _| {
    gtk::main_quit();
    Inhibit(false)
  });

  button.connect_clicked(|_| {
    println!("Clicked!");
  });

  gtk::main();
   */


  // replace_callback::comm2();
  replace_callback::comm3();

}


