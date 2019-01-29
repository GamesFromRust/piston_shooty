// TODO: clean out unused ids
// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {

        // The scrollable canvas.
        canvas,

        // The title and introduction widgets.
        title,
        introduction,

        // Shapes.
        shapes_canvas,
        rounded_rectangle,
        shapes_left_col,
        shapes_right_col,
        shapes_title,
        line,
        point_path,
        rectangle_fill,
        rectangle_outline,
        trapezoid,
        oval_fill,
        oval_outline,
        circle,

        // Image.
        image_title,
        rust_logo,

        // Button, XyPad, Toggle.
        button_title,
        button,
        xy_pad,
        toggle,
        ball,

        // NumberDialer, PlotPath
        dialer_title,
        number_dialer,
        plot_path,

        // Scrollbar
        canvas_scrollbar,

        // Main Menu World List
        world_list[],

        // FPS Counter
        fps_text,
        average_frame_time_text,
    }
}
