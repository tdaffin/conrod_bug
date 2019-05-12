
use conrod_core::{
    Positionable,
    Labelable,
    Sizeable,
    Borderable,
    Ui,
    widget_ids,
    widget::Widget,
};

widget_ids! {
    struct Ids {
        list,
    }
}

pub struct Gui {
    ui: Ui,
    ids: Ids,
}

impl Gui {
    pub fn new(width: f64, height: f64) -> Self {
        let mut ui = conrod_core::UiBuilder::new([width, height]).build();
        let ids = Ids::new(ui.widget_id_generator());

        let font = conrod_core::text::Font::from_bytes(include_bytes!("../assets/ProggyClean.ttf") as &[u8])
            .expect("failed to load font");

        ui.theme.font_id = Some(ui.fonts.insert(font));
        ui.theme.shape_color = conrod_core::color::CHARCOAL;
        ui.theme.label_color = conrod_core::color::WHITE;

        Self {
            ui,
            ids,
        }
    }

    pub fn handle_event(&mut self, e: conrod_core::event::Input) {
        self.ui.handle_event(e);
    }

    pub fn update(&mut self) {
        let ui = &mut self.ui.set_widgets();
        let ids = &self.ids;

        let (mut list_items_iter, scrollbar) = conrod_core::widget::List::flow_down(20)
            .top_left_with_margins_on(ui.window, ui.win_h/2.0-50.0, 30.0)
            .item_size(35.0)
            .h(ui.win_h/2.0-30.0)
            .w(ui.win_w/2.0-60.0)
            .scrollbar_on_top()
            .set(ids.list, ui);

        scrollbar.map(|s| s.set(ui));

        let mut i = 1;
        while let Some(item) = list_items_iter.next(ui) {
            let lbl = format!("Item {}", i);
            let button = conrod_core::widget::Button::new()
                .label(&lbl)
                .border(1.0)
                .border_color(conrod_core::color::WHITE)
                .label_font_size(15);
            item.set(button, ui);
            i += 1;
        }
    }

    pub fn draw_if_changed(&self) -> Option<conrod_core::render::Primitives> {
        self.ui.draw_if_changed()
    }
}
