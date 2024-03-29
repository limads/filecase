/*Copyright (c) 2022 Diego da Silva Lima. All rights reserved.

This work is licensed under the terms of the MIT license.  
For a copy, see <https://opensource.org/licenses/MIT>.*/

use gtk4::*;
use gtk4::prelude::*;

#[derive(Debug, Clone)]
pub struct OpenDialog {
    pub dialog : FileChooserDialog
}

impl OpenDialog {

    pub fn build(patterns : &[&str]) -> Self {
        let dialog = FileChooserDialog::new(
            Some("Open file"),
            None::<&Window>,
            FileChooserAction::Open,
            &[("Cancel", ResponseType::None), ("Open", ResponseType::Accept)]
        );
        dialog.connect_response(move |dialog, resp| {
            match resp {
                ResponseType::Reject | ResponseType::Accept | ResponseType::Yes | ResponseType::No |
                ResponseType::None | ResponseType::DeleteEvent => {
                    dialog.close();
                },
                _ => { }
            }
        });
        configure_dialog(&dialog);
        let filter = FileFilter::new();
        for pattern in patterns {
            filter.add_pattern(pattern);
        }
        dialog.set_filter(&filter);
        Self { dialog }
    }

}

pub fn configure_dialog(dialog : &impl GtkWindowExt) {
    dialog.set_modal(true);
    dialog.set_deletable(true);
    dialog.set_destroy_with_parent(true);
    dialog.set_hide_on_close(true);
}

#[derive(Debug, Clone)]
pub struct SaveDialog {
    pub dialog : FileChooserDialog
}

impl SaveDialog {

    pub fn build(patterns : &[&str]) -> Self {
        let dialog = FileChooserDialog::new(
            Some("Save file"),
            None::<&Window>,
            FileChooserAction::Save,
            &[("Cancel", ResponseType::None), ("Save", ResponseType::Accept)]
        );
        dialog.connect_response(move |dialog, resp| {
            match resp {
                ResponseType::Close | ResponseType::Reject | ResponseType::Accept | ResponseType::Yes |
                ResponseType::No | ResponseType::None | ResponseType::DeleteEvent => {
                    dialog.close();
                },
                _ => { }
            }
        });
        configure_dialog(&dialog);
        let filter = FileFilter::new();
        for pattern in patterns {
            filter.add_pattern(pattern);
        }
        dialog.set_filter(&filter);
        Self { dialog }
    }

}


