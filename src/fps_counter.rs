use conrod_core;
use conrod_core::Colorable;
use conrod_core::Positionable;
use conrod_core::Widget;
use time;
use crate::ui_widget_ids;

const NSEC_PER_SEC: u64 = 1_000_000_000;

// TODO: Read a book on how to do a fps counter.
pub struct FpsCounter {
    last_batch_start_time: u64,
    num_frames_in_batch: u64,
    average_frame_time: u64,
    fps: u64,
}

impl Default for FpsCounter {
    fn default() -> Self {
        FpsCounter {
            last_batch_start_time: time::precise_time_ns(),
            num_frames_in_batch: 0,
            average_frame_time: 1,
            fps: 0,
        }
    }
}

impl FpsCounter {
    pub fn calculate_fps(&mut self) {
        let curr_frame_time: u64 = time::precise_time_ns();

        self.num_frames_in_batch += 1;

        if curr_frame_time >= self.last_batch_start_time + NSEC_PER_SEC {
            self.average_frame_time = (curr_frame_time - self.last_batch_start_time) /
                                      self.num_frames_in_batch;
            self.last_batch_start_time = curr_frame_time;
            self.num_frames_in_batch = 0;
        }

        self.fps = NSEC_PER_SEC / self.average_frame_time;
    }

    pub fn update_ui(&self, ui_cell: &mut conrod_core::UiCell, ids: &ui_widget_ids::Ids) {
        let fps_text = "FPS: ".to_string() + &self.fps.to_string();
        let average_frame_time_text = "Average Frame Time: ".to_string() + &(self.average_frame_time as f64 / NSEC_PER_SEC as f64).to_string();

        conrod_core::widget::Text::new(&fps_text).font_size(14).color(conrod_core::color::WHITE).top_left_of(ids.canvas).set(ids.fps_text, ui_cell);
        conrod_core::widget::Text::new(&average_frame_time_text).font_size(14).color(conrod_core::color::WHITE).down_from(ids.fps_text, 2.0).set(ids.average_frame_time_text, ui_cell);
    }
}
