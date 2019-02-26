// TODO: clean out unused ids
// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {

        // The scrollable canvas.
        canvas,

        // Image.
        title,
        rust_logo,
        guns_hud[],
        shots_taken_hud[],
        bullets_remaining_hud[],

        // Main Menu World List
        world_list[],

        // FPS Counter
        fps_text,
        average_frame_time_text,
    }
}
