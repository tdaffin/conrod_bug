
use conrod_core::{
    Positionable,
    Labelable,
    Sizeable,
    Borderable,
    Ui,
    widget_ids,
    widget::{Widget, Text, Toggle, List},
};

widget_ids! {
    struct Ids {
        title,
        text,
        toggle,
        list,
    }
}

pub struct GuiState {
    pub toggle: bool,
}

pub struct Gui {
    ids: Ids,
}

impl Gui {
    pub fn new(ui: &mut Ui) -> Self {
        let ids = Ids::new(ui.widget_id_generator());

        let font = conrod_core::text::Font::from_bytes(include_bytes!("../assets/ProggyClean.ttf") as &[u8])
            .expect("failed to load font");

        ui.theme.font_id = Some(ui.fonts.insert(font));
        ui.theme.shape_color = conrod_core::color::CHARCOAL;
        ui.theme.label_color = conrod_core::color::WHITE;

        Self {
            ids,
        }
    }

    pub fn update(&self, ui: &mut Ui, state: &mut GuiState, x_off: f64, width: f64) {
        let ui = &mut ui.set_widgets();

        Text::new("Graphics Scizzor Inconsistency")
            .mid_top_with_margin_on(ui.window, 24.0)
            .set(self.ids.title, ui);

        Text::new("Click toggle button below to switch between renderers.\nOnly one of the two will give correct results")
            .down_from(self.ids.title, 18.0)
            .align_middle_x_of(self.ids.title)
            .padded_w_of(ui.window, 10.0)
            .wrap_by_word()
            .set(self.ids.text, ui);

        let label = if state.toggle {
            "opengl_graphics"
        } else {
            "gfx_graphics"
        };
        for v in Toggle::new(state.toggle)
            .down_from(self.ids.text, 10.0)
            .align_middle_x_of(self.ids.title)
            .w_h(240.0, 36.0)
            .label(label)
            .set(self.ids.toggle, ui)
        {
            state.toggle = v;
        }

        let (mut list_items_iter, scrollbar) = List::flow_down(20)
            .top_left_with_margins_on(ui.window, ui.win_h/2.0-50.0, 30.0 + x_off)
            .item_size(35.0)
            .h(ui.win_h/2.0-30.0)
            .w(width)
            .scrollbar_on_top()
            .set(self.ids.list, ui);

        scrollbar.map(|s| s.set(ui));

        while let Some(item) = list_items_iter.next(ui) {
            let lbl = format!("Item {}", item.i);
            let button = conrod_core::widget::Button::new()
                .label(&lbl)
                .border(1.0)
                .border_color(conrod_core::color::WHITE)
                .label_font_size(15);
            item.set(button, ui);
        }
    }

}
