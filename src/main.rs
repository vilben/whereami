mod ip_location;

use std::future::Future;
use std::io::{self, stdout};
use tokio::runtime::Runtime;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    style::Color,
    widgets::{canvas::*},
};
use ratatui::widgets::{Block, Borders};
use reqwest::Url;
use crate::ip_location::IpLocation;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;


    let mut zoom_value = 0.0;
    let max_zoom = 125.0;

    let rt = Runtime::new().unwrap();
    let location = rt.block_on(get_location());

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(get_ui(location.clone(), zoom_value))?;
        if zoom_value <= max_zoom {
            zoom_value = zoom_value + 1.0;
        }

        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn get_ui(ip_location: IpLocation, zoom: f64) -> impl FnOnce(&mut Frame) {
    let ui = move |frame: &mut Frame| {
        let lat = ip_location.lat;
        let lon = ip_location.lon;

        frame.render_widget(get_world_map(lon, lat, zoom, String::from("Hello World")), frame.size());
    };
    ui
}

async fn get_location() -> IpLocation {
    let url = Url::parse("http://ip-api.com/json/").unwrap();
    reqwest::get(url).await.unwrap().json::<IpLocation>().await.unwrap()
}

fn get_world_map(lon: f64, lat: f64, zoom: f64, title: String) -> Canvas<'static, impl Fn(&mut Context)> {
    let margin = 2.0;

    fn get_bounds(lon: f64, lat: f64, zoom: f64) -> (f64, f64, f64, f64) {
        let x_from = (-(180.0 - zoom)) + lon;
        let x_to = (180.0 - zoom) + lon;


        let y_from = (-(90.0 - (zoom) / 2.0)) + lat;
        let y_to = (90.0 - (zoom) / 2.0) + lat;

        (x_from, x_to, y_from, y_to)
    }
    let bounds = get_bounds(lon, lat, zoom);

    Canvas::default()
        .marker(Marker::Braille)
        .block(Block::default().title(title).borders(Borders::ALL))
        .x_bounds([bounds.0, bounds.1])
        .y_bounds([bounds.2, bounds.3])
        .paint(move |ctx| {
            ctx.draw(&Map {
                resolution: MapResolution::High,
                color: Color::White,
            });
            ctx.layer();
            ctx.draw(&Rectangle {
                x: lon - margin,
                y: lat - margin,
                width: margin * 2.0,
                height: margin * 2.0,
                color: Color::Red,
            });
        })
}
