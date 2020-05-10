use tui::{
    backend::TestBackend,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Fragment,
    widgets::{Block, Borders},
    Terminal,
};

#[test]
fn widgets_block_renders() {
    let backend = TestBackend::new(10, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|mut f| {
            let block = Block::default()
                .title(Fragment::styled(
                    "Title",
                    Style::default().fg(Color::LightBlue),
                ))
                .borders(Borders::ALL);
            f.render_widget(
                block,
                Rect {
                    x: 0,
                    y: 0,
                    width: 8,
                    height: 8,
                },
            );
        })
        .unwrap();
    let mut expected = Buffer::with_lines(vec![
        "┌Title─┐  ",
        "│      │  ",
        "│      │  ",
        "│      │  ",
        "│      │  ",
        "│      │  ",
        "│      │  ",
        "└──────┘  ",
        "          ",
        "          ",
    ]);
    for x in 1..=5 {
        expected.get_mut(x, 0).set_fg(Color::LightBlue);
    }
    terminal.backend().assert_buffer(&expected);
}
