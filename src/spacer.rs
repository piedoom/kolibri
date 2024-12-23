use crate::ui::{GuiResult, Interaction, Response, Ui, Widget};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Size;
use embedded_graphics::pixelcolor::PixelColor;

pub struct Spacer {
    space: Size,
}

impl Spacer {
    pub fn new(space: Size) -> Spacer {
        Spacer { space }
    }
}

impl<INTER> Widget<INTER> for Spacer
where
    INTER: Interaction,
{
    fn draw<DRAW: DrawTarget<Color = COL>, COL: PixelColor>(
        &mut self,
        ui: &mut Ui<DRAW, COL, INTER>,
    ) -> GuiResult<Response<INTER>> {
        // allocate space
        let space = ui.allocate_space(self.space)?;

        Ok(Response::new(space))
    }
}
