pub mod cassette;
pub mod cpu;
pub mod ppu;
pub mod ram;
pub mod screen;

use self::cassette::Cassette;
use self::ppu::Ppu;
use self::ppu::PpuRunResult;
use self::ram::Ram;
use self::cpu::{Cpu, Bus as CpuBus};
use self::screen::Screen;

use sdl2::event::Event;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 240;

pub struct Nes {
    cpu: Cpu,
    ppu: Ppu,
    cassette: Cassette,
    ram: Ram,
}

impl Nes {
    pub fn new(path: &str) -> Nes {
        let cassette = Cassette::new(path).unwrap();

        let mut nes = Nes {
            cpu: Cpu::new(),
            ppu: Ppu::new(cassette.character_rom.clone()),
            cassette: cassette,
            ram: Ram::new(vec![0; 0x0800]),
        };

        {
            let mut bus = CpuBus::new(&nes.cassette, &mut nes.ppu, &mut nes.ram);
            nes.cpu.reset(&mut bus);
        }

        nes
    }

    pub fn run(&mut self) {
        let mut screen = Screen::new(WIDTH, HEIGHT);

        'main: loop {
            let cycle = {
                let mut bus = CpuBus::new(&self.cassette, &mut self.ppu, &mut self.ram);
                self.cpu.run(&mut bus)
            };

            let ppu_run_result = self.ppu.run(cycle * 3);
            match ppu_run_result {
                PpuRunResult::FinishedBuildAllBackgroundLine => {
                    let background = &self.ppu.background;
                    screen.render_background(&background);
                },
                _ => {},
            };

            for event in screen.events.poll_iter() {
                match event {
                    Event::Quit {..} => break 'main,
                    _ => {}
                }
            }
        }
    }
}