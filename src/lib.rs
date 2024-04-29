#![allow(dead_code)]
use anyhow::Result;
use crossterm::style::Color::{self, DarkGreen, DarkGrey, Green, Grey, White};
use crossterm::{
    cursor::MoveTo,
    style::{Print, SetForegroundColor},
    ExecutableCommand,
};
use rand::random;
use std::io::{stdin, stdout, Read};

const MAX_SPEED: usize = 4;

lazy_static::lazy_static! {
    static ref COLORS: [Color; 5] = [DarkGrey, DarkGreen, Green, Grey, White];
}

type Corpus = Vec<char>;

fn pick_speed() -> usize {
    random::<usize>() % MAX_SPEED
}

fn pick_color() -> Color {
    COLORS[random::<usize>() % COLORS.len()]
}

fn pick_char(corpus: &Corpus) -> char {
    if rand::random::<usize>() % 3 == 0 {
        ' '
    } else {
        corpus[random::<usize>() % corpus.len()]
    }
}

fn pick_volatility() -> f32 {
    rand::random::<f32>() % 1.0
}

fn get_corpus() -> Result<Corpus> {
    let mut v = Vec::new();
    stdin().read_to_end(&mut v)?;
    Ok(v.iter().map(|x| (*x).into()).collect::<Corpus>())
}

#[derive(Debug, Clone)]
struct Position(usize, usize);

#[derive(Debug, Clone)]
struct Cell {
    c: char,
    color: Color,
    speed: usize,
    volatility: f32,
    position: Position,
    max_height: usize,
}

impl Cell {
    pub fn new(c: char, window: &Window) -> Self {
        Self {
            c,
            color: pick_color(),
            volatility: pick_volatility(),
            speed: pick_speed(),
            position: Position(rand::random::<usize>() % window.dimensions.0, 0),
            max_height: window.dimensions.1,
        }
    }

    pub fn iterate(&mut self, corpus: &Corpus) -> bool {
        if rand::random::<f32>() % 1.0 > self.volatility {
            if self.position.1 + self.speed >= self.max_height - 1 {
                return false;
            } else {
                self.position.1 += self.speed;
            }

            if rand::random::<usize>() % 10 == 0 {
                self.c = pick_char(corpus);
            }

            self.speed = pick_speed();
            self.color = pick_color();
            self.volatility = pick_volatility();
        }

        true
    }
}

#[derive(Debug, Clone)]
pub struct Window {
    cells: Vec<Cell>,
    dimensions: Position,
    corpus: Corpus,
}

impl Default for Window {
    fn default() -> Self {
        let mut w = Window::from_terminal().expect("Could not perform I/O");
        w.cells = w.generate_cells();
        w
    }
}

impl Window {
    pub fn from_terminal() -> Result<Self> {
        let (width, height) = crossterm::terminal::size()?;

        Ok(Self {
            cells: Default::default(),
            corpus: get_corpus()?,
            dimensions: Position(width.into(), height.into()),
        })
    }

    fn generate_cells(&mut self) -> Vec<Cell> {
        let mut v = Vec::new();
        for _ in 0..(rand::random::<usize>() % self.dimensions.0 / 10) {
            v.push(Cell::new(pick_char(&self.corpus), &self))
        }

        v
    }

    pub fn draw_loop(&mut self) -> Result<()> {
        let mut new_cells = Vec::new();

        for cell in &self.cells {
            let mut cell = cell.clone();

            stdout()
                .execute(MoveTo(
                    cell.position.0.try_into()?,
                    cell.position.1.try_into()?,
                ))?
                .execute(SetForegroundColor(cell.color))?
                .execute(Print(cell.c))?;

            if cell.iterate(&self.corpus) {
                new_cells.push(cell);
            }
        }

        std::thread::sleep(std::time::Duration::new(
            0,
            10000000 + (rand::random::<u32>() % 100000),
        ));

        new_cells.append(&mut self.generate_cells());

        self.cells = new_cells;

        Ok(())
    }
}
