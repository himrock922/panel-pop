extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::ttf::{Font, Sdl2TtfContext};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

macro_rules! rect(
  ($x:expr, $y:expr, $w:expr, $h:expr) => (
      Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
  )
);

type TextureManager<'l, T> = ResourceManager<'l, String, Texture<'l>, TextureCreator<T>>;
type FontManager<'l> = ResourceManager<'l, FontDetails, Font<'l, 'static>, Sdl2TtfContext>;

// Generic struct to cache any resource loaded by a ResourceLoader
pub struct ResourceManager<'l, K, R, L>
where
  K: Hash + Eq,
  L: 'l + ResourceLoader<'l, R>,
{
  loader: &'l L,
  cache: HashMap<K, Rc<R>>,
}

impl<'l, K, R, L> ResourceManager<'l, K, R, L>
where
  K: Hash + Eq,
  L: ResourceLoader<'l, R>,
{
  pub fn new(loader: &'l L) -> Self {
    ResourceManager {
      cache: HashMap::new(),
      loader: loader,
    }
  }

  // Generics magic to allow a HashMap to use String as a key
  // while allowing it to use &str for gets
  pub fn load<D>(&mut self, details: &D) -> Result<Rc<R>, String>
  where
    L: ResourceLoader<'l, R, Args = D>,
    D: Eq + Hash + ?Sized,
    K: Borrow<D> + for<'a> From<&'a D>,
  {
    self.cache.get(details).cloned().map_or_else(
      || {
        let resource = Rc::new(self.loader.load(details)?);
        self.cache.insert(details.into(), resource.clone());
        Ok(resource)
      },
      Ok,
    )
  }
}

// TextureCreator knows how to load Textures
impl<'l, T> ResourceLoader<'l, Texture<'l>> for TextureCreator<T> {
  type Args = str;
  fn load(&'l self, path: &str) -> Result<Texture, String> {
    println!("LOADED A TEXTURE");
    self.load_texture(path)
  }
}

// Font Context knows how to load Fonts
impl<'l> ResourceLoader<'l, Font<'l, 'static>> for Sdl2TtfContext {
  type Args = FontDetails;
  fn load(&'l self, details: &FontDetails) -> Result<Font<'l, 'static>, String> {
    println!("LOADED A FONT");
    self.load_font(&details.path, details.size)
  }
}

// Generic trait to Load any Resource Kind
pub trait ResourceLoader<'l, R> {
  type Args: ?Sized;
  fn load(&'l self, data: &Self::Args) -> Result<R, String>;
}

// Information needed to load a Font
#[derive(PartialEq, Eq, Hash)]
pub struct FontDetails {
  pub path: String,
  pub size: u16,
}

impl<'a> From<&'a FontDetails> for FontDetails {
  fn from(details: &'a FontDetails) -> FontDetails {
    FontDetails {
      path: details.path.clone(),
      size: details.size,
    }
  }
}

pub fn run() -> Result<(), String> {
  let image_path = String::from("assets/title.png");
  let font_path = String::from("assets/fonts/square_sans_serif_7.ttf");
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;
  let font_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
  let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
  let main_menus = vec!["1P Endless", "     VS AI", "     VS 2P", "   Options", "      EXIT"];

  let window = video_subsystem
    .window("panel-pop", 800, 600)
    .position_centered()
    .build()
    .map_err(|e| e.to_string())?;
  let mut canvas = window
    .into_canvas()
    .software()
    .build()
    .map_err(|e| e.to_string())?;

  let texture_creator = canvas.texture_creator();
  let mut texture_manager = TextureManager::new(&texture_creator);
  let mut font_manager = FontManager::new(&font_context);
  let details = FontDetails {
    path: font_path.clone(),
    size: 10,
  };

  // will load the image texture + font only once
  let texture = texture_manager.load(&image_path)?;
  let font = font_manager.load(&details)?;

  //draw all
  canvas.clear();
  canvas.copy(&texture, None, None)?;

  let main_menu_x = 600;
  let mut main_menu_y = 250;
  for menu in main_menus.iter() {
    // not recommended to create a texture from the font each iteration
    // but it is the simplest thing to do for this example
    let surface = font
      .render(menu)
      .blended(Color::RGB(0, 0, 0))
      .map_err(|e| e.to_string())?;
    let font_texture = texture_creator
      .create_texture_from_surface(&surface)
      .map_err(|e| e.to_string())?;
    canvas.copy(&font_texture, None, rect!(main_menu_x, main_menu_y, 125, 25))?;
    main_menu_y += 19;
  }
  canvas.present();

  'mainloop: loop {
    for event in sdl_context.event_pump()?.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Option::Some(Keycode::Escape),
          ..
        } => break 'mainloop,
        _ => {}
      }
    }
  }

  Ok(())
}

fn main() {
  let _ = run();
}

