#[macro_use] extern crate lazy_static;
/*use gtk4::ApplicationWindow;
use glib::source::timeout_add_seconds_local;
use gtk4::prelude::*;
use gtk4::Application;
*/
use glib::{source::timeout_add_seconds_local, Continue};
use gtk4::{traits::GridExt};
use libadwaita::{
    gtk::Orientation,
    prelude::{ApplicationExt, ApplicationExtManual, BoxExt, WidgetExt},
    Application, ApplicationWindow, HeaderBar, WindowTitle,                             };

use chrono::prelude::*;
use std::collections::HashSet;

mod clock;

fn main() {

    gtk4::init().expect("Failed to initialize GTK.");

    let app: Application = Application::builder()
        .application_id("rust-design-clock.test")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {

    let content = gtk4::Box::new(Orientation::Vertical, 0);

    let header = HeaderBar::builder()
    .title_widget(&WindowTitle::new("Rust Design Clock", ""))
    .build();

    content.append(&header);

    let grid: gtk4::Grid = build_grid();
    let idxs = get_idxs();
    
    fill_grid(&grid, &idxs);

    content.append(&grid);

    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .title("Clock")
        .content(&content)
        .build();

    window.show();

    timeout_add_seconds_local(5, move || {
        let now = Local::now();
        if now.second() < 9 {
            fill_grid(&grid, &get_idxs());
            //println!("Updating grid");
        }
        Continue(true)
    });

}

fn get_idxs() -> HashSet<(u32, u32)> {
    let now = Local::now();
    let minutes = now.minute();
    let minute_state = clock::MinuteStates::from_minutes(minutes);
    let minute_state_idxs = minute_state.state_to_idx();
    let hour = now.hour12().1;
    let hour_state = clock::HourStates::from_hours(hour);
    let hour_state_idxs = hour_state.hour_idxs();
    let hour_minutes_idxs: HashSet<(u32, u32)> = 
        hour_state_idxs.union(&minute_state_idxs).cloned().collect();
    let all_idxs: HashSet<(u32, u32)> = hour_minutes_idxs
        .union(&clock::TIME_IS_IDXS).cloned().collect();

    /*for row in 0.. clock::CLOCK_TEXT_DIM.0 {
        for col in 0.. clock::CLOCK_TEXT_DIM.1 {
            if all_idxs.contains(&(row, col)) {
                print!("{} ", clock::CLOCK_TEXT[row as usize][col as usize].to_string().green());
            } else {
                print!("{} ", clock::CLOCK_TEXT[row as usize][col as usize].to_string());
            }
        }
        println!("");
    }
    */
    all_idxs
    
}

fn build_grid() -> gtk4::Grid {
    let grid = gtk4::Grid::new();
    grid.set_column_homogeneous(true);
    grid.set_row_homogeneous(true);
    grid.set_column_spacing(10);
    grid.set_row_spacing(10);
    grid.set_margin_top(10);
    grid.set_margin_bottom(10);
    grid.set_hexpand(true);
    grid.set_vexpand(true);
    grid.set_halign(gtk4::Align::Center);
    grid.set_valign(gtk4::Align::Center);
    grid
}

fn fill_grid(grid: &gtk4::Grid, idxs: &HashSet<(u32, u32)>) {
    for row in 0.. clock::CLOCK_TEXT_DIM.0 {
        for col in 0.. clock::CLOCK_TEXT_DIM.1 {

            // delete the old widget
            match grid.child_at(col as i32, row as i32) {
                Some(widget) => {
                    //println!("Deleting widget at row: {}, col: {}", row, col);
                    grid.remove(&widget);
                }
                None => {
                    //println!("No widget at row: {}, col: {}", row, col);
                }
            }

            // add the new widget
            let c = clock::CLOCK_TEXT[row as usize][col as usize];
            let label = gtk4::Label::new(Some(&c.to_string()));

            if idxs.contains(&(row, col)) {
                label.set_markup("");
                label.set_markup(&format!("<span foreground=\"{}\"><b>{}</b></span>", "orange".to_string(), c));
            } else {
                label.set_markup("");
                label.set_markup(&format!("<span foreground=\"{}\">{}</span>", "gray".to_string(), c));
            }

            label.set_margin_top(5);
            label.set_margin_bottom(10);
            label.set_width_chars(3);

            grid.attach(&label, col as i32, row as i32, 1, 1);
        }
    }
}

/*
fn update_grid(grid: *mut gtk4::Grid) -> gtk4::prelude::Continue {
    unsafe {
        let idxs = get_idxs();
        fill_grid(&*grid, &idxs);
    }
    gtk4::prelude::Continue(true)
}
*/