use glib::clone;
use gtk::prelude::*;
use gtk::{Button, HeaderBar, Label, Notebook, Orientation, Paned, 
ScrolledWindow, TextBuffer, TextView, Window, WindowType};

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("ChatGPT Interface");
    window.set_default_size(800, 600);

    let header_bar = HeaderBar::new();
    header_bar.set_show_close_button(true);
    window.set_titlebar(Some(&header_bar));

    let notebook = Notebook::new();
    notebook.set_tab_pos(gtk::PositionType::Top);

    let gpt2_label = Label::new(Some("GPT-2"));
    let gpt2_buffer = TextBuffer::new(None);
    let gpt2_textview = TextView::new_with_buffer(&gpt2_buffer);
    let gpt2_scrolled_window = ScrolledWindow::new(None, None);
    gpt2_scrolled_window.set_policy(gtk::PolicyType::Automatic, 
gtk::PolicyType::Automatic);
    gpt2_scrolled_window.add(&gpt2_textview);
    notebook.append_page(&gpt2_scrolled_window, Some(&gpt2_label));

    let gpt3_label = Label::new(Some("GPT-3"));
    let gpt3_buffer = TextBuffer::new(None);
    let gpt3_textview = TextView::new_with_buffer(&gpt3_buffer);
    let gpt3_scrolled_window = ScrolledWindow::new(None, None);
    gpt3_scrolled_window.set_policy(gtk::PolicyType::Automatic, 
gtk::PolicyType::Automatic);
    gpt3_scrolled_window.add(&gpt3_textview);
    notebook.append_page(&gpt3_scrolled_window, Some(&gpt3_label));

    let gpt4_label = Label::new(Some("GPT-4"));
    let gpt4_buffer = TextBuffer::new(None);
    let gpt4_textview = TextView::new_with_buffer(&gpt4_buffer);
    let gpt4_scrolled_window = ScrolledWindow::new(None, None);
    gpt4_scrolled_window.set_policy(gtk::PolicyType::Automatic, 
gtk::PolicyType::Automatic);
    gpt4_scrolled_window.add(&gpt4_textview);
    notebook.append_page(&gpt4_scrolled_window, Some(&gpt4_label));

    let switch_gpt2_button = 
Button::new_from_icon_name(Some("document-edit-symbolic"), 
gtk::IconSize::SmallToolbar.into());
    let switch_gpt3_button = 
Button::new_from_icon_name(Some("document-edit-symbolic"), 
gtk::IconSize::SmallToolbar.into());
    let switch_gpt4_button = 
Button::new_from_icon_name(Some("document-edit-symbolic"), 
gtk::IconSize::SmallToolbar.into());

    let switch_gpt2_button_clone = switch_gpt2_button.clone();
    switch_gpt2_button_clone.connect_clicked(clone!(@weak notebook => move 
|_| {
        notebook.set_current_page(0);
    }));

    let switch_gpt3_button_clone = switch_gpt3_button.clone();
    switch_gpt3_button_clone.connect_clicked(clone!(@weak notebook => move 
|_| {
        notebook.set_current_page(1);
    }));

    let switch_gpt4_button_clone = switch_gpt4_button.clone();
    switch_gpt4_button_clone.connect_clicked(clone!(@weak notebook => move 
|_| {
        notebook.set_current_page(2);
    }));

    let paned = Paned::new(Orientation::Vertical);
    paned.pack1(&notebook, true, false);
    paned.pack2(&Label::new(Some("Chat Input")), true, false);

    header_bar.pack_start(&switch_gpt2_button);
    header_bar.pack_start(&switch_gpt3_button);
    header_bar.pack_start(&switch_gpt4_button);

    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    main_box.add(&paned);

    window.add(&main_box);
    window.show_all();

    gtk::main();
}

