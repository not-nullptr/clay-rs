use clay_layout::{
    elements::{rectangle::Rectangle, CornerRadius},
    fixed,
    layout::Layout,
    Clay,
};

fn main() {
    // Create the clay instance
    let clay = Clay::new((800., 600.).into());

    // Begin the layout
    clay.begin();

    // Adds a red rectangle with a corner radius of 5.
    // The Layout makes the rectangle have a width and height of 50.
    clay.with(
        [
            Layout::new().width(fixed!(50.)).height(fixed!(50.)).end(),
            Rectangle::new()
                .color((0xFF, 0x00, 0x00).into())
                .corner_radius(CornerRadius::All(5.))
                .end("Red Rectangle".into()),
        ],
        |_| {},
    );

    // Return the list of render commands of your layout
    let render_commands = clay.end();

    for command in render_commands {
        println!("Id of the element: {}", command.id); // Note: Ids are in fact numbers generated by Clay
        println!("Bounding box: {:?}", command.bounding_box);
        println!("Type and config: {:?}", command.config);
    }
}
