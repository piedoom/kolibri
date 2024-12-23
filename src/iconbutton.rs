use crate::smartstate::{Container, Smartstate};
use crate::ui::{GuiResult, Interaction, Response, Ui, Widget};
use core::cmp::max;
use core::marker::PhantomData;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::image::Image;
use embedded_graphics::pixelcolor::PixelColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use embedded_iconoir::prelude::{IconoirIcon, IconoirNewIcon};

pub struct IconButton<'a, ICON: IconoirIcon> {
    icon: PhantomData<ICON>,
    smartstate: Container<'a, Smartstate>,
}

impl<'a, ICON: IconoirIcon> IconButton<'a, ICON> {
    pub fn new(_icon: ICON) -> Self {
        Self {
            icon: PhantomData,
            smartstate: Container::empty(),
        }
    }

    pub fn new_from_type() -> Self {
        Self {
            icon: PhantomData,
            smartstate: Container::empty(),
        }
    }

    pub fn smartstate(mut self, smartstate: &'a mut Smartstate) -> Self {
        self.smartstate.set(smartstate);
        self
    }
}

impl<ICON: IconoirIcon, INTER> Widget<INTER> for IconButton<'_, ICON>
where
    INTER: Interaction,
{
    fn draw<DRAW: DrawTarget<Color = COL>, COL: PixelColor>(
        &mut self,
        ui: &mut Ui<DRAW, COL, INTER>,
    ) -> GuiResult<Response<INTER>> {
        // get size
        let icon = ICON::new(ui.style().icon_color);

        let padding = ui.style().spacing.button_padding;
        let border = ui.style().border_width;

        let height = max(
            max(ui.style().default_widget_height, ui.get_row_height()),
            icon.bounding_box().size.height + 2 * padding.height + 2 * border,
        );

        let width = height;

        let size = Size::new(width, height);

        /*
        let icon = match size.width - 2 * padding.width {
            0..=17 => 12,
            18..=24 => 18,
            24..=32 => 24,
            _ => 32,
        };
         */

        // allocate space
        let iresponse = ui.allocate_space(Size::new(size.width, max(size.height, height)))?;

        // translate icon
        let size = icon.bounding_box();

        // center icon
        let center_offset = iresponse.area.top_left
            + Point::new(
                ((iresponse.area.size.width - size.size.width) / 2) as i32,
                ((iresponse.area.size.height - size.size.height) / 2) as i32,
            );

        let icon_img = Image::new(&icon, center_offset);

        // check for click
        let click = iresponse.interaction.is_clicked();
        let down = iresponse.interaction.is_clicked() || iresponse.interaction.is_dragged();

        // styles and smartstate
        let prevstate = self.smartstate.clone_inner();

        let rect_style = if iresponse.interaction.is_none() {
            self.smartstate.modify(|st| *st = Smartstate::state(1));

            PrimitiveStyleBuilder::new()
                .stroke_color(ui.style().border_color)
                .stroke_width(ui.style().border_width)
                .fill_color(ui.style().item_background_color)
                .build()
        } else if iresponse.interaction.is_hovered() {
            self.smartstate.modify(|st| *st = Smartstate::state(2));
            PrimitiveStyleBuilder::new()
                .stroke_color(ui.style().highlight_border_color)
                .stroke_width(ui.style().highlight_border_width)
                .fill_color(ui.style().highlight_item_background_color)
                .build()
        } else {
            self.smartstate.modify(|st| *st = Smartstate::state(3));

            PrimitiveStyleBuilder::new()
                .stroke_color(ui.style().highlight_border_color)
                .stroke_width(ui.style().highlight_border_width)
                .fill_color(ui.style().primary_color)
                .build()
        };

        if !self.smartstate.eq_option(&prevstate) {
            ui.start_drawing(&iresponse.area);

            ui.draw(
                &Rectangle::new(iresponse.area.top_left, iresponse.area.size)
                    .into_styled(rect_style),
            )
            .ok();
            ui.draw(&icon_img).ok();

            ui.finalize()?;
        }

        Ok(Response::new(iresponse).set_clicked(click).set_down(down))
    }
}
